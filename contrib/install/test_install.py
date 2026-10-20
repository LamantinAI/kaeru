"""Exercise the installer with isolated downloads and service commands."""
import fcntl
import os
from pathlib import Path
import plistlib
import pty
import select
import signal
import subprocess
import tarfile
import tempfile
import termios
import time
import unittest

INSTALLER = Path(__file__).with_name("install.sh").resolve()


class InstallerAuthenticationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.commands = self.root / "commands"
        self.commands.mkdir()
        self.log = self.root / "commands.log"
        fake = self.root / "kaeru-mcp"
        fake.write_text('''#!/usr/bin/env bash
printf 'kaeru %s\\n' "$*" >> "$INSTALL_TEST_LOG"
if [[ "$1" == --auth-status ]]; then printf '%s\\n' "$INSTALL_TEST_MODE"; fi
''')
        fake.chmod(0o755)
        self.archive = self.root / "release.tar.gz"
        with tarfile.open(self.archive, "w:gz") as archive:
            archive.add(fake, arcname="kaeru-mcp")
        self.command("curl", '''
while (( $# )); do
    if [[ "$1" == -o ]]; then cp "$INSTALL_TEST_ARCHIVE" "$2"; exit; fi
    shift
done
printf '%s\\n' '{"tag_name":"v0.8.0"}'
''')
        self.command("uname", '[[ "$1" == -s ]] && echo "$INSTALL_TEST_OS" || echo "$INSTALL_TEST_ARCH"\n')
        for name in ("systemctl", "launchctl"):
            self.command(name, f'printf "{name} %s\\n" "$*" >> "$INSTALL_TEST_LOG"\n')
        self.command("sleep", "exit 0\n")
        home = self.root / "home"
        home.mkdir()
        self.env = {key: value for key, value in os.environ.items() if not key.startswith("KAERU_")}
        self.env.update(HOME=str(home), PATH=str(self.commands)+os.pathsep+os.environ["PATH"],
                        KAERU_INSTALL_DIR=str(home/"bin"), KAERU_VERSION="v0.8.0",
                        INSTALL_TEST_LOG=str(self.log), INSTALL_TEST_ARCHIVE=str(self.archive),
                        INSTALL_TEST_OS="Linux", INSTALL_TEST_ARCH="x86_64", INSTALL_TEST_MODE="disabled")

    def command(self, name, body):
        path = self.commands / name
        path.write_text("#!/usr/bin/env bash\nset -e\n"+body)
        path.chmod(0o755)

    def run_installer(self, **variables):
        result = subprocess.run(["bash", str(INSTALLER)], env={**self.env, **variables},
                                stdin=subprocess.DEVNULL, capture_output=True, text=True,
                                start_new_session=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stdout+result.stderr)
        return result.stdout+result.stderr

    def test_new_unattended_install_defaults_to_open_and_warns_about_hosting(self):
        output = self.run_installer()
        commands = self.log.read_text()
        self.assertIn("--auth disabled", commands)
        self.assertIn("hosted or publicly reachable server", output)
        self.assertLess(commands.index("systemctl --user stop"), commands.index("kaeru --auth disabled"))
        self.assertNotIn("--password-stdin", commands)
        unit = (Path(self.env["HOME"])/".config/systemd/user/kaeru-mcp.service").read_text()
        self.assertIn("KAERU_MCP_WEB_CONFIG_FILE=", unit)

    def test_unattended_update_preserves_protected_configuration(self):
        output = self.run_installer(INSTALL_TEST_MODE="enabled")
        self.assertIn("--auth enabled", self.log.read_text())
        self.assertNotIn("--auth disabled", self.log.read_text())
        self.assertIn("MCP_TOKEN", output)
        self.assertNotIn("Authentication is OFF", output)

    def test_explicit_disable_overrides_previous_choice(self):
        self.run_installer(KAERU_SETUP_AUTH="no", INSTALL_TEST_MODE="enabled")
        self.assertIn("--auth disabled", self.log.read_text())

    def test_explicit_enable_supports_a_password_file_without_printing_it(self):
        password = self.root/"password"
        password.write_text("do not print this password\n")
        output = self.run_installer(KAERU_SETUP_AUTH="yes", KAERU_WEB_PASSWORD_FILE=str(password))
        self.assertIn("--auth enabled", self.log.read_text())
        self.assertIn("--password-stdin", self.log.read_text())
        self.assertNotIn("do not print this password", output+self.log.read_text())

    def test_launchd_keeps_the_selected_config_path(self):
        path = str(self.root/"config with spaces & symbols"/"web.toml")
        self.run_installer(INSTALL_TEST_OS="Darwin", INSTALL_TEST_ARCH="arm64", KAERU_MCP_WEB_CONFIG_FILE=path)
        file = Path(self.env["HOME"])/"Library/LaunchAgents/ai.lamantin.kaeru-mcp.plist"
        config = plistlib.loads(file.read_bytes())
        self.assertEqual(config["EnvironmentVariables"]["KAERU_MCP_WEB_CONFIG_FILE"], path)

    def test_old_release_does_not_receive_an_unknown_setup_flag(self):
        self.run_installer(KAERU_VERSION="v0.7.5")
        commands = self.log.read_text()
        self.assertNotIn("kaeru --auth", commands)

    def interactive_install(self, answer):
        master, slave = pty.openpty()
        def controlling_terminal():
            os.setsid()
            fcntl.ioctl(slave, termios.TIOCSCTTY, 0)
        process = subprocess.Popen(["bash", "-c", 'cat "$INSTALL_TEST_SCRIPT" | bash'],
            env={**self.env, "INSTALL_TEST_SCRIPT":str(INSTALLER)}, stdin=slave, stdout=slave,
            stderr=slave, preexec_fn=controlling_terminal, close_fds=True)
        os.close(slave)
        output = b""
        answered = False
        try:
            deadline = time.monotonic()+10
            while time.monotonic() < deadline:
                if select.select([master], [], [], .1)[0]:
                    try:
                        chunk = os.read(master, 65536)
                    except OSError:
                        break
                    if not chunk:
                        break
                    output += chunk
                    if b"Enable authentication?" in output and not answered:
                        os.write(master, answer)
                        answered = True
                if process.poll() is not None:
                    break
            self.assertTrue(answered, output.decode(errors="replace"))
            self.assertEqual(process.wait(timeout=2), 0, output.decode(errors="replace"))
            return output.decode(errors="replace"), self.log.read_text()
        finally:
            if process.poll() is None:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
            os.close(master)

    def test_curl_pipe_install_reads_the_choice_from_the_terminal(self):
        _, commands = self.interactive_install(b"y\n")
        self.assertIn("--auth enabled", commands)

    def test_enter_at_the_new_install_prompt_selects_no_authentication(self):
        output, commands = self.interactive_install(b"\n")
        self.assertIn("default N", output)
        self.assertIn("--auth disabled", commands)
        self.assertIn("hosted or publicly reachable server", output)


if __name__ == "__main__":
    unittest.main()
