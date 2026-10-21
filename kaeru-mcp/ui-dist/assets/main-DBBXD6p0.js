import{m as ml,b as lo,r as td}from"./api-lY19cru_.js";/**
 * @license
 * Copyright 2010-2026 Three.js Authors
 * SPDX-License-Identifier: MIT
 */const gl="184",hs={ROTATE:0,DOLLY:1,PAN:2},rs={ROTATE:0,PAN:1,DOLLY_PAN:2,DOLLY_ROTATE:3},ed=0,ec=1,nd=2,zr=1,id=2,Ps=3,ci=0,Ke=1,Bn=2,Hn=0,us=1,ta=2,nc=3,ic=4,sd=5,Si=100,rd=101,ad=102,od=103,ld=104,cd=200,hd=201,ud=202,dd=203,co=204,ho=205,fd=206,pd=207,md=208,gd=209,_d=210,xd=211,vd=212,Md=213,Sd=214,uo=0,fo=1,po=2,ps=3,mo=4,go=5,_o=6,xo=7,xh=0,yd=1,Ed=2,Cn=0,vh=1,Mh=2,Sh=3,yh=4,Eh=5,bh=6,Th=7,Ah=300,Pi=301,ms=302,Ta=303,Aa=304,ga=306,vo=1e3,kn=1001,Mo=1002,Le=1003,bd=1004,er=1005,Be=1006,wa=1007,Ti=1008,an=1009,wh=1010,Rh=1011,ks=1012,_l=1013,Ln=1014,An=1015,Vn=1016,xl=1017,vl=1018,Hs=1020,Ch=35902,Ph=35899,Lh=1021,Dh=1022,pn=1023,Wn=1026,Ai=1027,Ih=1028,Ml=1029,Li=1030,Sl=1031,yl=1033,kr=33776,Hr=33777,Gr=33778,Vr=33779,So=35840,yo=35841,Eo=35842,bo=35843,To=36196,Ao=37492,wo=37496,Ro=37488,Co=37489,ea=37490,Po=37491,Lo=37808,Do=37809,Io=37810,Uo=37811,No=37812,Fo=37813,Oo=37814,Bo=37815,zo=37816,ko=37817,Ho=37818,Go=37819,Vo=37820,Wo=37821,Xo=36492,$o=36494,qo=36495,Yo=36283,jo=36284,na=36285,Ko=36286,Td=3200,sc=0,Ad=1,si="",sn="srgb",ia="srgb-linear",sa="linear",ie="srgb",ki=7680,rc=519,wd=512,Rd=513,Cd=514,El=515,Pd=516,Ld=517,bl=518,Dd=519,Zo=35044,ac="300 es",wn=2e3,ra=2001;function Id(n){for(let t=n.length-1;t>=0;--t)if(n[t]>=65535)return!0;return!1}function aa(n){return document.createElementNS("http://www.w3.org/1999/xhtml",n)}function Ud(){const n=aa("canvas");return n.style.display="block",n}const oc={};function oa(...n){const t="THREE."+n.shift();console.log(t,...n)}function Uh(n){const t=n[0];if(typeof t=="string"&&t.startsWith("TSL:")){const e=n[1];e&&e.isStackTrace?n[0]+=" "+e.getLocation():n[1]='Stack trace not available. Enable "THREE.Node.captureStackTrace" to capture stack traces.'}return n}function It(...n){n=Uh(n);const t="THREE."+n.shift();{const e=n[0];e&&e.isStackTrace?console.warn(e.getError(t)):console.warn(t,...n)}}function Kt(...n){n=Uh(n);const t="THREE."+n.shift();{const e=n[0];e&&e.isStackTrace?console.error(e.getError(t)):console.error(t,...n)}}function Jo(...n){const t=n.join(" ");t in oc||(oc[t]=!0,It(...n))}function Nd(n,t,e){return new Promise(function(i,s){function r(){switch(n.clientWaitSync(t,n.SYNC_FLUSH_COMMANDS_BIT,0)){case n.WAIT_FAILED:s();break;case n.TIMEOUT_EXPIRED:setTimeout(r,e);break;default:i()}}setTimeout(r,e)})}const Fd={[uo]:fo,[po]:_o,[mo]:xo,[ps]:go,[fo]:uo,[_o]:po,[xo]:mo,[go]:ps};class fi{addEventListener(t,e){this._listeners===void 0&&(this._listeners={});const i=this._listeners;i[t]===void 0&&(i[t]=[]),i[t].indexOf(e)===-1&&i[t].push(e)}hasEventListener(t,e){const i=this._listeners;return i===void 0?!1:i[t]!==void 0&&i[t].indexOf(e)!==-1}removeEventListener(t,e){const i=this._listeners;if(i===void 0)return;const s=i[t];if(s!==void 0){const r=s.indexOf(e);r!==-1&&s.splice(r,1)}}dispatchEvent(t){const e=this._listeners;if(e===void 0)return;const i=e[t.type];if(i!==void 0){t.target=this;const s=i.slice(0);for(let r=0,a=s.length;r<a;r++)s[r].call(this,t);t.target=null}}}const Fe=["00","01","02","03","04","05","06","07","08","09","0a","0b","0c","0d","0e","0f","10","11","12","13","14","15","16","17","18","19","1a","1b","1c","1d","1e","1f","20","21","22","23","24","25","26","27","28","29","2a","2b","2c","2d","2e","2f","30","31","32","33","34","35","36","37","38","39","3a","3b","3c","3d","3e","3f","40","41","42","43","44","45","46","47","48","49","4a","4b","4c","4d","4e","4f","50","51","52","53","54","55","56","57","58","59","5a","5b","5c","5d","5e","5f","60","61","62","63","64","65","66","67","68","69","6a","6b","6c","6d","6e","6f","70","71","72","73","74","75","76","77","78","79","7a","7b","7c","7d","7e","7f","80","81","82","83","84","85","86","87","88","89","8a","8b","8c","8d","8e","8f","90","91","92","93","94","95","96","97","98","99","9a","9b","9c","9d","9e","9f","a0","a1","a2","a3","a4","a5","a6","a7","a8","a9","aa","ab","ac","ad","ae","af","b0","b1","b2","b3","b4","b5","b6","b7","b8","b9","ba","bb","bc","bd","be","bf","c0","c1","c2","c3","c4","c5","c6","c7","c8","c9","ca","cb","cc","cd","ce","cf","d0","d1","d2","d3","d4","d5","d6","d7","d8","d9","da","db","dc","dd","de","df","e0","e1","e2","e3","e4","e5","e6","e7","e8","e9","ea","eb","ec","ed","ee","ef","f0","f1","f2","f3","f4","f5","f6","f7","f8","f9","fa","fb","fc","fd","fe","ff"],Wr=Math.PI/180,Qo=180/Math.PI;function oi(){const n=Math.random()*4294967295|0,t=Math.random()*4294967295|0,e=Math.random()*4294967295|0,i=Math.random()*4294967295|0;return(Fe[n&255]+Fe[n>>8&255]+Fe[n>>16&255]+Fe[n>>24&255]+"-"+Fe[t&255]+Fe[t>>8&255]+"-"+Fe[t>>16&15|64]+Fe[t>>24&255]+"-"+Fe[e&63|128]+Fe[e>>8&255]+"-"+Fe[e>>16&255]+Fe[e>>24&255]+Fe[i&255]+Fe[i>>8&255]+Fe[i>>16&255]+Fe[i>>24&255]).toLowerCase()}function Yt(n,t,e){return Math.max(t,Math.min(e,n))}function Od(n,t){return(n%t+t)%t}function Ra(n,t,e){return(1-e)*n+e*t}function bn(n,t){switch(t.constructor){case Float32Array:return n;case Uint32Array:return n/4294967295;case Uint16Array:return n/65535;case Uint8Array:return n/255;case Int32Array:return Math.max(n/2147483647,-1);case Int16Array:return Math.max(n/32767,-1);case Int8Array:return Math.max(n/127,-1);default:throw new Error("Invalid component type.")}}function oe(n,t){switch(t.constructor){case Float32Array:return n;case Uint32Array:return Math.round(n*4294967295);case Uint16Array:return Math.round(n*65535);case Uint8Array:return Math.round(n*255);case Int32Array:return Math.round(n*2147483647);case Int16Array:return Math.round(n*32767);case Int8Array:return Math.round(n*127);default:throw new Error("Invalid component type.")}}const Bd={DEG2RAD:Wr};class Nt{static{Nt.prototype.isVector2=!0}constructor(t=0,e=0){this.x=t,this.y=e}get width(){return this.x}set width(t){this.x=t}get height(){return this.y}set height(t){this.y=t}set(t,e){return this.x=t,this.y=e,this}setScalar(t){return this.x=t,this.y=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y)}copy(t){return this.x=t.x,this.y=t.y,this}add(t){return this.x+=t.x,this.y+=t.y,this}addScalar(t){return this.x+=t,this.y+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this}subScalar(t){return this.x-=t,this.y-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this}multiply(t){return this.x*=t.x,this.y*=t.y,this}multiplyScalar(t){return this.x*=t,this.y*=t,this}divide(t){return this.x/=t.x,this.y/=t.y,this}divideScalar(t){return this.multiplyScalar(1/t)}applyMatrix3(t){const e=this.x,i=this.y,s=t.elements;return this.x=s[0]*e+s[3]*i+s[6],this.y=s[1]*e+s[4]*i+s[7],this}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this}clamp(t,e){return this.x=Yt(this.x,t.x,e.x),this.y=Yt(this.y,t.y,e.y),this}clampScalar(t,e){return this.x=Yt(this.x,t,e),this.y=Yt(this.y,t,e),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(Yt(i,t,e))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this}negate(){return this.x=-this.x,this.y=-this.y,this}dot(t){return this.x*t.x+this.y*t.y}cross(t){return this.x*t.y-this.y*t.x}lengthSq(){return this.x*this.x+this.y*this.y}length(){return Math.sqrt(this.x*this.x+this.y*this.y)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)}normalize(){return this.divideScalar(this.length()||1)}angle(){return Math.atan2(-this.y,-this.x)+Math.PI}angleTo(t){const e=Math.sqrt(this.lengthSq()*t.lengthSq());if(e===0)return Math.PI/2;const i=this.dot(t)/e;return Math.acos(Yt(i,-1,1))}distanceTo(t){return Math.sqrt(this.distanceToSquared(t))}distanceToSquared(t){const e=this.x-t.x,i=this.y-t.y;return e*e+i*i}manhattanDistanceTo(t){return Math.abs(this.x-t.x)+Math.abs(this.y-t.y)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this}equals(t){return t.x===this.x&&t.y===this.y}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this}rotateAround(t,e){const i=Math.cos(e),s=Math.sin(e),r=this.x-t.x,a=this.y-t.y;return this.x=r*i-a*s+t.x,this.y=r*s+a*i+t.y,this}random(){return this.x=Math.random(),this.y=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y}}class hi{constructor(t=0,e=0,i=0,s=1){this.isQuaternion=!0,this._x=t,this._y=e,this._z=i,this._w=s}static slerpFlat(t,e,i,s,r,a,o){let c=i[s+0],l=i[s+1],u=i[s+2],g=i[s+3],h=r[a+0],m=r[a+1],_=r[a+2],y=r[a+3];if(g!==y||c!==h||l!==m||u!==_){let f=c*h+l*m+u*_+g*y;f<0&&(h=-h,m=-m,_=-_,y=-y,f=-f);let d=1-o;if(f<.9995){const E=Math.acos(f),A=Math.sin(E);d=Math.sin(d*E)/A,o=Math.sin(o*E)/A,c=c*d+h*o,l=l*d+m*o,u=u*d+_*o,g=g*d+y*o}else{c=c*d+h*o,l=l*d+m*o,u=u*d+_*o,g=g*d+y*o;const E=1/Math.sqrt(c*c+l*l+u*u+g*g);c*=E,l*=E,u*=E,g*=E}}t[e]=c,t[e+1]=l,t[e+2]=u,t[e+3]=g}static multiplyQuaternionsFlat(t,e,i,s,r,a){const o=i[s],c=i[s+1],l=i[s+2],u=i[s+3],g=r[a],h=r[a+1],m=r[a+2],_=r[a+3];return t[e]=o*_+u*g+c*m-l*h,t[e+1]=c*_+u*h+l*g-o*m,t[e+2]=l*_+u*m+o*h-c*g,t[e+3]=u*_-o*g-c*h-l*m,t}get x(){return this._x}set x(t){this._x=t,this._onChangeCallback()}get y(){return this._y}set y(t){this._y=t,this._onChangeCallback()}get z(){return this._z}set z(t){this._z=t,this._onChangeCallback()}get w(){return this._w}set w(t){this._w=t,this._onChangeCallback()}set(t,e,i,s){return this._x=t,this._y=e,this._z=i,this._w=s,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._w)}copy(t){return this._x=t.x,this._y=t.y,this._z=t.z,this._w=t.w,this._onChangeCallback(),this}setFromEuler(t,e=!0){const i=t._x,s=t._y,r=t._z,a=t._order,o=Math.cos,c=Math.sin,l=o(i/2),u=o(s/2),g=o(r/2),h=c(i/2),m=c(s/2),_=c(r/2);switch(a){case"XYZ":this._x=h*u*g+l*m*_,this._y=l*m*g-h*u*_,this._z=l*u*_+h*m*g,this._w=l*u*g-h*m*_;break;case"YXZ":this._x=h*u*g+l*m*_,this._y=l*m*g-h*u*_,this._z=l*u*_-h*m*g,this._w=l*u*g+h*m*_;break;case"ZXY":this._x=h*u*g-l*m*_,this._y=l*m*g+h*u*_,this._z=l*u*_+h*m*g,this._w=l*u*g-h*m*_;break;case"ZYX":this._x=h*u*g-l*m*_,this._y=l*m*g+h*u*_,this._z=l*u*_-h*m*g,this._w=l*u*g+h*m*_;break;case"YZX":this._x=h*u*g+l*m*_,this._y=l*m*g+h*u*_,this._z=l*u*_-h*m*g,this._w=l*u*g-h*m*_;break;case"XZY":this._x=h*u*g-l*m*_,this._y=l*m*g-h*u*_,this._z=l*u*_+h*m*g,this._w=l*u*g+h*m*_;break;default:It("Quaternion: .setFromEuler() encountered an unknown order: "+a)}return e===!0&&this._onChangeCallback(),this}setFromAxisAngle(t,e){const i=e/2,s=Math.sin(i);return this._x=t.x*s,this._y=t.y*s,this._z=t.z*s,this._w=Math.cos(i),this._onChangeCallback(),this}setFromRotationMatrix(t){const e=t.elements,i=e[0],s=e[4],r=e[8],a=e[1],o=e[5],c=e[9],l=e[2],u=e[6],g=e[10],h=i+o+g;if(h>0){const m=.5/Math.sqrt(h+1);this._w=.25/m,this._x=(u-c)*m,this._y=(r-l)*m,this._z=(a-s)*m}else if(i>o&&i>g){const m=2*Math.sqrt(1+i-o-g);this._w=(u-c)/m,this._x=.25*m,this._y=(s+a)/m,this._z=(r+l)/m}else if(o>g){const m=2*Math.sqrt(1+o-i-g);this._w=(r-l)/m,this._x=(s+a)/m,this._y=.25*m,this._z=(c+u)/m}else{const m=2*Math.sqrt(1+g-i-o);this._w=(a-s)/m,this._x=(r+l)/m,this._y=(c+u)/m,this._z=.25*m}return this._onChangeCallback(),this}setFromUnitVectors(t,e){let i=t.dot(e)+1;return i<1e-8?(i=0,Math.abs(t.x)>Math.abs(t.z)?(this._x=-t.y,this._y=t.x,this._z=0,this._w=i):(this._x=0,this._y=-t.z,this._z=t.y,this._w=i)):(this._x=t.y*e.z-t.z*e.y,this._y=t.z*e.x-t.x*e.z,this._z=t.x*e.y-t.y*e.x,this._w=i),this.normalize()}angleTo(t){return 2*Math.acos(Math.abs(Yt(this.dot(t),-1,1)))}rotateTowards(t,e){const i=this.angleTo(t);if(i===0)return this;const s=Math.min(1,e/i);return this.slerp(t,s),this}identity(){return this.set(0,0,0,1)}invert(){return this.conjugate()}conjugate(){return this._x*=-1,this._y*=-1,this._z*=-1,this._onChangeCallback(),this}dot(t){return this._x*t._x+this._y*t._y+this._z*t._z+this._w*t._w}lengthSq(){return this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w}length(){return Math.sqrt(this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w)}normalize(){let t=this.length();return t===0?(this._x=0,this._y=0,this._z=0,this._w=1):(t=1/t,this._x=this._x*t,this._y=this._y*t,this._z=this._z*t,this._w=this._w*t),this._onChangeCallback(),this}multiply(t){return this.multiplyQuaternions(this,t)}premultiply(t){return this.multiplyQuaternions(t,this)}multiplyQuaternions(t,e){const i=t._x,s=t._y,r=t._z,a=t._w,o=e._x,c=e._y,l=e._z,u=e._w;return this._x=i*u+a*o+s*l-r*c,this._y=s*u+a*c+r*o-i*l,this._z=r*u+a*l+i*c-s*o,this._w=a*u-i*o-s*c-r*l,this._onChangeCallback(),this}slerp(t,e){let i=t._x,s=t._y,r=t._z,a=t._w,o=this.dot(t);o<0&&(i=-i,s=-s,r=-r,a=-a,o=-o);let c=1-e;if(o<.9995){const l=Math.acos(o),u=Math.sin(l);c=Math.sin(c*l)/u,e=Math.sin(e*l)/u,this._x=this._x*c+i*e,this._y=this._y*c+s*e,this._z=this._z*c+r*e,this._w=this._w*c+a*e,this._onChangeCallback()}else this._x=this._x*c+i*e,this._y=this._y*c+s*e,this._z=this._z*c+r*e,this._w=this._w*c+a*e,this.normalize();return this}slerpQuaternions(t,e,i){return this.copy(t).slerp(e,i)}random(){const t=2*Math.PI*Math.random(),e=2*Math.PI*Math.random(),i=Math.random(),s=Math.sqrt(1-i),r=Math.sqrt(i);return this.set(s*Math.sin(t),s*Math.cos(t),r*Math.sin(e),r*Math.cos(e))}equals(t){return t._x===this._x&&t._y===this._y&&t._z===this._z&&t._w===this._w}fromArray(t,e=0){return this._x=t[e],this._y=t[e+1],this._z=t[e+2],this._w=t[e+3],this._onChangeCallback(),this}toArray(t=[],e=0){return t[e]=this._x,t[e+1]=this._y,t[e+2]=this._z,t[e+3]=this._w,t}fromBufferAttribute(t,e){return this._x=t.getX(e),this._y=t.getY(e),this._z=t.getZ(e),this._w=t.getW(e),this._onChangeCallback(),this}toJSON(){return this.toArray()}_onChange(t){return this._onChangeCallback=t,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._w}}class F{static{F.prototype.isVector3=!0}constructor(t=0,e=0,i=0){this.x=t,this.y=e,this.z=i}set(t,e,i){return i===void 0&&(i=this.z),this.x=t,this.y=e,this.z=i,this}setScalar(t){return this.x=t,this.y=t,this.z=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setZ(t){return this.z=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;case 2:this.z=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;case 2:return this.z;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y,this.z)}copy(t){return this.x=t.x,this.y=t.y,this.z=t.z,this}add(t){return this.x+=t.x,this.y+=t.y,this.z+=t.z,this}addScalar(t){return this.x+=t,this.y+=t,this.z+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this.z=t.z+e.z,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this.z+=t.z*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this.z-=t.z,this}subScalar(t){return this.x-=t,this.y-=t,this.z-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this.z=t.z-e.z,this}multiply(t){return this.x*=t.x,this.y*=t.y,this.z*=t.z,this}multiplyScalar(t){return this.x*=t,this.y*=t,this.z*=t,this}multiplyVectors(t,e){return this.x=t.x*e.x,this.y=t.y*e.y,this.z=t.z*e.z,this}applyEuler(t){return this.applyQuaternion(lc.setFromEuler(t))}applyAxisAngle(t,e){return this.applyQuaternion(lc.setFromAxisAngle(t,e))}applyMatrix3(t){const e=this.x,i=this.y,s=this.z,r=t.elements;return this.x=r[0]*e+r[3]*i+r[6]*s,this.y=r[1]*e+r[4]*i+r[7]*s,this.z=r[2]*e+r[5]*i+r[8]*s,this}applyNormalMatrix(t){return this.applyMatrix3(t).normalize()}applyMatrix4(t){const e=this.x,i=this.y,s=this.z,r=t.elements,a=1/(r[3]*e+r[7]*i+r[11]*s+r[15]);return this.x=(r[0]*e+r[4]*i+r[8]*s+r[12])*a,this.y=(r[1]*e+r[5]*i+r[9]*s+r[13])*a,this.z=(r[2]*e+r[6]*i+r[10]*s+r[14])*a,this}applyQuaternion(t){const e=this.x,i=this.y,s=this.z,r=t.x,a=t.y,o=t.z,c=t.w,l=2*(a*s-o*i),u=2*(o*e-r*s),g=2*(r*i-a*e);return this.x=e+c*l+a*g-o*u,this.y=i+c*u+o*l-r*g,this.z=s+c*g+r*u-a*l,this}project(t){return this.applyMatrix4(t.matrixWorldInverse).applyMatrix4(t.projectionMatrix)}unproject(t){return this.applyMatrix4(t.projectionMatrixInverse).applyMatrix4(t.matrixWorld)}transformDirection(t){const e=this.x,i=this.y,s=this.z,r=t.elements;return this.x=r[0]*e+r[4]*i+r[8]*s,this.y=r[1]*e+r[5]*i+r[9]*s,this.z=r[2]*e+r[6]*i+r[10]*s,this.normalize()}divide(t){return this.x/=t.x,this.y/=t.y,this.z/=t.z,this}divideScalar(t){return this.multiplyScalar(1/t)}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this.z=Math.min(this.z,t.z),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this.z=Math.max(this.z,t.z),this}clamp(t,e){return this.x=Yt(this.x,t.x,e.x),this.y=Yt(this.y,t.y,e.y),this.z=Yt(this.z,t.z,e.z),this}clampScalar(t,e){return this.x=Yt(this.x,t,e),this.y=Yt(this.y,t,e),this.z=Yt(this.z,t,e),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(Yt(i,t,e))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this}dot(t){return this.x*t.x+this.y*t.y+this.z*t.z}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)}normalize(){return this.divideScalar(this.length()||1)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this.z+=(t.z-this.z)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this.z=t.z+(e.z-t.z)*i,this}cross(t){return this.crossVectors(this,t)}crossVectors(t,e){const i=t.x,s=t.y,r=t.z,a=e.x,o=e.y,c=e.z;return this.x=s*c-r*o,this.y=r*a-i*c,this.z=i*o-s*a,this}projectOnVector(t){const e=t.lengthSq();if(e===0)return this.set(0,0,0);const i=t.dot(this)/e;return this.copy(t).multiplyScalar(i)}projectOnPlane(t){return Ca.copy(this).projectOnVector(t),this.sub(Ca)}reflect(t){return this.sub(Ca.copy(t).multiplyScalar(2*this.dot(t)))}angleTo(t){const e=Math.sqrt(this.lengthSq()*t.lengthSq());if(e===0)return Math.PI/2;const i=this.dot(t)/e;return Math.acos(Yt(i,-1,1))}distanceTo(t){return Math.sqrt(this.distanceToSquared(t))}distanceToSquared(t){const e=this.x-t.x,i=this.y-t.y,s=this.z-t.z;return e*e+i*i+s*s}manhattanDistanceTo(t){return Math.abs(this.x-t.x)+Math.abs(this.y-t.y)+Math.abs(this.z-t.z)}setFromSpherical(t){return this.setFromSphericalCoords(t.radius,t.phi,t.theta)}setFromSphericalCoords(t,e,i){const s=Math.sin(e)*t;return this.x=s*Math.sin(i),this.y=Math.cos(e)*t,this.z=s*Math.cos(i),this}setFromCylindrical(t){return this.setFromCylindricalCoords(t.radius,t.theta,t.y)}setFromCylindricalCoords(t,e,i){return this.x=t*Math.sin(e),this.y=i,this.z=t*Math.cos(e),this}setFromMatrixPosition(t){const e=t.elements;return this.x=e[12],this.y=e[13],this.z=e[14],this}setFromMatrixScale(t){const e=this.setFromMatrixColumn(t,0).length(),i=this.setFromMatrixColumn(t,1).length(),s=this.setFromMatrixColumn(t,2).length();return this.x=e,this.y=i,this.z=s,this}setFromMatrixColumn(t,e){return this.fromArray(t.elements,e*4)}setFromMatrix3Column(t,e){return this.fromArray(t.elements,e*3)}setFromEuler(t){return this.x=t._x,this.y=t._y,this.z=t._z,this}setFromColor(t){return this.x=t.r,this.y=t.g,this.z=t.b,this}equals(t){return t.x===this.x&&t.y===this.y&&t.z===this.z}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this.z=t[e+2],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t[e+2]=this.z,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this.z=t.getZ(e),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this}randomDirection(){const t=Math.random()*Math.PI*2,e=Math.random()*2-1,i=Math.sqrt(1-e*e);return this.x=i*Math.cos(t),this.y=e,this.z=i*Math.sin(t),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z}}const Ca=new F,lc=new hi;class zt{static{zt.prototype.isMatrix3=!0}constructor(t,e,i,s,r,a,o,c,l){this.elements=[1,0,0,0,1,0,0,0,1],t!==void 0&&this.set(t,e,i,s,r,a,o,c,l)}set(t,e,i,s,r,a,o,c,l){const u=this.elements;return u[0]=t,u[1]=s,u[2]=o,u[3]=e,u[4]=r,u[5]=c,u[6]=i,u[7]=a,u[8]=l,this}identity(){return this.set(1,0,0,0,1,0,0,0,1),this}copy(t){const e=this.elements,i=t.elements;return e[0]=i[0],e[1]=i[1],e[2]=i[2],e[3]=i[3],e[4]=i[4],e[5]=i[5],e[6]=i[6],e[7]=i[7],e[8]=i[8],this}extractBasis(t,e,i){return t.setFromMatrix3Column(this,0),e.setFromMatrix3Column(this,1),i.setFromMatrix3Column(this,2),this}setFromMatrix4(t){const e=t.elements;return this.set(e[0],e[4],e[8],e[1],e[5],e[9],e[2],e[6],e[10]),this}multiply(t){return this.multiplyMatrices(this,t)}premultiply(t){return this.multiplyMatrices(t,this)}multiplyMatrices(t,e){const i=t.elements,s=e.elements,r=this.elements,a=i[0],o=i[3],c=i[6],l=i[1],u=i[4],g=i[7],h=i[2],m=i[5],_=i[8],y=s[0],f=s[3],d=s[6],E=s[1],A=s[4],T=s[7],D=s[2],b=s[5],N=s[8];return r[0]=a*y+o*E+c*D,r[3]=a*f+o*A+c*b,r[6]=a*d+o*T+c*N,r[1]=l*y+u*E+g*D,r[4]=l*f+u*A+g*b,r[7]=l*d+u*T+g*N,r[2]=h*y+m*E+_*D,r[5]=h*f+m*A+_*b,r[8]=h*d+m*T+_*N,this}multiplyScalar(t){const e=this.elements;return e[0]*=t,e[3]*=t,e[6]*=t,e[1]*=t,e[4]*=t,e[7]*=t,e[2]*=t,e[5]*=t,e[8]*=t,this}determinant(){const t=this.elements,e=t[0],i=t[1],s=t[2],r=t[3],a=t[4],o=t[5],c=t[6],l=t[7],u=t[8];return e*a*u-e*o*l-i*r*u+i*o*c+s*r*l-s*a*c}invert(){const t=this.elements,e=t[0],i=t[1],s=t[2],r=t[3],a=t[4],o=t[5],c=t[6],l=t[7],u=t[8],g=u*a-o*l,h=o*c-u*r,m=l*r-a*c,_=e*g+i*h+s*m;if(_===0)return this.set(0,0,0,0,0,0,0,0,0);const y=1/_;return t[0]=g*y,t[1]=(s*l-u*i)*y,t[2]=(o*i-s*a)*y,t[3]=h*y,t[4]=(u*e-s*c)*y,t[5]=(s*r-o*e)*y,t[6]=m*y,t[7]=(i*c-l*e)*y,t[8]=(a*e-i*r)*y,this}transpose(){let t;const e=this.elements;return t=e[1],e[1]=e[3],e[3]=t,t=e[2],e[2]=e[6],e[6]=t,t=e[5],e[5]=e[7],e[7]=t,this}getNormalMatrix(t){return this.setFromMatrix4(t).invert().transpose()}transposeIntoArray(t){const e=this.elements;return t[0]=e[0],t[1]=e[3],t[2]=e[6],t[3]=e[1],t[4]=e[4],t[5]=e[7],t[6]=e[2],t[7]=e[5],t[8]=e[8],this}setUvTransform(t,e,i,s,r,a,o){const c=Math.cos(r),l=Math.sin(r);return this.set(i*c,i*l,-i*(c*a+l*o)+a+t,-s*l,s*c,-s*(-l*a+c*o)+o+e,0,0,1),this}scale(t,e){return this.premultiply(Pa.makeScale(t,e)),this}rotate(t){return this.premultiply(Pa.makeRotation(-t)),this}translate(t,e){return this.premultiply(Pa.makeTranslation(t,e)),this}makeTranslation(t,e){return t.isVector2?this.set(1,0,t.x,0,1,t.y,0,0,1):this.set(1,0,t,0,1,e,0,0,1),this}makeRotation(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,-i,0,i,e,0,0,0,1),this}makeScale(t,e){return this.set(t,0,0,0,e,0,0,0,1),this}equals(t){const e=this.elements,i=t.elements;for(let s=0;s<9;s++)if(e[s]!==i[s])return!1;return!0}fromArray(t,e=0){for(let i=0;i<9;i++)this.elements[i]=t[i+e];return this}toArray(t=[],e=0){const i=this.elements;return t[e]=i[0],t[e+1]=i[1],t[e+2]=i[2],t[e+3]=i[3],t[e+4]=i[4],t[e+5]=i[5],t[e+6]=i[6],t[e+7]=i[7],t[e+8]=i[8],t}clone(){return new this.constructor().fromArray(this.elements)}}const Pa=new zt,cc=new zt().set(.4123908,.3575843,.1804808,.212639,.7151687,.0721923,.0193308,.1191948,.9505322),hc=new zt().set(3.2409699,-1.5373832,-.4986108,-.9692436,1.8759675,.0415551,.0556301,-.203977,1.0569715);function zd(){const n={enabled:!0,workingColorSpace:ia,spaces:{},convert:function(s,r,a){return this.enabled===!1||r===a||!r||!a||(this.spaces[r].transfer===ie&&(s.r=Gn(s.r),s.g=Gn(s.g),s.b=Gn(s.b)),this.spaces[r].primaries!==this.spaces[a].primaries&&(s.applyMatrix3(this.spaces[r].toXYZ),s.applyMatrix3(this.spaces[a].fromXYZ)),this.spaces[a].transfer===ie&&(s.r=ds(s.r),s.g=ds(s.g),s.b=ds(s.b))),s},workingToColorSpace:function(s,r){return this.convert(s,this.workingColorSpace,r)},colorSpaceToWorking:function(s,r){return this.convert(s,r,this.workingColorSpace)},getPrimaries:function(s){return this.spaces[s].primaries},getTransfer:function(s){return s===si?sa:this.spaces[s].transfer},getToneMappingMode:function(s){return this.spaces[s].outputColorSpaceConfig.toneMappingMode||"standard"},getLuminanceCoefficients:function(s,r=this.workingColorSpace){return s.fromArray(this.spaces[r].luminanceCoefficients)},define:function(s){Object.assign(this.spaces,s)},_getMatrix:function(s,r,a){return s.copy(this.spaces[r].toXYZ).multiply(this.spaces[a].fromXYZ)},_getDrawingBufferColorSpace:function(s){return this.spaces[s].outputColorSpaceConfig.drawingBufferColorSpace},_getUnpackColorSpace:function(s=this.workingColorSpace){return this.spaces[s].workingColorSpaceConfig.unpackColorSpace},fromWorkingColorSpace:function(s,r){return Jo("ColorManagement: .fromWorkingColorSpace() has been renamed to .workingToColorSpace()."),n.workingToColorSpace(s,r)},toWorkingColorSpace:function(s,r){return Jo("ColorManagement: .toWorkingColorSpace() has been renamed to .colorSpaceToWorking()."),n.colorSpaceToWorking(s,r)}},t=[.64,.33,.3,.6,.15,.06],e=[.2126,.7152,.0722],i=[.3127,.329];return n.define({[ia]:{primaries:t,whitePoint:i,transfer:sa,toXYZ:cc,fromXYZ:hc,luminanceCoefficients:e,workingColorSpaceConfig:{unpackColorSpace:sn},outputColorSpaceConfig:{drawingBufferColorSpace:sn}},[sn]:{primaries:t,whitePoint:i,transfer:ie,toXYZ:cc,fromXYZ:hc,luminanceCoefficients:e,outputColorSpaceConfig:{drawingBufferColorSpace:sn}}}),n}const jt=zd();function Gn(n){return n<.04045?n*.0773993808:Math.pow(n*.9478672986+.0521327014,2.4)}function ds(n){return n<.0031308?n*12.92:1.055*Math.pow(n,.41666)-.055}let Hi;class kd{static getDataURL(t,e="image/png"){if(/^data:/i.test(t.src)||typeof HTMLCanvasElement>"u")return t.src;let i;if(t instanceof HTMLCanvasElement)i=t;else{Hi===void 0&&(Hi=aa("canvas")),Hi.width=t.width,Hi.height=t.height;const s=Hi.getContext("2d");t instanceof ImageData?s.putImageData(t,0,0):s.drawImage(t,0,0,t.width,t.height),i=Hi}return i.toDataURL(e)}static sRGBToLinear(t){if(typeof HTMLImageElement<"u"&&t instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&t instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&t instanceof ImageBitmap){const e=aa("canvas");e.width=t.width,e.height=t.height;const i=e.getContext("2d");i.drawImage(t,0,0,t.width,t.height);const s=i.getImageData(0,0,t.width,t.height),r=s.data;for(let a=0;a<r.length;a++)r[a]=Gn(r[a]/255)*255;return i.putImageData(s,0,0),e}else if(t.data){const e=t.data.slice(0);for(let i=0;i<e.length;i++)e instanceof Uint8Array||e instanceof Uint8ClampedArray?e[i]=Math.floor(Gn(e[i]/255)*255):e[i]=Gn(e[i]);return{data:e,width:t.width,height:t.height}}else return It("ImageUtils.sRGBToLinear(): Unsupported image type. No color space conversion applied."),t}}let Hd=0;class Tl{constructor(t=null){this.isSource=!0,Object.defineProperty(this,"id",{value:Hd++}),this.uuid=oi(),this.data=t,this.dataReady=!0,this.version=0}getSize(t){const e=this.data;return typeof HTMLVideoElement<"u"&&e instanceof HTMLVideoElement?t.set(e.videoWidth,e.videoHeight,0):typeof VideoFrame<"u"&&e instanceof VideoFrame?t.set(e.displayWidth,e.displayHeight,0):e!==null?t.set(e.width,e.height,e.depth||0):t.set(0,0,0),t}set needsUpdate(t){t===!0&&this.version++}toJSON(t){const e=t===void 0||typeof t=="string";if(!e&&t.images[this.uuid]!==void 0)return t.images[this.uuid];const i={uuid:this.uuid,url:""},s=this.data;if(s!==null){let r;if(Array.isArray(s)){r=[];for(let a=0,o=s.length;a<o;a++)s[a].isDataTexture?r.push(La(s[a].image)):r.push(La(s[a]))}else r=La(s);i.url=r}return e||(t.images[this.uuid]=i),i}}function La(n){return typeof HTMLImageElement<"u"&&n instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&n instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&n instanceof ImageBitmap?kd.getDataURL(n):n.data?{data:Array.from(n.data),width:n.width,height:n.height,type:n.data.constructor.name}:(It("Texture: Unable to serialize Texture."),{})}let Gd=0;const Da=new F;class ze extends fi{constructor(t=ze.DEFAULT_IMAGE,e=ze.DEFAULT_MAPPING,i=kn,s=kn,r=Be,a=Ti,o=pn,c=an,l=ze.DEFAULT_ANISOTROPY,u=si){super(),this.isTexture=!0,Object.defineProperty(this,"id",{value:Gd++}),this.uuid=oi(),this.name="",this.source=new Tl(t),this.mipmaps=[],this.mapping=e,this.channel=0,this.wrapS=i,this.wrapT=s,this.magFilter=r,this.minFilter=a,this.anisotropy=l,this.format=o,this.internalFormat=null,this.type=c,this.offset=new Nt(0,0),this.repeat=new Nt(1,1),this.center=new Nt(0,0),this.rotation=0,this.matrixAutoUpdate=!0,this.matrix=new zt,this.generateMipmaps=!0,this.premultiplyAlpha=!1,this.flipY=!0,this.unpackAlignment=4,this.colorSpace=u,this.userData={},this.updateRanges=[],this.version=0,this.onUpdate=null,this.renderTarget=null,this.isRenderTargetTexture=!1,this.isArrayTexture=!!(t&&t.depth&&t.depth>1),this.pmremVersion=0,this.normalized=!1}get width(){return this.source.getSize(Da).x}get height(){return this.source.getSize(Da).y}get depth(){return this.source.getSize(Da).z}get image(){return this.source.data}set image(t){this.source.data=t}updateMatrix(){this.matrix.setUvTransform(this.offset.x,this.offset.y,this.repeat.x,this.repeat.y,this.rotation,this.center.x,this.center.y)}addUpdateRange(t,e){this.updateRanges.push({start:t,count:e})}clearUpdateRanges(){this.updateRanges.length=0}clone(){return new this.constructor().copy(this)}copy(t){return this.name=t.name,this.source=t.source,this.mipmaps=t.mipmaps.slice(0),this.mapping=t.mapping,this.channel=t.channel,this.wrapS=t.wrapS,this.wrapT=t.wrapT,this.magFilter=t.magFilter,this.minFilter=t.minFilter,this.anisotropy=t.anisotropy,this.format=t.format,this.internalFormat=t.internalFormat,this.type=t.type,this.normalized=t.normalized,this.offset.copy(t.offset),this.repeat.copy(t.repeat),this.center.copy(t.center),this.rotation=t.rotation,this.matrixAutoUpdate=t.matrixAutoUpdate,this.matrix.copy(t.matrix),this.generateMipmaps=t.generateMipmaps,this.premultiplyAlpha=t.premultiplyAlpha,this.flipY=t.flipY,this.unpackAlignment=t.unpackAlignment,this.colorSpace=t.colorSpace,this.renderTarget=t.renderTarget,this.isRenderTargetTexture=t.isRenderTargetTexture,this.isArrayTexture=t.isArrayTexture,this.userData=JSON.parse(JSON.stringify(t.userData)),this.needsUpdate=!0,this}setValues(t){for(const e in t){const i=t[e];if(i===void 0){It(`Texture.setValues(): parameter '${e}' has value of undefined.`);continue}const s=this[e];if(s===void 0){It(`Texture.setValues(): property '${e}' does not exist.`);continue}s&&i&&s.isVector2&&i.isVector2||s&&i&&s.isVector3&&i.isVector3||s&&i&&s.isMatrix3&&i.isMatrix3?s.copy(i):this[e]=i}}toJSON(t){const e=t===void 0||typeof t=="string";if(!e&&t.textures[this.uuid]!==void 0)return t.textures[this.uuid];const i={metadata:{version:4.7,type:"Texture",generator:"Texture.toJSON"},uuid:this.uuid,name:this.name,image:this.source.toJSON(t).uuid,mapping:this.mapping,channel:this.channel,repeat:[this.repeat.x,this.repeat.y],offset:[this.offset.x,this.offset.y],center:[this.center.x,this.center.y],rotation:this.rotation,wrap:[this.wrapS,this.wrapT],format:this.format,internalFormat:this.internalFormat,type:this.type,normalized:this.normalized,colorSpace:this.colorSpace,minFilter:this.minFilter,magFilter:this.magFilter,anisotropy:this.anisotropy,flipY:this.flipY,generateMipmaps:this.generateMipmaps,premultiplyAlpha:this.premultiplyAlpha,unpackAlignment:this.unpackAlignment};return Object.keys(this.userData).length>0&&(i.userData=this.userData),e||(t.textures[this.uuid]=i),i}dispose(){this.dispatchEvent({type:"dispose"})}transformUv(t){if(this.mapping!==Ah)return t;if(t.applyMatrix3(this.matrix),t.x<0||t.x>1)switch(this.wrapS){case vo:t.x=t.x-Math.floor(t.x);break;case kn:t.x=t.x<0?0:1;break;case Mo:Math.abs(Math.floor(t.x)%2)===1?t.x=Math.ceil(t.x)-t.x:t.x=t.x-Math.floor(t.x);break}if(t.y<0||t.y>1)switch(this.wrapT){case vo:t.y=t.y-Math.floor(t.y);break;case kn:t.y=t.y<0?0:1;break;case Mo:Math.abs(Math.floor(t.y)%2)===1?t.y=Math.ceil(t.y)-t.y:t.y=t.y-Math.floor(t.y);break}return this.flipY&&(t.y=1-t.y),t}set needsUpdate(t){t===!0&&(this.version++,this.source.needsUpdate=!0)}set needsPMREMUpdate(t){t===!0&&this.pmremVersion++}}ze.DEFAULT_IMAGE=null;ze.DEFAULT_MAPPING=Ah;ze.DEFAULT_ANISOTROPY=1;class ye{static{ye.prototype.isVector4=!0}constructor(t=0,e=0,i=0,s=1){this.x=t,this.y=e,this.z=i,this.w=s}get width(){return this.z}set width(t){this.z=t}get height(){return this.w}set height(t){this.w=t}set(t,e,i,s){return this.x=t,this.y=e,this.z=i,this.w=s,this}setScalar(t){return this.x=t,this.y=t,this.z=t,this.w=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setZ(t){return this.z=t,this}setW(t){return this.w=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;case 2:this.z=e;break;case 3:this.w=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;case 2:return this.z;case 3:return this.w;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y,this.z,this.w)}copy(t){return this.x=t.x,this.y=t.y,this.z=t.z,this.w=t.w!==void 0?t.w:1,this}add(t){return this.x+=t.x,this.y+=t.y,this.z+=t.z,this.w+=t.w,this}addScalar(t){return this.x+=t,this.y+=t,this.z+=t,this.w+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this.z=t.z+e.z,this.w=t.w+e.w,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this.z+=t.z*e,this.w+=t.w*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this.z-=t.z,this.w-=t.w,this}subScalar(t){return this.x-=t,this.y-=t,this.z-=t,this.w-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this.z=t.z-e.z,this.w=t.w-e.w,this}multiply(t){return this.x*=t.x,this.y*=t.y,this.z*=t.z,this.w*=t.w,this}multiplyScalar(t){return this.x*=t,this.y*=t,this.z*=t,this.w*=t,this}applyMatrix4(t){const e=this.x,i=this.y,s=this.z,r=this.w,a=t.elements;return this.x=a[0]*e+a[4]*i+a[8]*s+a[12]*r,this.y=a[1]*e+a[5]*i+a[9]*s+a[13]*r,this.z=a[2]*e+a[6]*i+a[10]*s+a[14]*r,this.w=a[3]*e+a[7]*i+a[11]*s+a[15]*r,this}divide(t){return this.x/=t.x,this.y/=t.y,this.z/=t.z,this.w/=t.w,this}divideScalar(t){return this.multiplyScalar(1/t)}setAxisAngleFromQuaternion(t){this.w=2*Math.acos(t.w);const e=Math.sqrt(1-t.w*t.w);return e<1e-4?(this.x=1,this.y=0,this.z=0):(this.x=t.x/e,this.y=t.y/e,this.z=t.z/e),this}setAxisAngleFromRotationMatrix(t){let e,i,s,r;const c=t.elements,l=c[0],u=c[4],g=c[8],h=c[1],m=c[5],_=c[9],y=c[2],f=c[6],d=c[10];if(Math.abs(u-h)<.01&&Math.abs(g-y)<.01&&Math.abs(_-f)<.01){if(Math.abs(u+h)<.1&&Math.abs(g+y)<.1&&Math.abs(_+f)<.1&&Math.abs(l+m+d-3)<.1)return this.set(1,0,0,0),this;e=Math.PI;const A=(l+1)/2,T=(m+1)/2,D=(d+1)/2,b=(u+h)/4,N=(g+y)/4,M=(_+f)/4;return A>T&&A>D?A<.01?(i=0,s=.707106781,r=.707106781):(i=Math.sqrt(A),s=b/i,r=N/i):T>D?T<.01?(i=.707106781,s=0,r=.707106781):(s=Math.sqrt(T),i=b/s,r=M/s):D<.01?(i=.707106781,s=.707106781,r=0):(r=Math.sqrt(D),i=N/r,s=M/r),this.set(i,s,r,e),this}let E=Math.sqrt((f-_)*(f-_)+(g-y)*(g-y)+(h-u)*(h-u));return Math.abs(E)<.001&&(E=1),this.x=(f-_)/E,this.y=(g-y)/E,this.z=(h-u)/E,this.w=Math.acos((l+m+d-1)/2),this}setFromMatrixPosition(t){const e=t.elements;return this.x=e[12],this.y=e[13],this.z=e[14],this.w=e[15],this}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this.z=Math.min(this.z,t.z),this.w=Math.min(this.w,t.w),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this.z=Math.max(this.z,t.z),this.w=Math.max(this.w,t.w),this}clamp(t,e){return this.x=Yt(this.x,t.x,e.x),this.y=Yt(this.y,t.y,e.y),this.z=Yt(this.z,t.z,e.z),this.w=Yt(this.w,t.w,e.w),this}clampScalar(t,e){return this.x=Yt(this.x,t,e),this.y=Yt(this.y,t,e),this.z=Yt(this.z,t,e),this.w=Yt(this.w,t,e),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(Yt(i,t,e))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this.w=Math.floor(this.w),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this.w=Math.ceil(this.w),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this.w=Math.round(this.w),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this.w=Math.trunc(this.w),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this.w=-this.w,this}dot(t){return this.x*t.x+this.y*t.y+this.z*t.z+this.w*t.w}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)+Math.abs(this.w)}normalize(){return this.divideScalar(this.length()||1)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this.z+=(t.z-this.z)*e,this.w+=(t.w-this.w)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this.z=t.z+(e.z-t.z)*i,this.w=t.w+(e.w-t.w)*i,this}equals(t){return t.x===this.x&&t.y===this.y&&t.z===this.z&&t.w===this.w}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this.z=t[e+2],this.w=t[e+3],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t[e+2]=this.z,t[e+3]=this.w,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this.z=t.getZ(e),this.w=t.getW(e),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this.w=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z,yield this.w}}class Vd extends fi{constructor(t=1,e=1,i={}){super(),i=Object.assign({generateMipmaps:!1,internalFormat:null,minFilter:Be,depthBuffer:!0,stencilBuffer:!1,resolveDepthBuffer:!0,resolveStencilBuffer:!0,depthTexture:null,samples:0,count:1,depth:1,multiview:!1},i),this.isRenderTarget=!0,this.width=t,this.height=e,this.depth=i.depth,this.scissor=new ye(0,0,t,e),this.scissorTest=!1,this.viewport=new ye(0,0,t,e),this.textures=[];const s={width:t,height:e,depth:i.depth},r=new ze(s),a=i.count;for(let o=0;o<a;o++)this.textures[o]=r.clone(),this.textures[o].isRenderTargetTexture=!0,this.textures[o].renderTarget=this;this._setTextureOptions(i),this.depthBuffer=i.depthBuffer,this.stencilBuffer=i.stencilBuffer,this.resolveDepthBuffer=i.resolveDepthBuffer,this.resolveStencilBuffer=i.resolveStencilBuffer,this._depthTexture=null,this.depthTexture=i.depthTexture,this.samples=i.samples,this.multiview=i.multiview}_setTextureOptions(t={}){const e={minFilter:Be,generateMipmaps:!1,flipY:!1,internalFormat:null};t.mapping!==void 0&&(e.mapping=t.mapping),t.wrapS!==void 0&&(e.wrapS=t.wrapS),t.wrapT!==void 0&&(e.wrapT=t.wrapT),t.wrapR!==void 0&&(e.wrapR=t.wrapR),t.magFilter!==void 0&&(e.magFilter=t.magFilter),t.minFilter!==void 0&&(e.minFilter=t.minFilter),t.format!==void 0&&(e.format=t.format),t.type!==void 0&&(e.type=t.type),t.anisotropy!==void 0&&(e.anisotropy=t.anisotropy),t.colorSpace!==void 0&&(e.colorSpace=t.colorSpace),t.flipY!==void 0&&(e.flipY=t.flipY),t.generateMipmaps!==void 0&&(e.generateMipmaps=t.generateMipmaps),t.internalFormat!==void 0&&(e.internalFormat=t.internalFormat);for(let i=0;i<this.textures.length;i++)this.textures[i].setValues(e)}get texture(){return this.textures[0]}set texture(t){this.textures[0]=t}set depthTexture(t){this._depthTexture!==null&&(this._depthTexture.renderTarget=null),t!==null&&(t.renderTarget=this),this._depthTexture=t}get depthTexture(){return this._depthTexture}setSize(t,e,i=1){if(this.width!==t||this.height!==e||this.depth!==i){this.width=t,this.height=e,this.depth=i;for(let s=0,r=this.textures.length;s<r;s++)this.textures[s].image.width=t,this.textures[s].image.height=e,this.textures[s].image.depth=i,this.textures[s].isData3DTexture!==!0&&(this.textures[s].isArrayTexture=this.textures[s].image.depth>1);this.dispose()}this.viewport.set(0,0,t,e),this.scissor.set(0,0,t,e)}clone(){return new this.constructor().copy(this)}copy(t){this.width=t.width,this.height=t.height,this.depth=t.depth,this.scissor.copy(t.scissor),this.scissorTest=t.scissorTest,this.viewport.copy(t.viewport),this.textures.length=0;for(let e=0,i=t.textures.length;e<i;e++){this.textures[e]=t.textures[e].clone(),this.textures[e].isRenderTargetTexture=!0,this.textures[e].renderTarget=this;const s=Object.assign({},t.textures[e].image);this.textures[e].source=new Tl(s)}return this.depthBuffer=t.depthBuffer,this.stencilBuffer=t.stencilBuffer,this.resolveDepthBuffer=t.resolveDepthBuffer,this.resolveStencilBuffer=t.resolveStencilBuffer,t.depthTexture!==null&&(this.depthTexture=t.depthTexture.clone()),this.samples=t.samples,this.multiview=t.multiview,this}dispose(){this.dispatchEvent({type:"dispose"})}}class Pn extends Vd{constructor(t=1,e=1,i={}){super(t,e,i),this.isWebGLRenderTarget=!0}}class Nh extends ze{constructor(t=null,e=1,i=1,s=1){super(null),this.isDataArrayTexture=!0,this.image={data:t,width:e,height:i,depth:s},this.magFilter=Le,this.minFilter=Le,this.wrapR=kn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1,this.layerUpdates=new Set}addLayerUpdate(t){this.layerUpdates.add(t)}clearLayerUpdates(){this.layerUpdates.clear()}}class Wd extends ze{constructor(t=null,e=1,i=1,s=1){super(null),this.isData3DTexture=!0,this.image={data:t,width:e,height:i,depth:s},this.magFilter=Le,this.minFilter=Le,this.wrapR=kn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}}class me{static{me.prototype.isMatrix4=!0}constructor(t,e,i,s,r,a,o,c,l,u,g,h,m,_,y,f){this.elements=[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],t!==void 0&&this.set(t,e,i,s,r,a,o,c,l,u,g,h,m,_,y,f)}set(t,e,i,s,r,a,o,c,l,u,g,h,m,_,y,f){const d=this.elements;return d[0]=t,d[4]=e,d[8]=i,d[12]=s,d[1]=r,d[5]=a,d[9]=o,d[13]=c,d[2]=l,d[6]=u,d[10]=g,d[14]=h,d[3]=m,d[7]=_,d[11]=y,d[15]=f,this}identity(){return this.set(1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1),this}clone(){return new me().fromArray(this.elements)}copy(t){const e=this.elements,i=t.elements;return e[0]=i[0],e[1]=i[1],e[2]=i[2],e[3]=i[3],e[4]=i[4],e[5]=i[5],e[6]=i[6],e[7]=i[7],e[8]=i[8],e[9]=i[9],e[10]=i[10],e[11]=i[11],e[12]=i[12],e[13]=i[13],e[14]=i[14],e[15]=i[15],this}copyPosition(t){const e=this.elements,i=t.elements;return e[12]=i[12],e[13]=i[13],e[14]=i[14],this}setFromMatrix3(t){const e=t.elements;return this.set(e[0],e[3],e[6],0,e[1],e[4],e[7],0,e[2],e[5],e[8],0,0,0,0,1),this}extractBasis(t,e,i){return this.determinant()===0?(t.set(1,0,0),e.set(0,1,0),i.set(0,0,1),this):(t.setFromMatrixColumn(this,0),e.setFromMatrixColumn(this,1),i.setFromMatrixColumn(this,2),this)}makeBasis(t,e,i){return this.set(t.x,e.x,i.x,0,t.y,e.y,i.y,0,t.z,e.z,i.z,0,0,0,0,1),this}extractRotation(t){if(t.determinant()===0)return this.identity();const e=this.elements,i=t.elements,s=1/Gi.setFromMatrixColumn(t,0).length(),r=1/Gi.setFromMatrixColumn(t,1).length(),a=1/Gi.setFromMatrixColumn(t,2).length();return e[0]=i[0]*s,e[1]=i[1]*s,e[2]=i[2]*s,e[3]=0,e[4]=i[4]*r,e[5]=i[5]*r,e[6]=i[6]*r,e[7]=0,e[8]=i[8]*a,e[9]=i[9]*a,e[10]=i[10]*a,e[11]=0,e[12]=0,e[13]=0,e[14]=0,e[15]=1,this}makeRotationFromEuler(t){const e=this.elements,i=t.x,s=t.y,r=t.z,a=Math.cos(i),o=Math.sin(i),c=Math.cos(s),l=Math.sin(s),u=Math.cos(r),g=Math.sin(r);if(t.order==="XYZ"){const h=a*u,m=a*g,_=o*u,y=o*g;e[0]=c*u,e[4]=-c*g,e[8]=l,e[1]=m+_*l,e[5]=h-y*l,e[9]=-o*c,e[2]=y-h*l,e[6]=_+m*l,e[10]=a*c}else if(t.order==="YXZ"){const h=c*u,m=c*g,_=l*u,y=l*g;e[0]=h+y*o,e[4]=_*o-m,e[8]=a*l,e[1]=a*g,e[5]=a*u,e[9]=-o,e[2]=m*o-_,e[6]=y+h*o,e[10]=a*c}else if(t.order==="ZXY"){const h=c*u,m=c*g,_=l*u,y=l*g;e[0]=h-y*o,e[4]=-a*g,e[8]=_+m*o,e[1]=m+_*o,e[5]=a*u,e[9]=y-h*o,e[2]=-a*l,e[6]=o,e[10]=a*c}else if(t.order==="ZYX"){const h=a*u,m=a*g,_=o*u,y=o*g;e[0]=c*u,e[4]=_*l-m,e[8]=h*l+y,e[1]=c*g,e[5]=y*l+h,e[9]=m*l-_,e[2]=-l,e[6]=o*c,e[10]=a*c}else if(t.order==="YZX"){const h=a*c,m=a*l,_=o*c,y=o*l;e[0]=c*u,e[4]=y-h*g,e[8]=_*g+m,e[1]=g,e[5]=a*u,e[9]=-o*u,e[2]=-l*u,e[6]=m*g+_,e[10]=h-y*g}else if(t.order==="XZY"){const h=a*c,m=a*l,_=o*c,y=o*l;e[0]=c*u,e[4]=-g,e[8]=l*u,e[1]=h*g+y,e[5]=a*u,e[9]=m*g-_,e[2]=_*g-m,e[6]=o*u,e[10]=y*g+h}return e[3]=0,e[7]=0,e[11]=0,e[12]=0,e[13]=0,e[14]=0,e[15]=1,this}makeRotationFromQuaternion(t){return this.compose(Xd,t,$d)}lookAt(t,e,i){const s=this.elements;return Je.subVectors(t,e),Je.lengthSq()===0&&(Je.z=1),Je.normalize(),Zn.crossVectors(i,Je),Zn.lengthSq()===0&&(Math.abs(i.z)===1?Je.x+=1e-4:Je.z+=1e-4,Je.normalize(),Zn.crossVectors(i,Je)),Zn.normalize(),nr.crossVectors(Je,Zn),s[0]=Zn.x,s[4]=nr.x,s[8]=Je.x,s[1]=Zn.y,s[5]=nr.y,s[9]=Je.y,s[2]=Zn.z,s[6]=nr.z,s[10]=Je.z,this}multiply(t){return this.multiplyMatrices(this,t)}premultiply(t){return this.multiplyMatrices(t,this)}multiplyMatrices(t,e){const i=t.elements,s=e.elements,r=this.elements,a=i[0],o=i[4],c=i[8],l=i[12],u=i[1],g=i[5],h=i[9],m=i[13],_=i[2],y=i[6],f=i[10],d=i[14],E=i[3],A=i[7],T=i[11],D=i[15],b=s[0],N=s[4],M=s[8],w=s[12],z=s[1],P=s[5],V=s[9],J=s[13],K=s[2],B=s[6],W=s[10],$=s[14],et=s[3],at=s[7],ct=s[11],yt=s[15];return r[0]=a*b+o*z+c*K+l*et,r[4]=a*N+o*P+c*B+l*at,r[8]=a*M+o*V+c*W+l*ct,r[12]=a*w+o*J+c*$+l*yt,r[1]=u*b+g*z+h*K+m*et,r[5]=u*N+g*P+h*B+m*at,r[9]=u*M+g*V+h*W+m*ct,r[13]=u*w+g*J+h*$+m*yt,r[2]=_*b+y*z+f*K+d*et,r[6]=_*N+y*P+f*B+d*at,r[10]=_*M+y*V+f*W+d*ct,r[14]=_*w+y*J+f*$+d*yt,r[3]=E*b+A*z+T*K+D*et,r[7]=E*N+A*P+T*B+D*at,r[11]=E*M+A*V+T*W+D*ct,r[15]=E*w+A*J+T*$+D*yt,this}multiplyScalar(t){const e=this.elements;return e[0]*=t,e[4]*=t,e[8]*=t,e[12]*=t,e[1]*=t,e[5]*=t,e[9]*=t,e[13]*=t,e[2]*=t,e[6]*=t,e[10]*=t,e[14]*=t,e[3]*=t,e[7]*=t,e[11]*=t,e[15]*=t,this}determinant(){const t=this.elements,e=t[0],i=t[4],s=t[8],r=t[12],a=t[1],o=t[5],c=t[9],l=t[13],u=t[2],g=t[6],h=t[10],m=t[14],_=t[3],y=t[7],f=t[11],d=t[15],E=c*m-l*h,A=o*m-l*g,T=o*h-c*g,D=a*m-l*u,b=a*h-c*u,N=a*g-o*u;return e*(y*E-f*A+d*T)-i*(_*E-f*D+d*b)+s*(_*A-y*D+d*N)-r*(_*T-y*b+f*N)}transpose(){const t=this.elements;let e;return e=t[1],t[1]=t[4],t[4]=e,e=t[2],t[2]=t[8],t[8]=e,e=t[6],t[6]=t[9],t[9]=e,e=t[3],t[3]=t[12],t[12]=e,e=t[7],t[7]=t[13],t[13]=e,e=t[11],t[11]=t[14],t[14]=e,this}setPosition(t,e,i){const s=this.elements;return t.isVector3?(s[12]=t.x,s[13]=t.y,s[14]=t.z):(s[12]=t,s[13]=e,s[14]=i),this}invert(){const t=this.elements,e=t[0],i=t[1],s=t[2],r=t[3],a=t[4],o=t[5],c=t[6],l=t[7],u=t[8],g=t[9],h=t[10],m=t[11],_=t[12],y=t[13],f=t[14],d=t[15],E=e*o-i*a,A=e*c-s*a,T=e*l-r*a,D=i*c-s*o,b=i*l-r*o,N=s*l-r*c,M=u*y-g*_,w=u*f-h*_,z=u*d-m*_,P=g*f-h*y,V=g*d-m*y,J=h*d-m*f,K=E*J-A*V+T*P+D*z-b*w+N*M;if(K===0)return this.set(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);const B=1/K;return t[0]=(o*J-c*V+l*P)*B,t[1]=(s*V-i*J-r*P)*B,t[2]=(y*N-f*b+d*D)*B,t[3]=(h*b-g*N-m*D)*B,t[4]=(c*z-a*J-l*w)*B,t[5]=(e*J-s*z+r*w)*B,t[6]=(f*T-_*N-d*A)*B,t[7]=(u*N-h*T+m*A)*B,t[8]=(a*V-o*z+l*M)*B,t[9]=(i*z-e*V-r*M)*B,t[10]=(_*b-y*T+d*E)*B,t[11]=(g*T-u*b-m*E)*B,t[12]=(o*w-a*P-c*M)*B,t[13]=(e*P-i*w+s*M)*B,t[14]=(y*A-_*D-f*E)*B,t[15]=(u*D-g*A+h*E)*B,this}scale(t){const e=this.elements,i=t.x,s=t.y,r=t.z;return e[0]*=i,e[4]*=s,e[8]*=r,e[1]*=i,e[5]*=s,e[9]*=r,e[2]*=i,e[6]*=s,e[10]*=r,e[3]*=i,e[7]*=s,e[11]*=r,this}getMaxScaleOnAxis(){const t=this.elements,e=t[0]*t[0]+t[1]*t[1]+t[2]*t[2],i=t[4]*t[4]+t[5]*t[5]+t[6]*t[6],s=t[8]*t[8]+t[9]*t[9]+t[10]*t[10];return Math.sqrt(Math.max(e,i,s))}makeTranslation(t,e,i){return t.isVector3?this.set(1,0,0,t.x,0,1,0,t.y,0,0,1,t.z,0,0,0,1):this.set(1,0,0,t,0,1,0,e,0,0,1,i,0,0,0,1),this}makeRotationX(t){const e=Math.cos(t),i=Math.sin(t);return this.set(1,0,0,0,0,e,-i,0,0,i,e,0,0,0,0,1),this}makeRotationY(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,0,i,0,0,1,0,0,-i,0,e,0,0,0,0,1),this}makeRotationZ(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,-i,0,0,i,e,0,0,0,0,1,0,0,0,0,1),this}makeRotationAxis(t,e){const i=Math.cos(e),s=Math.sin(e),r=1-i,a=t.x,o=t.y,c=t.z,l=r*a,u=r*o;return this.set(l*a+i,l*o-s*c,l*c+s*o,0,l*o+s*c,u*o+i,u*c-s*a,0,l*c-s*o,u*c+s*a,r*c*c+i,0,0,0,0,1),this}makeScale(t,e,i){return this.set(t,0,0,0,0,e,0,0,0,0,i,0,0,0,0,1),this}makeShear(t,e,i,s,r,a){return this.set(1,i,r,0,t,1,a,0,e,s,1,0,0,0,0,1),this}compose(t,e,i){const s=this.elements,r=e._x,a=e._y,o=e._z,c=e._w,l=r+r,u=a+a,g=o+o,h=r*l,m=r*u,_=r*g,y=a*u,f=a*g,d=o*g,E=c*l,A=c*u,T=c*g,D=i.x,b=i.y,N=i.z;return s[0]=(1-(y+d))*D,s[1]=(m+T)*D,s[2]=(_-A)*D,s[3]=0,s[4]=(m-T)*b,s[5]=(1-(h+d))*b,s[6]=(f+E)*b,s[7]=0,s[8]=(_+A)*N,s[9]=(f-E)*N,s[10]=(1-(h+y))*N,s[11]=0,s[12]=t.x,s[13]=t.y,s[14]=t.z,s[15]=1,this}decompose(t,e,i){const s=this.elements;t.x=s[12],t.y=s[13],t.z=s[14];const r=this.determinant();if(r===0)return i.set(1,1,1),e.identity(),this;let a=Gi.set(s[0],s[1],s[2]).length();const o=Gi.set(s[4],s[5],s[6]).length(),c=Gi.set(s[8],s[9],s[10]).length();r<0&&(a=-a),cn.copy(this);const l=1/a,u=1/o,g=1/c;return cn.elements[0]*=l,cn.elements[1]*=l,cn.elements[2]*=l,cn.elements[4]*=u,cn.elements[5]*=u,cn.elements[6]*=u,cn.elements[8]*=g,cn.elements[9]*=g,cn.elements[10]*=g,e.setFromRotationMatrix(cn),i.x=a,i.y=o,i.z=c,this}makePerspective(t,e,i,s,r,a,o=wn,c=!1){const l=this.elements,u=2*r/(e-t),g=2*r/(i-s),h=(e+t)/(e-t),m=(i+s)/(i-s);let _,y;if(c)_=r/(a-r),y=a*r/(a-r);else if(o===wn)_=-(a+r)/(a-r),y=-2*a*r/(a-r);else if(o===ra)_=-a/(a-r),y=-a*r/(a-r);else throw new Error("THREE.Matrix4.makePerspective(): Invalid coordinate system: "+o);return l[0]=u,l[4]=0,l[8]=h,l[12]=0,l[1]=0,l[5]=g,l[9]=m,l[13]=0,l[2]=0,l[6]=0,l[10]=_,l[14]=y,l[3]=0,l[7]=0,l[11]=-1,l[15]=0,this}makeOrthographic(t,e,i,s,r,a,o=wn,c=!1){const l=this.elements,u=2/(e-t),g=2/(i-s),h=-(e+t)/(e-t),m=-(i+s)/(i-s);let _,y;if(c)_=1/(a-r),y=a/(a-r);else if(o===wn)_=-2/(a-r),y=-(a+r)/(a-r);else if(o===ra)_=-1/(a-r),y=-r/(a-r);else throw new Error("THREE.Matrix4.makeOrthographic(): Invalid coordinate system: "+o);return l[0]=u,l[4]=0,l[8]=0,l[12]=h,l[1]=0,l[5]=g,l[9]=0,l[13]=m,l[2]=0,l[6]=0,l[10]=_,l[14]=y,l[3]=0,l[7]=0,l[11]=0,l[15]=1,this}equals(t){const e=this.elements,i=t.elements;for(let s=0;s<16;s++)if(e[s]!==i[s])return!1;return!0}fromArray(t,e=0){for(let i=0;i<16;i++)this.elements[i]=t[i+e];return this}toArray(t=[],e=0){const i=this.elements;return t[e]=i[0],t[e+1]=i[1],t[e+2]=i[2],t[e+3]=i[3],t[e+4]=i[4],t[e+5]=i[5],t[e+6]=i[6],t[e+7]=i[7],t[e+8]=i[8],t[e+9]=i[9],t[e+10]=i[10],t[e+11]=i[11],t[e+12]=i[12],t[e+13]=i[13],t[e+14]=i[14],t[e+15]=i[15],t}}const Gi=new F,cn=new me,Xd=new F(0,0,0),$d=new F(1,1,1),Zn=new F,nr=new F,Je=new F,uc=new me,dc=new hi;class Di{constructor(t=0,e=0,i=0,s=Di.DEFAULT_ORDER){this.isEuler=!0,this._x=t,this._y=e,this._z=i,this._order=s}get x(){return this._x}set x(t){this._x=t,this._onChangeCallback()}get y(){return this._y}set y(t){this._y=t,this._onChangeCallback()}get z(){return this._z}set z(t){this._z=t,this._onChangeCallback()}get order(){return this._order}set order(t){this._order=t,this._onChangeCallback()}set(t,e,i,s=this._order){return this._x=t,this._y=e,this._z=i,this._order=s,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._order)}copy(t){return this._x=t._x,this._y=t._y,this._z=t._z,this._order=t._order,this._onChangeCallback(),this}setFromRotationMatrix(t,e=this._order,i=!0){const s=t.elements,r=s[0],a=s[4],o=s[8],c=s[1],l=s[5],u=s[9],g=s[2],h=s[6],m=s[10];switch(e){case"XYZ":this._y=Math.asin(Yt(o,-1,1)),Math.abs(o)<.9999999?(this._x=Math.atan2(-u,m),this._z=Math.atan2(-a,r)):(this._x=Math.atan2(h,l),this._z=0);break;case"YXZ":this._x=Math.asin(-Yt(u,-1,1)),Math.abs(u)<.9999999?(this._y=Math.atan2(o,m),this._z=Math.atan2(c,l)):(this._y=Math.atan2(-g,r),this._z=0);break;case"ZXY":this._x=Math.asin(Yt(h,-1,1)),Math.abs(h)<.9999999?(this._y=Math.atan2(-g,m),this._z=Math.atan2(-a,l)):(this._y=0,this._z=Math.atan2(c,r));break;case"ZYX":this._y=Math.asin(-Yt(g,-1,1)),Math.abs(g)<.9999999?(this._x=Math.atan2(h,m),this._z=Math.atan2(c,r)):(this._x=0,this._z=Math.atan2(-a,l));break;case"YZX":this._z=Math.asin(Yt(c,-1,1)),Math.abs(c)<.9999999?(this._x=Math.atan2(-u,l),this._y=Math.atan2(-g,r)):(this._x=0,this._y=Math.atan2(o,m));break;case"XZY":this._z=Math.asin(-Yt(a,-1,1)),Math.abs(a)<.9999999?(this._x=Math.atan2(h,l),this._y=Math.atan2(o,r)):(this._x=Math.atan2(-u,m),this._y=0);break;default:It("Euler: .setFromRotationMatrix() encountered an unknown order: "+e)}return this._order=e,i===!0&&this._onChangeCallback(),this}setFromQuaternion(t,e,i){return uc.makeRotationFromQuaternion(t),this.setFromRotationMatrix(uc,e,i)}setFromVector3(t,e=this._order){return this.set(t.x,t.y,t.z,e)}reorder(t){return dc.setFromEuler(this),this.setFromQuaternion(dc,t)}equals(t){return t._x===this._x&&t._y===this._y&&t._z===this._z&&t._order===this._order}fromArray(t){return this._x=t[0],this._y=t[1],this._z=t[2],t[3]!==void 0&&(this._order=t[3]),this._onChangeCallback(),this}toArray(t=[],e=0){return t[e]=this._x,t[e+1]=this._y,t[e+2]=this._z,t[e+3]=this._order,t}_onChange(t){return this._onChangeCallback=t,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._order}}Di.DEFAULT_ORDER="XYZ";class Fh{constructor(){this.mask=1}set(t){this.mask=(1<<t|0)>>>0}enable(t){this.mask|=1<<t|0}enableAll(){this.mask=-1}toggle(t){this.mask^=1<<t|0}disable(t){this.mask&=~(1<<t|0)}disableAll(){this.mask=0}test(t){return(this.mask&t.mask)!==0}isEnabled(t){return(this.mask&(1<<t|0))!==0}}let qd=0;const fc=new F,Vi=new hi,In=new me,ir=new F,Ms=new F,Yd=new F,jd=new hi,pc=new F(1,0,0),mc=new F(0,1,0),gc=new F(0,0,1),_c={type:"added"},Kd={type:"removed"},Wi={type:"childadded",child:null},Ia={type:"childremoved",child:null};class ke extends fi{constructor(){super(),this.isObject3D=!0,Object.defineProperty(this,"id",{value:qd++}),this.uuid=oi(),this.name="",this.type="Object3D",this.parent=null,this.children=[],this.up=ke.DEFAULT_UP.clone();const t=new F,e=new Di,i=new hi,s=new F(1,1,1);function r(){i.setFromEuler(e,!1)}function a(){e.setFromQuaternion(i,void 0,!1)}e._onChange(r),i._onChange(a),Object.defineProperties(this,{position:{configurable:!0,enumerable:!0,value:t},rotation:{configurable:!0,enumerable:!0,value:e},quaternion:{configurable:!0,enumerable:!0,value:i},scale:{configurable:!0,enumerable:!0,value:s},modelViewMatrix:{value:new me},normalMatrix:{value:new zt}}),this.matrix=new me,this.matrixWorld=new me,this.matrixAutoUpdate=ke.DEFAULT_MATRIX_AUTO_UPDATE,this.matrixWorldAutoUpdate=ke.DEFAULT_MATRIX_WORLD_AUTO_UPDATE,this.matrixWorldNeedsUpdate=!1,this.layers=new Fh,this.visible=!0,this.castShadow=!1,this.receiveShadow=!1,this.frustumCulled=!0,this.renderOrder=0,this.animations=[],this.customDepthMaterial=void 0,this.customDistanceMaterial=void 0,this.static=!1,this.userData={},this.pivot=null}onBeforeShadow(){}onAfterShadow(){}onBeforeRender(){}onAfterRender(){}applyMatrix4(t){this.matrixAutoUpdate&&this.updateMatrix(),this.matrix.premultiply(t),this.matrix.decompose(this.position,this.quaternion,this.scale)}applyQuaternion(t){return this.quaternion.premultiply(t),this}setRotationFromAxisAngle(t,e){this.quaternion.setFromAxisAngle(t,e)}setRotationFromEuler(t){this.quaternion.setFromEuler(t,!0)}setRotationFromMatrix(t){this.quaternion.setFromRotationMatrix(t)}setRotationFromQuaternion(t){this.quaternion.copy(t)}rotateOnAxis(t,e){return Vi.setFromAxisAngle(t,e),this.quaternion.multiply(Vi),this}rotateOnWorldAxis(t,e){return Vi.setFromAxisAngle(t,e),this.quaternion.premultiply(Vi),this}rotateX(t){return this.rotateOnAxis(pc,t)}rotateY(t){return this.rotateOnAxis(mc,t)}rotateZ(t){return this.rotateOnAxis(gc,t)}translateOnAxis(t,e){return fc.copy(t).applyQuaternion(this.quaternion),this.position.add(fc.multiplyScalar(e)),this}translateX(t){return this.translateOnAxis(pc,t)}translateY(t){return this.translateOnAxis(mc,t)}translateZ(t){return this.translateOnAxis(gc,t)}localToWorld(t){return this.updateWorldMatrix(!0,!1),t.applyMatrix4(this.matrixWorld)}worldToLocal(t){return this.updateWorldMatrix(!0,!1),t.applyMatrix4(In.copy(this.matrixWorld).invert())}lookAt(t,e,i){t.isVector3?ir.copy(t):ir.set(t,e,i);const s=this.parent;this.updateWorldMatrix(!0,!1),Ms.setFromMatrixPosition(this.matrixWorld),this.isCamera||this.isLight?In.lookAt(Ms,ir,this.up):In.lookAt(ir,Ms,this.up),this.quaternion.setFromRotationMatrix(In),s&&(In.extractRotation(s.matrixWorld),Vi.setFromRotationMatrix(In),this.quaternion.premultiply(Vi.invert()))}add(t){if(arguments.length>1){for(let e=0;e<arguments.length;e++)this.add(arguments[e]);return this}return t===this?(Kt("Object3D.add: object can't be added as a child of itself.",t),this):(t&&t.isObject3D?(t.removeFromParent(),t.parent=this,this.children.push(t),t.dispatchEvent(_c),Wi.child=t,this.dispatchEvent(Wi),Wi.child=null):Kt("Object3D.add: object not an instance of THREE.Object3D.",t),this)}remove(t){if(arguments.length>1){for(let i=0;i<arguments.length;i++)this.remove(arguments[i]);return this}const e=this.children.indexOf(t);return e!==-1&&(t.parent=null,this.children.splice(e,1),t.dispatchEvent(Kd),Ia.child=t,this.dispatchEvent(Ia),Ia.child=null),this}removeFromParent(){const t=this.parent;return t!==null&&t.remove(this),this}clear(){return this.remove(...this.children)}attach(t){return this.updateWorldMatrix(!0,!1),In.copy(this.matrixWorld).invert(),t.parent!==null&&(t.parent.updateWorldMatrix(!0,!1),In.multiply(t.parent.matrixWorld)),t.applyMatrix4(In),t.removeFromParent(),t.parent=this,this.children.push(t),t.updateWorldMatrix(!1,!0),t.dispatchEvent(_c),Wi.child=t,this.dispatchEvent(Wi),Wi.child=null,this}getObjectById(t){return this.getObjectByProperty("id",t)}getObjectByName(t){return this.getObjectByProperty("name",t)}getObjectByProperty(t,e){if(this[t]===e)return this;for(let i=0,s=this.children.length;i<s;i++){const a=this.children[i].getObjectByProperty(t,e);if(a!==void 0)return a}}getObjectsByProperty(t,e,i=[]){this[t]===e&&i.push(this);const s=this.children;for(let r=0,a=s.length;r<a;r++)s[r].getObjectsByProperty(t,e,i);return i}getWorldPosition(t){return this.updateWorldMatrix(!0,!1),t.setFromMatrixPosition(this.matrixWorld)}getWorldQuaternion(t){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Ms,t,Yd),t}getWorldScale(t){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Ms,jd,t),t}getWorldDirection(t){this.updateWorldMatrix(!0,!1);const e=this.matrixWorld.elements;return t.set(e[8],e[9],e[10]).normalize()}raycast(){}traverse(t){t(this);const e=this.children;for(let i=0,s=e.length;i<s;i++)e[i].traverse(t)}traverseVisible(t){if(this.visible===!1)return;t(this);const e=this.children;for(let i=0,s=e.length;i<s;i++)e[i].traverseVisible(t)}traverseAncestors(t){const e=this.parent;e!==null&&(t(e),e.traverseAncestors(t))}updateMatrix(){this.matrix.compose(this.position,this.quaternion,this.scale);const t=this.pivot;if(t!==null){const e=t.x,i=t.y,s=t.z,r=this.matrix.elements;r[12]+=e-r[0]*e-r[4]*i-r[8]*s,r[13]+=i-r[1]*e-r[5]*i-r[9]*s,r[14]+=s-r[2]*e-r[6]*i-r[10]*s}this.matrixWorldNeedsUpdate=!0}updateMatrixWorld(t){this.matrixAutoUpdate&&this.updateMatrix(),(this.matrixWorldNeedsUpdate||t)&&(this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),this.matrixWorldNeedsUpdate=!1,t=!0);const e=this.children;for(let i=0,s=e.length;i<s;i++)e[i].updateMatrixWorld(t)}updateWorldMatrix(t,e){const i=this.parent;if(t===!0&&i!==null&&i.updateWorldMatrix(!0,!1),this.matrixAutoUpdate&&this.updateMatrix(),this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),e===!0){const s=this.children;for(let r=0,a=s.length;r<a;r++)s[r].updateWorldMatrix(!1,!0)}}toJSON(t){const e=t===void 0||typeof t=="string",i={};e&&(t={geometries:{},materials:{},textures:{},images:{},shapes:{},skeletons:{},animations:{},nodes:{}},i.metadata={version:4.7,type:"Object",generator:"Object3D.toJSON"});const s={};s.uuid=this.uuid,s.type=this.type,this.name!==""&&(s.name=this.name),this.castShadow===!0&&(s.castShadow=!0),this.receiveShadow===!0&&(s.receiveShadow=!0),this.visible===!1&&(s.visible=!1),this.frustumCulled===!1&&(s.frustumCulled=!1),this.renderOrder!==0&&(s.renderOrder=this.renderOrder),this.static!==!1&&(s.static=this.static),Object.keys(this.userData).length>0&&(s.userData=this.userData),s.layers=this.layers.mask,s.matrix=this.matrix.toArray(),s.up=this.up.toArray(),this.pivot!==null&&(s.pivot=this.pivot.toArray()),this.matrixAutoUpdate===!1&&(s.matrixAutoUpdate=!1),this.morphTargetDictionary!==void 0&&(s.morphTargetDictionary=Object.assign({},this.morphTargetDictionary)),this.morphTargetInfluences!==void 0&&(s.morphTargetInfluences=this.morphTargetInfluences.slice()),this.isInstancedMesh&&(s.type="InstancedMesh",s.count=this.count,s.instanceMatrix=this.instanceMatrix.toJSON(),this.instanceColor!==null&&(s.instanceColor=this.instanceColor.toJSON())),this.isBatchedMesh&&(s.type="BatchedMesh",s.perObjectFrustumCulled=this.perObjectFrustumCulled,s.sortObjects=this.sortObjects,s.drawRanges=this._drawRanges,s.reservedRanges=this._reservedRanges,s.geometryInfo=this._geometryInfo.map(o=>({...o,boundingBox:o.boundingBox?o.boundingBox.toJSON():void 0,boundingSphere:o.boundingSphere?o.boundingSphere.toJSON():void 0})),s.instanceInfo=this._instanceInfo.map(o=>({...o})),s.availableInstanceIds=this._availableInstanceIds.slice(),s.availableGeometryIds=this._availableGeometryIds.slice(),s.nextIndexStart=this._nextIndexStart,s.nextVertexStart=this._nextVertexStart,s.geometryCount=this._geometryCount,s.maxInstanceCount=this._maxInstanceCount,s.maxVertexCount=this._maxVertexCount,s.maxIndexCount=this._maxIndexCount,s.geometryInitialized=this._geometryInitialized,s.matricesTexture=this._matricesTexture.toJSON(t),s.indirectTexture=this._indirectTexture.toJSON(t),this._colorsTexture!==null&&(s.colorsTexture=this._colorsTexture.toJSON(t)),this.boundingSphere!==null&&(s.boundingSphere=this.boundingSphere.toJSON()),this.boundingBox!==null&&(s.boundingBox=this.boundingBox.toJSON()));function r(o,c){return o[c.uuid]===void 0&&(o[c.uuid]=c.toJSON(t)),c.uuid}if(this.isScene)this.background&&(this.background.isColor?s.background=this.background.toJSON():this.background.isTexture&&(s.background=this.background.toJSON(t).uuid)),this.environment&&this.environment.isTexture&&this.environment.isRenderTargetTexture!==!0&&(s.environment=this.environment.toJSON(t).uuid);else if(this.isMesh||this.isLine||this.isPoints){s.geometry=r(t.geometries,this.geometry);const o=this.geometry.parameters;if(o!==void 0&&o.shapes!==void 0){const c=o.shapes;if(Array.isArray(c))for(let l=0,u=c.length;l<u;l++){const g=c[l];r(t.shapes,g)}else r(t.shapes,c)}}if(this.isSkinnedMesh&&(s.bindMode=this.bindMode,s.bindMatrix=this.bindMatrix.toArray(),this.skeleton!==void 0&&(r(t.skeletons,this.skeleton),s.skeleton=this.skeleton.uuid)),this.material!==void 0)if(Array.isArray(this.material)){const o=[];for(let c=0,l=this.material.length;c<l;c++)o.push(r(t.materials,this.material[c]));s.material=o}else s.material=r(t.materials,this.material);if(this.children.length>0){s.children=[];for(let o=0;o<this.children.length;o++)s.children.push(this.children[o].toJSON(t).object)}if(this.animations.length>0){s.animations=[];for(let o=0;o<this.animations.length;o++){const c=this.animations[o];s.animations.push(r(t.animations,c))}}if(e){const o=a(t.geometries),c=a(t.materials),l=a(t.textures),u=a(t.images),g=a(t.shapes),h=a(t.skeletons),m=a(t.animations),_=a(t.nodes);o.length>0&&(i.geometries=o),c.length>0&&(i.materials=c),l.length>0&&(i.textures=l),u.length>0&&(i.images=u),g.length>0&&(i.shapes=g),h.length>0&&(i.skeletons=h),m.length>0&&(i.animations=m),_.length>0&&(i.nodes=_)}return i.object=s,i;function a(o){const c=[];for(const l in o){const u=o[l];delete u.metadata,c.push(u)}return c}}clone(t){return new this.constructor().copy(this,t)}copy(t,e=!0){if(this.name=t.name,this.up.copy(t.up),this.position.copy(t.position),this.rotation.order=t.rotation.order,this.quaternion.copy(t.quaternion),this.scale.copy(t.scale),this.pivot=t.pivot!==null?t.pivot.clone():null,this.matrix.copy(t.matrix),this.matrixWorld.copy(t.matrixWorld),this.matrixAutoUpdate=t.matrixAutoUpdate,this.matrixWorldAutoUpdate=t.matrixWorldAutoUpdate,this.matrixWorldNeedsUpdate=t.matrixWorldNeedsUpdate,this.layers.mask=t.layers.mask,this.visible=t.visible,this.castShadow=t.castShadow,this.receiveShadow=t.receiveShadow,this.frustumCulled=t.frustumCulled,this.renderOrder=t.renderOrder,this.static=t.static,this.animations=t.animations.slice(),this.userData=JSON.parse(JSON.stringify(t.userData)),e===!0)for(let i=0;i<t.children.length;i++){const s=t.children[i];this.add(s.clone())}return this}}ke.DEFAULT_UP=new F(0,1,0);ke.DEFAULT_MATRIX_AUTO_UPDATE=!0;ke.DEFAULT_MATRIX_WORLD_AUTO_UPDATE=!0;class sr extends ke{constructor(){super(),this.isGroup=!0,this.type="Group"}}const Zd={type:"move"};class Ua{constructor(){this._targetRay=null,this._grip=null,this._hand=null}getHandSpace(){return this._hand===null&&(this._hand=new sr,this._hand.matrixAutoUpdate=!1,this._hand.visible=!1,this._hand.joints={},this._hand.inputState={pinching:!1}),this._hand}getTargetRaySpace(){return this._targetRay===null&&(this._targetRay=new sr,this._targetRay.matrixAutoUpdate=!1,this._targetRay.visible=!1,this._targetRay.hasLinearVelocity=!1,this._targetRay.linearVelocity=new F,this._targetRay.hasAngularVelocity=!1,this._targetRay.angularVelocity=new F),this._targetRay}getGripSpace(){return this._grip===null&&(this._grip=new sr,this._grip.matrixAutoUpdate=!1,this._grip.visible=!1,this._grip.hasLinearVelocity=!1,this._grip.linearVelocity=new F,this._grip.hasAngularVelocity=!1,this._grip.angularVelocity=new F,this._grip.eventsEnabled=!1),this._grip}dispatchEvent(t){return this._targetRay!==null&&this._targetRay.dispatchEvent(t),this._grip!==null&&this._grip.dispatchEvent(t),this._hand!==null&&this._hand.dispatchEvent(t),this}connect(t){if(t&&t.hand){const e=this._hand;if(e)for(const i of t.hand.values())this._getHandJoint(e,i)}return this.dispatchEvent({type:"connected",data:t}),this}disconnect(t){return this.dispatchEvent({type:"disconnected",data:t}),this._targetRay!==null&&(this._targetRay.visible=!1),this._grip!==null&&(this._grip.visible=!1),this._hand!==null&&(this._hand.visible=!1),this}update(t,e,i){let s=null,r=null,a=null;const o=this._targetRay,c=this._grip,l=this._hand;if(t&&e.session.visibilityState!=="visible-blurred"){if(l&&t.hand){a=!0;for(const y of t.hand.values()){const f=e.getJointPose(y,i),d=this._getHandJoint(l,y);f!==null&&(d.matrix.fromArray(f.transform.matrix),d.matrix.decompose(d.position,d.rotation,d.scale),d.matrixWorldNeedsUpdate=!0,d.jointRadius=f.radius),d.visible=f!==null}const u=l.joints["index-finger-tip"],g=l.joints["thumb-tip"],h=u.position.distanceTo(g.position),m=.02,_=.005;l.inputState.pinching&&h>m+_?(l.inputState.pinching=!1,this.dispatchEvent({type:"pinchend",handedness:t.handedness,target:this})):!l.inputState.pinching&&h<=m-_&&(l.inputState.pinching=!0,this.dispatchEvent({type:"pinchstart",handedness:t.handedness,target:this}))}else c!==null&&t.gripSpace&&(r=e.getPose(t.gripSpace,i),r!==null&&(c.matrix.fromArray(r.transform.matrix),c.matrix.decompose(c.position,c.rotation,c.scale),c.matrixWorldNeedsUpdate=!0,r.linearVelocity?(c.hasLinearVelocity=!0,c.linearVelocity.copy(r.linearVelocity)):c.hasLinearVelocity=!1,r.angularVelocity?(c.hasAngularVelocity=!0,c.angularVelocity.copy(r.angularVelocity)):c.hasAngularVelocity=!1,c.eventsEnabled&&c.dispatchEvent({type:"gripUpdated",data:t,target:this})));o!==null&&(s=e.getPose(t.targetRaySpace,i),s===null&&r!==null&&(s=r),s!==null&&(o.matrix.fromArray(s.transform.matrix),o.matrix.decompose(o.position,o.rotation,o.scale),o.matrixWorldNeedsUpdate=!0,s.linearVelocity?(o.hasLinearVelocity=!0,o.linearVelocity.copy(s.linearVelocity)):o.hasLinearVelocity=!1,s.angularVelocity?(o.hasAngularVelocity=!0,o.angularVelocity.copy(s.angularVelocity)):o.hasAngularVelocity=!1,this.dispatchEvent(Zd)))}return o!==null&&(o.visible=s!==null),c!==null&&(c.visible=r!==null),l!==null&&(l.visible=a!==null),this}_getHandJoint(t,e){if(t.joints[e.jointName]===void 0){const i=new sr;i.matrixAutoUpdate=!1,i.visible=!1,t.joints[e.jointName]=i,t.add(i)}return t.joints[e.jointName]}}const Oh={aliceblue:15792383,antiquewhite:16444375,aqua:65535,aquamarine:8388564,azure:15794175,beige:16119260,bisque:16770244,black:0,blanchedalmond:16772045,blue:255,blueviolet:9055202,brown:10824234,burlywood:14596231,cadetblue:6266528,chartreuse:8388352,chocolate:13789470,coral:16744272,cornflowerblue:6591981,cornsilk:16775388,crimson:14423100,cyan:65535,darkblue:139,darkcyan:35723,darkgoldenrod:12092939,darkgray:11119017,darkgreen:25600,darkgrey:11119017,darkkhaki:12433259,darkmagenta:9109643,darkolivegreen:5597999,darkorange:16747520,darkorchid:10040012,darkred:9109504,darksalmon:15308410,darkseagreen:9419919,darkslateblue:4734347,darkslategray:3100495,darkslategrey:3100495,darkturquoise:52945,darkviolet:9699539,deeppink:16716947,deepskyblue:49151,dimgray:6908265,dimgrey:6908265,dodgerblue:2003199,firebrick:11674146,floralwhite:16775920,forestgreen:2263842,fuchsia:16711935,gainsboro:14474460,ghostwhite:16316671,gold:16766720,goldenrod:14329120,gray:8421504,green:32768,greenyellow:11403055,grey:8421504,honeydew:15794160,hotpink:16738740,indianred:13458524,indigo:4915330,ivory:16777200,khaki:15787660,lavender:15132410,lavenderblush:16773365,lawngreen:8190976,lemonchiffon:16775885,lightblue:11393254,lightcoral:15761536,lightcyan:14745599,lightgoldenrodyellow:16448210,lightgray:13882323,lightgreen:9498256,lightgrey:13882323,lightpink:16758465,lightsalmon:16752762,lightseagreen:2142890,lightskyblue:8900346,lightslategray:7833753,lightslategrey:7833753,lightsteelblue:11584734,lightyellow:16777184,lime:65280,limegreen:3329330,linen:16445670,magenta:16711935,maroon:8388608,mediumaquamarine:6737322,mediumblue:205,mediumorchid:12211667,mediumpurple:9662683,mediumseagreen:3978097,mediumslateblue:8087790,mediumspringgreen:64154,mediumturquoise:4772300,mediumvioletred:13047173,midnightblue:1644912,mintcream:16121850,mistyrose:16770273,moccasin:16770229,navajowhite:16768685,navy:128,oldlace:16643558,olive:8421376,olivedrab:7048739,orange:16753920,orangered:16729344,orchid:14315734,palegoldenrod:15657130,palegreen:10025880,paleturquoise:11529966,palevioletred:14381203,papayawhip:16773077,peachpuff:16767673,peru:13468991,pink:16761035,plum:14524637,powderblue:11591910,purple:8388736,rebeccapurple:6697881,red:16711680,rosybrown:12357519,royalblue:4286945,saddlebrown:9127187,salmon:16416882,sandybrown:16032864,seagreen:3050327,seashell:16774638,sienna:10506797,silver:12632256,skyblue:8900331,slateblue:6970061,slategray:7372944,slategrey:7372944,snow:16775930,springgreen:65407,steelblue:4620980,tan:13808780,teal:32896,thistle:14204888,tomato:16737095,turquoise:4251856,violet:15631086,wheat:16113331,white:16777215,whitesmoke:16119285,yellow:16776960,yellowgreen:10145074},Jn={h:0,s:0,l:0},rr={h:0,s:0,l:0};function Na(n,t,e){return e<0&&(e+=1),e>1&&(e-=1),e<1/6?n+(t-n)*6*e:e<1/2?t:e<2/3?n+(t-n)*6*(2/3-e):n}class Bt{constructor(t,e,i){return this.isColor=!0,this.r=1,this.g=1,this.b=1,this.set(t,e,i)}set(t,e,i){if(e===void 0&&i===void 0){const s=t;s&&s.isColor?this.copy(s):typeof s=="number"?this.setHex(s):typeof s=="string"&&this.setStyle(s)}else this.setRGB(t,e,i);return this}setScalar(t){return this.r=t,this.g=t,this.b=t,this}setHex(t,e=sn){return t=Math.floor(t),this.r=(t>>16&255)/255,this.g=(t>>8&255)/255,this.b=(t&255)/255,jt.colorSpaceToWorking(this,e),this}setRGB(t,e,i,s=jt.workingColorSpace){return this.r=t,this.g=e,this.b=i,jt.colorSpaceToWorking(this,s),this}setHSL(t,e,i,s=jt.workingColorSpace){if(t=Od(t,1),e=Yt(e,0,1),i=Yt(i,0,1),e===0)this.r=this.g=this.b=i;else{const r=i<=.5?i*(1+e):i+e-i*e,a=2*i-r;this.r=Na(a,r,t+1/3),this.g=Na(a,r,t),this.b=Na(a,r,t-1/3)}return jt.colorSpaceToWorking(this,s),this}setStyle(t,e=sn){function i(r){r!==void 0&&parseFloat(r)<1&&It("Color: Alpha component of "+t+" will be ignored.")}let s;if(s=/^(\w+)\(([^\)]*)\)/.exec(t)){let r;const a=s[1],o=s[2];switch(a){case"rgb":case"rgba":if(r=/^\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return i(r[4]),this.setRGB(Math.min(255,parseInt(r[1],10))/255,Math.min(255,parseInt(r[2],10))/255,Math.min(255,parseInt(r[3],10))/255,e);if(r=/^\s*(\d+)\%\s*,\s*(\d+)\%\s*,\s*(\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return i(r[4]),this.setRGB(Math.min(100,parseInt(r[1],10))/100,Math.min(100,parseInt(r[2],10))/100,Math.min(100,parseInt(r[3],10))/100,e);break;case"hsl":case"hsla":if(r=/^\s*(\d*\.?\d+)\s*,\s*(\d*\.?\d+)\%\s*,\s*(\d*\.?\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return i(r[4]),this.setHSL(parseFloat(r[1])/360,parseFloat(r[2])/100,parseFloat(r[3])/100,e);break;default:It("Color: Unknown color model "+t)}}else if(s=/^\#([A-Fa-f\d]+)$/.exec(t)){const r=s[1],a=r.length;if(a===3)return this.setRGB(parseInt(r.charAt(0),16)/15,parseInt(r.charAt(1),16)/15,parseInt(r.charAt(2),16)/15,e);if(a===6)return this.setHex(parseInt(r,16),e);It("Color: Invalid hex color "+t)}else if(t&&t.length>0)return this.setColorName(t,e);return this}setColorName(t,e=sn){const i=Oh[t.toLowerCase()];return i!==void 0?this.setHex(i,e):It("Color: Unknown color "+t),this}clone(){return new this.constructor(this.r,this.g,this.b)}copy(t){return this.r=t.r,this.g=t.g,this.b=t.b,this}copySRGBToLinear(t){return this.r=Gn(t.r),this.g=Gn(t.g),this.b=Gn(t.b),this}copyLinearToSRGB(t){return this.r=ds(t.r),this.g=ds(t.g),this.b=ds(t.b),this}convertSRGBToLinear(){return this.copySRGBToLinear(this),this}convertLinearToSRGB(){return this.copyLinearToSRGB(this),this}getHex(t=sn){return jt.workingToColorSpace(Oe.copy(this),t),Math.round(Yt(Oe.r*255,0,255))*65536+Math.round(Yt(Oe.g*255,0,255))*256+Math.round(Yt(Oe.b*255,0,255))}getHexString(t=sn){return("000000"+this.getHex(t).toString(16)).slice(-6)}getHSL(t,e=jt.workingColorSpace){jt.workingToColorSpace(Oe.copy(this),e);const i=Oe.r,s=Oe.g,r=Oe.b,a=Math.max(i,s,r),o=Math.min(i,s,r);let c,l;const u=(o+a)/2;if(o===a)c=0,l=0;else{const g=a-o;switch(l=u<=.5?g/(a+o):g/(2-a-o),a){case i:c=(s-r)/g+(s<r?6:0);break;case s:c=(r-i)/g+2;break;case r:c=(i-s)/g+4;break}c/=6}return t.h=c,t.s=l,t.l=u,t}getRGB(t,e=jt.workingColorSpace){return jt.workingToColorSpace(Oe.copy(this),e),t.r=Oe.r,t.g=Oe.g,t.b=Oe.b,t}getStyle(t=sn){jt.workingToColorSpace(Oe.copy(this),t);const e=Oe.r,i=Oe.g,s=Oe.b;return t!==sn?`color(${t} ${e.toFixed(3)} ${i.toFixed(3)} ${s.toFixed(3)})`:`rgb(${Math.round(e*255)},${Math.round(i*255)},${Math.round(s*255)})`}offsetHSL(t,e,i){return this.getHSL(Jn),this.setHSL(Jn.h+t,Jn.s+e,Jn.l+i)}add(t){return this.r+=t.r,this.g+=t.g,this.b+=t.b,this}addColors(t,e){return this.r=t.r+e.r,this.g=t.g+e.g,this.b=t.b+e.b,this}addScalar(t){return this.r+=t,this.g+=t,this.b+=t,this}sub(t){return this.r=Math.max(0,this.r-t.r),this.g=Math.max(0,this.g-t.g),this.b=Math.max(0,this.b-t.b),this}multiply(t){return this.r*=t.r,this.g*=t.g,this.b*=t.b,this}multiplyScalar(t){return this.r*=t,this.g*=t,this.b*=t,this}lerp(t,e){return this.r+=(t.r-this.r)*e,this.g+=(t.g-this.g)*e,this.b+=(t.b-this.b)*e,this}lerpColors(t,e,i){return this.r=t.r+(e.r-t.r)*i,this.g=t.g+(e.g-t.g)*i,this.b=t.b+(e.b-t.b)*i,this}lerpHSL(t,e){this.getHSL(Jn),t.getHSL(rr);const i=Ra(Jn.h,rr.h,e),s=Ra(Jn.s,rr.s,e),r=Ra(Jn.l,rr.l,e);return this.setHSL(i,s,r),this}setFromVector3(t){return this.r=t.x,this.g=t.y,this.b=t.z,this}applyMatrix3(t){const e=this.r,i=this.g,s=this.b,r=t.elements;return this.r=r[0]*e+r[3]*i+r[6]*s,this.g=r[1]*e+r[4]*i+r[7]*s,this.b=r[2]*e+r[5]*i+r[8]*s,this}equals(t){return t.r===this.r&&t.g===this.g&&t.b===this.b}fromArray(t,e=0){return this.r=t[e],this.g=t[e+1],this.b=t[e+2],this}toArray(t=[],e=0){return t[e]=this.r,t[e+1]=this.g,t[e+2]=this.b,t}fromBufferAttribute(t,e){return this.r=t.getX(e),this.g=t.getY(e),this.b=t.getZ(e),this}toJSON(){return this.getHex()}*[Symbol.iterator](){yield this.r,yield this.g,yield this.b}}const Oe=new Bt;Bt.NAMES=Oh;class Al{constructor(t,e=25e-5){this.isFogExp2=!0,this.name="",this.color=new Bt(t),this.density=e}clone(){return new Al(this.color,this.density)}toJSON(){return{type:"FogExp2",name:this.name,color:this.color.getHex(),density:this.density}}}class Jd extends ke{constructor(){super(),this.isScene=!0,this.type="Scene",this.background=null,this.environment=null,this.fog=null,this.backgroundBlurriness=0,this.backgroundIntensity=1,this.backgroundRotation=new Di,this.environmentIntensity=1,this.environmentRotation=new Di,this.overrideMaterial=null,typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}copy(t,e){return super.copy(t,e),t.background!==null&&(this.background=t.background.clone()),t.environment!==null&&(this.environment=t.environment.clone()),t.fog!==null&&(this.fog=t.fog.clone()),this.backgroundBlurriness=t.backgroundBlurriness,this.backgroundIntensity=t.backgroundIntensity,this.backgroundRotation.copy(t.backgroundRotation),this.environmentIntensity=t.environmentIntensity,this.environmentRotation.copy(t.environmentRotation),t.overrideMaterial!==null&&(this.overrideMaterial=t.overrideMaterial.clone()),this.matrixAutoUpdate=t.matrixAutoUpdate,this}toJSON(t){const e=super.toJSON(t);return this.fog!==null&&(e.object.fog=this.fog.toJSON()),this.backgroundBlurriness>0&&(e.object.backgroundBlurriness=this.backgroundBlurriness),this.backgroundIntensity!==1&&(e.object.backgroundIntensity=this.backgroundIntensity),e.object.backgroundRotation=this.backgroundRotation.toArray(),this.environmentIntensity!==1&&(e.object.environmentIntensity=this.environmentIntensity),e.object.environmentRotation=this.environmentRotation.toArray(),e}}const hn=new F,Un=new F,Fa=new F,Nn=new F,Xi=new F,$i=new F,xc=new F,Oa=new F,Ba=new F,za=new F,ka=new ye,Ha=new ye,Ga=new ye;class on{constructor(t=new F,e=new F,i=new F){this.a=t,this.b=e,this.c=i}static getNormal(t,e,i,s){s.subVectors(i,e),hn.subVectors(t,e),s.cross(hn);const r=s.lengthSq();return r>0?s.multiplyScalar(1/Math.sqrt(r)):s.set(0,0,0)}static getBarycoord(t,e,i,s,r){hn.subVectors(s,e),Un.subVectors(i,e),Fa.subVectors(t,e);const a=hn.dot(hn),o=hn.dot(Un),c=hn.dot(Fa),l=Un.dot(Un),u=Un.dot(Fa),g=a*l-o*o;if(g===0)return r.set(0,0,0),null;const h=1/g,m=(l*c-o*u)*h,_=(a*u-o*c)*h;return r.set(1-m-_,_,m)}static containsPoint(t,e,i,s){return this.getBarycoord(t,e,i,s,Nn)===null?!1:Nn.x>=0&&Nn.y>=0&&Nn.x+Nn.y<=1}static getInterpolation(t,e,i,s,r,a,o,c){return this.getBarycoord(t,e,i,s,Nn)===null?(c.x=0,c.y=0,"z"in c&&(c.z=0),"w"in c&&(c.w=0),null):(c.setScalar(0),c.addScaledVector(r,Nn.x),c.addScaledVector(a,Nn.y),c.addScaledVector(o,Nn.z),c)}static getInterpolatedAttribute(t,e,i,s,r,a){return ka.setScalar(0),Ha.setScalar(0),Ga.setScalar(0),ka.fromBufferAttribute(t,e),Ha.fromBufferAttribute(t,i),Ga.fromBufferAttribute(t,s),a.setScalar(0),a.addScaledVector(ka,r.x),a.addScaledVector(Ha,r.y),a.addScaledVector(Ga,r.z),a}static isFrontFacing(t,e,i,s){return hn.subVectors(i,e),Un.subVectors(t,e),hn.cross(Un).dot(s)<0}set(t,e,i){return this.a.copy(t),this.b.copy(e),this.c.copy(i),this}setFromPointsAndIndices(t,e,i,s){return this.a.copy(t[e]),this.b.copy(t[i]),this.c.copy(t[s]),this}setFromAttributeAndIndices(t,e,i,s){return this.a.fromBufferAttribute(t,e),this.b.fromBufferAttribute(t,i),this.c.fromBufferAttribute(t,s),this}clone(){return new this.constructor().copy(this)}copy(t){return this.a.copy(t.a),this.b.copy(t.b),this.c.copy(t.c),this}getArea(){return hn.subVectors(this.c,this.b),Un.subVectors(this.a,this.b),hn.cross(Un).length()*.5}getMidpoint(t){return t.addVectors(this.a,this.b).add(this.c).multiplyScalar(1/3)}getNormal(t){return on.getNormal(this.a,this.b,this.c,t)}getPlane(t){return t.setFromCoplanarPoints(this.a,this.b,this.c)}getBarycoord(t,e){return on.getBarycoord(t,this.a,this.b,this.c,e)}getInterpolation(t,e,i,s,r){return on.getInterpolation(t,this.a,this.b,this.c,e,i,s,r)}containsPoint(t){return on.containsPoint(t,this.a,this.b,this.c)}isFrontFacing(t){return on.isFrontFacing(this.a,this.b,this.c,t)}intersectsBox(t){return t.intersectsTriangle(this)}closestPointToPoint(t,e){const i=this.a,s=this.b,r=this.c;let a,o;Xi.subVectors(s,i),$i.subVectors(r,i),Oa.subVectors(t,i);const c=Xi.dot(Oa),l=$i.dot(Oa);if(c<=0&&l<=0)return e.copy(i);Ba.subVectors(t,s);const u=Xi.dot(Ba),g=$i.dot(Ba);if(u>=0&&g<=u)return e.copy(s);const h=c*g-u*l;if(h<=0&&c>=0&&u<=0)return a=c/(c-u),e.copy(i).addScaledVector(Xi,a);za.subVectors(t,r);const m=Xi.dot(za),_=$i.dot(za);if(_>=0&&m<=_)return e.copy(r);const y=m*l-c*_;if(y<=0&&l>=0&&_<=0)return o=l/(l-_),e.copy(i).addScaledVector($i,o);const f=u*_-m*g;if(f<=0&&g-u>=0&&m-_>=0)return xc.subVectors(r,s),o=(g-u)/(g-u+(m-_)),e.copy(s).addScaledVector(xc,o);const d=1/(f+y+h);return a=y*d,o=h*d,e.copy(i).addScaledVector(Xi,a).addScaledVector($i,o)}equals(t){return t.a.equals(this.a)&&t.b.equals(this.b)&&t.c.equals(this.c)}}class Ys{constructor(t=new F(1/0,1/0,1/0),e=new F(-1/0,-1/0,-1/0)){this.isBox3=!0,this.min=t,this.max=e}set(t,e){return this.min.copy(t),this.max.copy(e),this}setFromArray(t){this.makeEmpty();for(let e=0,i=t.length;e<i;e+=3)this.expandByPoint(un.fromArray(t,e));return this}setFromBufferAttribute(t){this.makeEmpty();for(let e=0,i=t.count;e<i;e++)this.expandByPoint(un.fromBufferAttribute(t,e));return this}setFromPoints(t){this.makeEmpty();for(let e=0,i=t.length;e<i;e++)this.expandByPoint(t[e]);return this}setFromCenterAndSize(t,e){const i=un.copy(e).multiplyScalar(.5);return this.min.copy(t).sub(i),this.max.copy(t).add(i),this}setFromObject(t,e=!1){return this.makeEmpty(),this.expandByObject(t,e)}clone(){return new this.constructor().copy(this)}copy(t){return this.min.copy(t.min),this.max.copy(t.max),this}makeEmpty(){return this.min.x=this.min.y=this.min.z=1/0,this.max.x=this.max.y=this.max.z=-1/0,this}isEmpty(){return this.max.x<this.min.x||this.max.y<this.min.y||this.max.z<this.min.z}getCenter(t){return this.isEmpty()?t.set(0,0,0):t.addVectors(this.min,this.max).multiplyScalar(.5)}getSize(t){return this.isEmpty()?t.set(0,0,0):t.subVectors(this.max,this.min)}expandByPoint(t){return this.min.min(t),this.max.max(t),this}expandByVector(t){return this.min.sub(t),this.max.add(t),this}expandByScalar(t){return this.min.addScalar(-t),this.max.addScalar(t),this}expandByObject(t,e=!1){t.updateWorldMatrix(!1,!1);const i=t.geometry;if(i!==void 0){const r=i.getAttribute("position");if(e===!0&&r!==void 0&&t.isInstancedMesh!==!0)for(let a=0,o=r.count;a<o;a++)t.isMesh===!0?t.getVertexPosition(a,un):un.fromBufferAttribute(r,a),un.applyMatrix4(t.matrixWorld),this.expandByPoint(un);else t.boundingBox!==void 0?(t.boundingBox===null&&t.computeBoundingBox(),ar.copy(t.boundingBox)):(i.boundingBox===null&&i.computeBoundingBox(),ar.copy(i.boundingBox)),ar.applyMatrix4(t.matrixWorld),this.union(ar)}const s=t.children;for(let r=0,a=s.length;r<a;r++)this.expandByObject(s[r],e);return this}containsPoint(t){return t.x>=this.min.x&&t.x<=this.max.x&&t.y>=this.min.y&&t.y<=this.max.y&&t.z>=this.min.z&&t.z<=this.max.z}containsBox(t){return this.min.x<=t.min.x&&t.max.x<=this.max.x&&this.min.y<=t.min.y&&t.max.y<=this.max.y&&this.min.z<=t.min.z&&t.max.z<=this.max.z}getParameter(t,e){return e.set((t.x-this.min.x)/(this.max.x-this.min.x),(t.y-this.min.y)/(this.max.y-this.min.y),(t.z-this.min.z)/(this.max.z-this.min.z))}intersectsBox(t){return t.max.x>=this.min.x&&t.min.x<=this.max.x&&t.max.y>=this.min.y&&t.min.y<=this.max.y&&t.max.z>=this.min.z&&t.min.z<=this.max.z}intersectsSphere(t){return this.clampPoint(t.center,un),un.distanceToSquared(t.center)<=t.radius*t.radius}intersectsPlane(t){let e,i;return t.normal.x>0?(e=t.normal.x*this.min.x,i=t.normal.x*this.max.x):(e=t.normal.x*this.max.x,i=t.normal.x*this.min.x),t.normal.y>0?(e+=t.normal.y*this.min.y,i+=t.normal.y*this.max.y):(e+=t.normal.y*this.max.y,i+=t.normal.y*this.min.y),t.normal.z>0?(e+=t.normal.z*this.min.z,i+=t.normal.z*this.max.z):(e+=t.normal.z*this.max.z,i+=t.normal.z*this.min.z),e<=-t.constant&&i>=-t.constant}intersectsTriangle(t){if(this.isEmpty())return!1;this.getCenter(Ss),or.subVectors(this.max,Ss),qi.subVectors(t.a,Ss),Yi.subVectors(t.b,Ss),ji.subVectors(t.c,Ss),Qn.subVectors(Yi,qi),ti.subVectors(ji,Yi),gi.subVectors(qi,ji);let e=[0,-Qn.z,Qn.y,0,-ti.z,ti.y,0,-gi.z,gi.y,Qn.z,0,-Qn.x,ti.z,0,-ti.x,gi.z,0,-gi.x,-Qn.y,Qn.x,0,-ti.y,ti.x,0,-gi.y,gi.x,0];return!Va(e,qi,Yi,ji,or)||(e=[1,0,0,0,1,0,0,0,1],!Va(e,qi,Yi,ji,or))?!1:(lr.crossVectors(Qn,ti),e=[lr.x,lr.y,lr.z],Va(e,qi,Yi,ji,or))}clampPoint(t,e){return e.copy(t).clamp(this.min,this.max)}distanceToPoint(t){return this.clampPoint(t,un).distanceTo(t)}getBoundingSphere(t){return this.isEmpty()?t.makeEmpty():(this.getCenter(t.center),t.radius=this.getSize(un).length()*.5),t}intersect(t){return this.min.max(t.min),this.max.min(t.max),this.isEmpty()&&this.makeEmpty(),this}union(t){return this.min.min(t.min),this.max.max(t.max),this}applyMatrix4(t){return this.isEmpty()?this:(Fn[0].set(this.min.x,this.min.y,this.min.z).applyMatrix4(t),Fn[1].set(this.min.x,this.min.y,this.max.z).applyMatrix4(t),Fn[2].set(this.min.x,this.max.y,this.min.z).applyMatrix4(t),Fn[3].set(this.min.x,this.max.y,this.max.z).applyMatrix4(t),Fn[4].set(this.max.x,this.min.y,this.min.z).applyMatrix4(t),Fn[5].set(this.max.x,this.min.y,this.max.z).applyMatrix4(t),Fn[6].set(this.max.x,this.max.y,this.min.z).applyMatrix4(t),Fn[7].set(this.max.x,this.max.y,this.max.z).applyMatrix4(t),this.setFromPoints(Fn),this)}translate(t){return this.min.add(t),this.max.add(t),this}equals(t){return t.min.equals(this.min)&&t.max.equals(this.max)}toJSON(){return{min:this.min.toArray(),max:this.max.toArray()}}fromJSON(t){return this.min.fromArray(t.min),this.max.fromArray(t.max),this}}const Fn=[new F,new F,new F,new F,new F,new F,new F,new F],un=new F,ar=new Ys,qi=new F,Yi=new F,ji=new F,Qn=new F,ti=new F,gi=new F,Ss=new F,or=new F,lr=new F,_i=new F;function Va(n,t,e,i,s){for(let r=0,a=n.length-3;r<=a;r+=3){_i.fromArray(n,r);const o=s.x*Math.abs(_i.x)+s.y*Math.abs(_i.y)+s.z*Math.abs(_i.z),c=t.dot(_i),l=e.dot(_i),u=i.dot(_i);if(Math.max(-Math.max(c,l,u),Math.min(c,l,u))>o)return!1}return!0}const Te=new F,cr=new Nt;let Qd=0;class Ae extends fi{constructor(t,e,i=!1){if(super(),Array.isArray(t))throw new TypeError("THREE.BufferAttribute: array should be a Typed Array.");this.isBufferAttribute=!0,Object.defineProperty(this,"id",{value:Qd++}),this.name="",this.array=t,this.itemSize=e,this.count=t!==void 0?t.length/e:0,this.normalized=i,this.usage=Zo,this.updateRanges=[],this.gpuType=An,this.version=0}onUploadCallback(){}set needsUpdate(t){t===!0&&this.version++}setUsage(t){return this.usage=t,this}addUpdateRange(t,e){this.updateRanges.push({start:t,count:e})}clearUpdateRanges(){this.updateRanges.length=0}copy(t){return this.name=t.name,this.array=new t.array.constructor(t.array),this.itemSize=t.itemSize,this.count=t.count,this.normalized=t.normalized,this.usage=t.usage,this.gpuType=t.gpuType,this}copyAt(t,e,i){t*=this.itemSize,i*=e.itemSize;for(let s=0,r=this.itemSize;s<r;s++)this.array[t+s]=e.array[i+s];return this}copyArray(t){return this.array.set(t),this}applyMatrix3(t){if(this.itemSize===2)for(let e=0,i=this.count;e<i;e++)cr.fromBufferAttribute(this,e),cr.applyMatrix3(t),this.setXY(e,cr.x,cr.y);else if(this.itemSize===3)for(let e=0,i=this.count;e<i;e++)Te.fromBufferAttribute(this,e),Te.applyMatrix3(t),this.setXYZ(e,Te.x,Te.y,Te.z);return this}applyMatrix4(t){for(let e=0,i=this.count;e<i;e++)Te.fromBufferAttribute(this,e),Te.applyMatrix4(t),this.setXYZ(e,Te.x,Te.y,Te.z);return this}applyNormalMatrix(t){for(let e=0,i=this.count;e<i;e++)Te.fromBufferAttribute(this,e),Te.applyNormalMatrix(t),this.setXYZ(e,Te.x,Te.y,Te.z);return this}transformDirection(t){for(let e=0,i=this.count;e<i;e++)Te.fromBufferAttribute(this,e),Te.transformDirection(t),this.setXYZ(e,Te.x,Te.y,Te.z);return this}set(t,e=0){return this.array.set(t,e),this}getComponent(t,e){let i=this.array[t*this.itemSize+e];return this.normalized&&(i=bn(i,this.array)),i}setComponent(t,e,i){return this.normalized&&(i=oe(i,this.array)),this.array[t*this.itemSize+e]=i,this}getX(t){let e=this.array[t*this.itemSize];return this.normalized&&(e=bn(e,this.array)),e}setX(t,e){return this.normalized&&(e=oe(e,this.array)),this.array[t*this.itemSize]=e,this}getY(t){let e=this.array[t*this.itemSize+1];return this.normalized&&(e=bn(e,this.array)),e}setY(t,e){return this.normalized&&(e=oe(e,this.array)),this.array[t*this.itemSize+1]=e,this}getZ(t){let e=this.array[t*this.itemSize+2];return this.normalized&&(e=bn(e,this.array)),e}setZ(t,e){return this.normalized&&(e=oe(e,this.array)),this.array[t*this.itemSize+2]=e,this}getW(t){let e=this.array[t*this.itemSize+3];return this.normalized&&(e=bn(e,this.array)),e}setW(t,e){return this.normalized&&(e=oe(e,this.array)),this.array[t*this.itemSize+3]=e,this}setXY(t,e,i){return t*=this.itemSize,this.normalized&&(e=oe(e,this.array),i=oe(i,this.array)),this.array[t+0]=e,this.array[t+1]=i,this}setXYZ(t,e,i,s){return t*=this.itemSize,this.normalized&&(e=oe(e,this.array),i=oe(i,this.array),s=oe(s,this.array)),this.array[t+0]=e,this.array[t+1]=i,this.array[t+2]=s,this}setXYZW(t,e,i,s,r){return t*=this.itemSize,this.normalized&&(e=oe(e,this.array),i=oe(i,this.array),s=oe(s,this.array),r=oe(r,this.array)),this.array[t+0]=e,this.array[t+1]=i,this.array[t+2]=s,this.array[t+3]=r,this}onUpload(t){return this.onUploadCallback=t,this}clone(){return new this.constructor(this.array,this.itemSize).copy(this)}toJSON(){const t={itemSize:this.itemSize,type:this.array.constructor.name,array:Array.from(this.array),normalized:this.normalized};return this.name!==""&&(t.name=this.name),this.usage!==Zo&&(t.usage=this.usage),t}dispose(){this.dispatchEvent({type:"dispose"})}}class Bh extends Ae{constructor(t,e,i){super(new Uint16Array(t),e,i)}}class zh extends Ae{constructor(t,e,i){super(new Uint32Array(t),e,i)}}class mn extends Ae{constructor(t,e,i){super(new Float32Array(t),e,i)}}const tf=new Ys,ys=new F,Wa=new F;class js{constructor(t=new F,e=-1){this.isSphere=!0,this.center=t,this.radius=e}set(t,e){return this.center.copy(t),this.radius=e,this}setFromPoints(t,e){const i=this.center;e!==void 0?i.copy(e):tf.setFromPoints(t).getCenter(i);let s=0;for(let r=0,a=t.length;r<a;r++)s=Math.max(s,i.distanceToSquared(t[r]));return this.radius=Math.sqrt(s),this}copy(t){return this.center.copy(t.center),this.radius=t.radius,this}isEmpty(){return this.radius<0}makeEmpty(){return this.center.set(0,0,0),this.radius=-1,this}containsPoint(t){return t.distanceToSquared(this.center)<=this.radius*this.radius}distanceToPoint(t){return t.distanceTo(this.center)-this.radius}intersectsSphere(t){const e=this.radius+t.radius;return t.center.distanceToSquared(this.center)<=e*e}intersectsBox(t){return t.intersectsSphere(this)}intersectsPlane(t){return Math.abs(t.distanceToPoint(this.center))<=this.radius}clampPoint(t,e){const i=this.center.distanceToSquared(t);return e.copy(t),i>this.radius*this.radius&&(e.sub(this.center).normalize(),e.multiplyScalar(this.radius).add(this.center)),e}getBoundingBox(t){return this.isEmpty()?(t.makeEmpty(),t):(t.set(this.center,this.center),t.expandByScalar(this.radius),t)}applyMatrix4(t){return this.center.applyMatrix4(t),this.radius=this.radius*t.getMaxScaleOnAxis(),this}translate(t){return this.center.add(t),this}expandByPoint(t){if(this.isEmpty())return this.center.copy(t),this.radius=0,this;ys.subVectors(t,this.center);const e=ys.lengthSq();if(e>this.radius*this.radius){const i=Math.sqrt(e),s=(i-this.radius)*.5;this.center.addScaledVector(ys,s/i),this.radius+=s}return this}union(t){return t.isEmpty()?this:this.isEmpty()?(this.copy(t),this):(this.center.equals(t.center)===!0?this.radius=Math.max(this.radius,t.radius):(Wa.subVectors(t.center,this.center).setLength(t.radius),this.expandByPoint(ys.copy(t.center).add(Wa)),this.expandByPoint(ys.copy(t.center).sub(Wa))),this)}equals(t){return t.center.equals(this.center)&&t.radius===this.radius}clone(){return new this.constructor().copy(this)}toJSON(){return{radius:this.radius,center:this.center.toArray()}}fromJSON(t){return this.radius=t.radius,this.center.fromArray(t.center),this}}let ef=0;const nn=new me,Xa=new ke,Ki=new F,Qe=new Ys,Es=new Ys,Pe=new F;class Ue extends fi{constructor(){super(),this.isBufferGeometry=!0,Object.defineProperty(this,"id",{value:ef++}),this.uuid=oi(),this.name="",this.type="BufferGeometry",this.index=null,this.indirect=null,this.indirectOffset=0,this.attributes={},this.morphAttributes={},this.morphTargetsRelative=!1,this.groups=[],this.boundingBox=null,this.boundingSphere=null,this.drawRange={start:0,count:1/0},this.userData={}}getIndex(){return this.index}setIndex(t){return Array.isArray(t)?this.index=new(Id(t)?zh:Bh)(t,1):this.index=t,this}setIndirect(t,e=0){return this.indirect=t,this.indirectOffset=e,this}getIndirect(){return this.indirect}getAttribute(t){return this.attributes[t]}setAttribute(t,e){return this.attributes[t]=e,this}deleteAttribute(t){return delete this.attributes[t],this}hasAttribute(t){return this.attributes[t]!==void 0}addGroup(t,e,i=0){this.groups.push({start:t,count:e,materialIndex:i})}clearGroups(){this.groups=[]}setDrawRange(t,e){this.drawRange.start=t,this.drawRange.count=e}applyMatrix4(t){const e=this.attributes.position;e!==void 0&&(e.applyMatrix4(t),e.needsUpdate=!0);const i=this.attributes.normal;if(i!==void 0){const r=new zt().getNormalMatrix(t);i.applyNormalMatrix(r),i.needsUpdate=!0}const s=this.attributes.tangent;return s!==void 0&&(s.transformDirection(t),s.needsUpdate=!0),this.boundingBox!==null&&this.computeBoundingBox(),this.boundingSphere!==null&&this.computeBoundingSphere(),this}applyQuaternion(t){return nn.makeRotationFromQuaternion(t),this.applyMatrix4(nn),this}rotateX(t){return nn.makeRotationX(t),this.applyMatrix4(nn),this}rotateY(t){return nn.makeRotationY(t),this.applyMatrix4(nn),this}rotateZ(t){return nn.makeRotationZ(t),this.applyMatrix4(nn),this}translate(t,e,i){return nn.makeTranslation(t,e,i),this.applyMatrix4(nn),this}scale(t,e,i){return nn.makeScale(t,e,i),this.applyMatrix4(nn),this}lookAt(t){return Xa.lookAt(t),Xa.updateMatrix(),this.applyMatrix4(Xa.matrix),this}center(){return this.computeBoundingBox(),this.boundingBox.getCenter(Ki).negate(),this.translate(Ki.x,Ki.y,Ki.z),this}setFromPoints(t){const e=this.getAttribute("position");if(e===void 0){const i=[];for(let s=0,r=t.length;s<r;s++){const a=t[s];i.push(a.x,a.y,a.z||0)}this.setAttribute("position",new mn(i,3))}else{const i=Math.min(t.length,e.count);for(let s=0;s<i;s++){const r=t[s];e.setXYZ(s,r.x,r.y,r.z||0)}t.length>e.count&&It("BufferGeometry: Buffer size too small for points data. Use .dispose() and create a new geometry."),e.needsUpdate=!0}return this}computeBoundingBox(){this.boundingBox===null&&(this.boundingBox=new Ys);const t=this.attributes.position,e=this.morphAttributes.position;if(t&&t.isGLBufferAttribute){Kt("BufferGeometry.computeBoundingBox(): GLBufferAttribute requires a manual bounding box.",this),this.boundingBox.set(new F(-1/0,-1/0,-1/0),new F(1/0,1/0,1/0));return}if(t!==void 0){if(this.boundingBox.setFromBufferAttribute(t),e)for(let i=0,s=e.length;i<s;i++){const r=e[i];Qe.setFromBufferAttribute(r),this.morphTargetsRelative?(Pe.addVectors(this.boundingBox.min,Qe.min),this.boundingBox.expandByPoint(Pe),Pe.addVectors(this.boundingBox.max,Qe.max),this.boundingBox.expandByPoint(Pe)):(this.boundingBox.expandByPoint(Qe.min),this.boundingBox.expandByPoint(Qe.max))}}else this.boundingBox.makeEmpty();(isNaN(this.boundingBox.min.x)||isNaN(this.boundingBox.min.y)||isNaN(this.boundingBox.min.z))&&Kt('BufferGeometry.computeBoundingBox(): Computed min/max have NaN values. The "position" attribute is likely to have NaN values.',this)}computeBoundingSphere(){this.boundingSphere===null&&(this.boundingSphere=new js);const t=this.attributes.position,e=this.morphAttributes.position;if(t&&t.isGLBufferAttribute){Kt("BufferGeometry.computeBoundingSphere(): GLBufferAttribute requires a manual bounding sphere.",this),this.boundingSphere.set(new F,1/0);return}if(t){const i=this.boundingSphere.center;if(Qe.setFromBufferAttribute(t),e)for(let r=0,a=e.length;r<a;r++){const o=e[r];Es.setFromBufferAttribute(o),this.morphTargetsRelative?(Pe.addVectors(Qe.min,Es.min),Qe.expandByPoint(Pe),Pe.addVectors(Qe.max,Es.max),Qe.expandByPoint(Pe)):(Qe.expandByPoint(Es.min),Qe.expandByPoint(Es.max))}Qe.getCenter(i);let s=0;for(let r=0,a=t.count;r<a;r++)Pe.fromBufferAttribute(t,r),s=Math.max(s,i.distanceToSquared(Pe));if(e)for(let r=0,a=e.length;r<a;r++){const o=e[r],c=this.morphTargetsRelative;for(let l=0,u=o.count;l<u;l++)Pe.fromBufferAttribute(o,l),c&&(Ki.fromBufferAttribute(t,l),Pe.add(Ki)),s=Math.max(s,i.distanceToSquared(Pe))}this.boundingSphere.radius=Math.sqrt(s),isNaN(this.boundingSphere.radius)&&Kt('BufferGeometry.computeBoundingSphere(): Computed radius is NaN. The "position" attribute is likely to have NaN values.',this)}}computeTangents(){const t=this.index,e=this.attributes;if(t===null||e.position===void 0||e.normal===void 0||e.uv===void 0){Kt("BufferGeometry: .computeTangents() failed. Missing required attributes (index, position, normal or uv)");return}const i=e.position,s=e.normal,r=e.uv;this.hasAttribute("tangent")===!1&&this.setAttribute("tangent",new Ae(new Float32Array(4*i.count),4));const a=this.getAttribute("tangent"),o=[],c=[];for(let M=0;M<i.count;M++)o[M]=new F,c[M]=new F;const l=new F,u=new F,g=new F,h=new Nt,m=new Nt,_=new Nt,y=new F,f=new F;function d(M,w,z){l.fromBufferAttribute(i,M),u.fromBufferAttribute(i,w),g.fromBufferAttribute(i,z),h.fromBufferAttribute(r,M),m.fromBufferAttribute(r,w),_.fromBufferAttribute(r,z),u.sub(l),g.sub(l),m.sub(h),_.sub(h);const P=1/(m.x*_.y-_.x*m.y);isFinite(P)&&(y.copy(u).multiplyScalar(_.y).addScaledVector(g,-m.y).multiplyScalar(P),f.copy(g).multiplyScalar(m.x).addScaledVector(u,-_.x).multiplyScalar(P),o[M].add(y),o[w].add(y),o[z].add(y),c[M].add(f),c[w].add(f),c[z].add(f))}let E=this.groups;E.length===0&&(E=[{start:0,count:t.count}]);for(let M=0,w=E.length;M<w;++M){const z=E[M],P=z.start,V=z.count;for(let J=P,K=P+V;J<K;J+=3)d(t.getX(J+0),t.getX(J+1),t.getX(J+2))}const A=new F,T=new F,D=new F,b=new F;function N(M){D.fromBufferAttribute(s,M),b.copy(D);const w=o[M];A.copy(w),A.sub(D.multiplyScalar(D.dot(w))).normalize(),T.crossVectors(b,w);const P=T.dot(c[M])<0?-1:1;a.setXYZW(M,A.x,A.y,A.z,P)}for(let M=0,w=E.length;M<w;++M){const z=E[M],P=z.start,V=z.count;for(let J=P,K=P+V;J<K;J+=3)N(t.getX(J+0)),N(t.getX(J+1)),N(t.getX(J+2))}}computeVertexNormals(){const t=this.index,e=this.getAttribute("position");if(e!==void 0){let i=this.getAttribute("normal");if(i===void 0)i=new Ae(new Float32Array(e.count*3),3),this.setAttribute("normal",i);else for(let h=0,m=i.count;h<m;h++)i.setXYZ(h,0,0,0);const s=new F,r=new F,a=new F,o=new F,c=new F,l=new F,u=new F,g=new F;if(t)for(let h=0,m=t.count;h<m;h+=3){const _=t.getX(h+0),y=t.getX(h+1),f=t.getX(h+2);s.fromBufferAttribute(e,_),r.fromBufferAttribute(e,y),a.fromBufferAttribute(e,f),u.subVectors(a,r),g.subVectors(s,r),u.cross(g),o.fromBufferAttribute(i,_),c.fromBufferAttribute(i,y),l.fromBufferAttribute(i,f),o.add(u),c.add(u),l.add(u),i.setXYZ(_,o.x,o.y,o.z),i.setXYZ(y,c.x,c.y,c.z),i.setXYZ(f,l.x,l.y,l.z)}else for(let h=0,m=e.count;h<m;h+=3)s.fromBufferAttribute(e,h+0),r.fromBufferAttribute(e,h+1),a.fromBufferAttribute(e,h+2),u.subVectors(a,r),g.subVectors(s,r),u.cross(g),i.setXYZ(h+0,u.x,u.y,u.z),i.setXYZ(h+1,u.x,u.y,u.z),i.setXYZ(h+2,u.x,u.y,u.z);this.normalizeNormals(),i.needsUpdate=!0}}normalizeNormals(){const t=this.attributes.normal;for(let e=0,i=t.count;e<i;e++)Pe.fromBufferAttribute(t,e),Pe.normalize(),t.setXYZ(e,Pe.x,Pe.y,Pe.z)}toNonIndexed(){function t(o,c){const l=o.array,u=o.itemSize,g=o.normalized,h=new l.constructor(c.length*u);let m=0,_=0;for(let y=0,f=c.length;y<f;y++){o.isInterleavedBufferAttribute?m=c[y]*o.data.stride+o.offset:m=c[y]*u;for(let d=0;d<u;d++)h[_++]=l[m++]}return new Ae(h,u,g)}if(this.index===null)return It("BufferGeometry.toNonIndexed(): BufferGeometry is already non-indexed."),this;const e=new Ue,i=this.index.array,s=this.attributes;for(const o in s){const c=s[o],l=t(c,i);e.setAttribute(o,l)}const r=this.morphAttributes;for(const o in r){const c=[],l=r[o];for(let u=0,g=l.length;u<g;u++){const h=l[u],m=t(h,i);c.push(m)}e.morphAttributes[o]=c}e.morphTargetsRelative=this.morphTargetsRelative;const a=this.groups;for(let o=0,c=a.length;o<c;o++){const l=a[o];e.addGroup(l.start,l.count,l.materialIndex)}return e}toJSON(){const t={metadata:{version:4.7,type:"BufferGeometry",generator:"BufferGeometry.toJSON"}};if(t.uuid=this.uuid,t.type=this.type,this.name!==""&&(t.name=this.name),Object.keys(this.userData).length>0&&(t.userData=this.userData),this.parameters!==void 0){const c=this.parameters;for(const l in c)c[l]!==void 0&&(t[l]=c[l]);return t}t.data={attributes:{}};const e=this.index;e!==null&&(t.data.index={type:e.array.constructor.name,array:Array.prototype.slice.call(e.array)});const i=this.attributes;for(const c in i){const l=i[c];t.data.attributes[c]=l.toJSON(t.data)}const s={};let r=!1;for(const c in this.morphAttributes){const l=this.morphAttributes[c],u=[];for(let g=0,h=l.length;g<h;g++){const m=l[g];u.push(m.toJSON(t.data))}u.length>0&&(s[c]=u,r=!0)}r&&(t.data.morphAttributes=s,t.data.morphTargetsRelative=this.morphTargetsRelative);const a=this.groups;a.length>0&&(t.data.groups=JSON.parse(JSON.stringify(a)));const o=this.boundingSphere;return o!==null&&(t.data.boundingSphere=o.toJSON()),t}clone(){return new this.constructor().copy(this)}copy(t){this.index=null,this.attributes={},this.morphAttributes={},this.groups=[],this.boundingBox=null,this.boundingSphere=null;const e={};this.name=t.name;const i=t.index;i!==null&&this.setIndex(i.clone());const s=t.attributes;for(const l in s){const u=s[l];this.setAttribute(l,u.clone(e))}const r=t.morphAttributes;for(const l in r){const u=[],g=r[l];for(let h=0,m=g.length;h<m;h++)u.push(g[h].clone(e));this.morphAttributes[l]=u}this.morphTargetsRelative=t.morphTargetsRelative;const a=t.groups;for(let l=0,u=a.length;l<u;l++){const g=a[l];this.addGroup(g.start,g.count,g.materialIndex)}const o=t.boundingBox;o!==null&&(this.boundingBox=o.clone());const c=t.boundingSphere;return c!==null&&(this.boundingSphere=c.clone()),this.drawRange.start=t.drawRange.start,this.drawRange.count=t.drawRange.count,this.userData=t.userData,this}dispose(){this.dispatchEvent({type:"dispose"})}}class nf{constructor(t,e){this.isInterleavedBuffer=!0,this.array=t,this.stride=e,this.count=t!==void 0?t.length/e:0,this.usage=Zo,this.updateRanges=[],this.version=0,this.uuid=oi()}onUploadCallback(){}set needsUpdate(t){t===!0&&this.version++}setUsage(t){return this.usage=t,this}addUpdateRange(t,e){this.updateRanges.push({start:t,count:e})}clearUpdateRanges(){this.updateRanges.length=0}copy(t){return this.array=new t.array.constructor(t.array),this.count=t.count,this.stride=t.stride,this.usage=t.usage,this}copyAt(t,e,i){t*=this.stride,i*=e.stride;for(let s=0,r=this.stride;s<r;s++)this.array[t+s]=e.array[i+s];return this}set(t,e=0){return this.array.set(t,e),this}clone(t){t.arrayBuffers===void 0&&(t.arrayBuffers={}),this.array.buffer._uuid===void 0&&(this.array.buffer._uuid=oi()),t.arrayBuffers[this.array.buffer._uuid]===void 0&&(t.arrayBuffers[this.array.buffer._uuid]=this.array.slice(0).buffer);const e=new this.array.constructor(t.arrayBuffers[this.array.buffer._uuid]),i=new this.constructor(e,this.stride);return i.setUsage(this.usage),i}onUpload(t){return this.onUploadCallback=t,this}toJSON(t){return t.arrayBuffers===void 0&&(t.arrayBuffers={}),this.array.buffer._uuid===void 0&&(this.array.buffer._uuid=oi()),t.arrayBuffers[this.array.buffer._uuid]===void 0&&(t.arrayBuffers[this.array.buffer._uuid]=Array.from(new Uint32Array(this.array.buffer))),{uuid:this.uuid,buffer:this.array.buffer._uuid,type:this.array.constructor.name,stride:this.stride}}}const He=new F;class la{constructor(t,e,i,s=!1){this.isInterleavedBufferAttribute=!0,this.name="",this.data=t,this.itemSize=e,this.offset=i,this.normalized=s}get count(){return this.data.count}get array(){return this.data.array}set needsUpdate(t){this.data.needsUpdate=t}applyMatrix4(t){for(let e=0,i=this.data.count;e<i;e++)He.fromBufferAttribute(this,e),He.applyMatrix4(t),this.setXYZ(e,He.x,He.y,He.z);return this}applyNormalMatrix(t){for(let e=0,i=this.count;e<i;e++)He.fromBufferAttribute(this,e),He.applyNormalMatrix(t),this.setXYZ(e,He.x,He.y,He.z);return this}transformDirection(t){for(let e=0,i=this.count;e<i;e++)He.fromBufferAttribute(this,e),He.transformDirection(t),this.setXYZ(e,He.x,He.y,He.z);return this}getComponent(t,e){let i=this.array[t*this.data.stride+this.offset+e];return this.normalized&&(i=bn(i,this.array)),i}setComponent(t,e,i){return this.normalized&&(i=oe(i,this.array)),this.data.array[t*this.data.stride+this.offset+e]=i,this}setX(t,e){return this.normalized&&(e=oe(e,this.array)),this.data.array[t*this.data.stride+this.offset]=e,this}setY(t,e){return this.normalized&&(e=oe(e,this.array)),this.data.array[t*this.data.stride+this.offset+1]=e,this}setZ(t,e){return this.normalized&&(e=oe(e,this.array)),this.data.array[t*this.data.stride+this.offset+2]=e,this}setW(t,e){return this.normalized&&(e=oe(e,this.array)),this.data.array[t*this.data.stride+this.offset+3]=e,this}getX(t){let e=this.data.array[t*this.data.stride+this.offset];return this.normalized&&(e=bn(e,this.array)),e}getY(t){let e=this.data.array[t*this.data.stride+this.offset+1];return this.normalized&&(e=bn(e,this.array)),e}getZ(t){let e=this.data.array[t*this.data.stride+this.offset+2];return this.normalized&&(e=bn(e,this.array)),e}getW(t){let e=this.data.array[t*this.data.stride+this.offset+3];return this.normalized&&(e=bn(e,this.array)),e}setXY(t,e,i){return t=t*this.data.stride+this.offset,this.normalized&&(e=oe(e,this.array),i=oe(i,this.array)),this.data.array[t+0]=e,this.data.array[t+1]=i,this}setXYZ(t,e,i,s){return t=t*this.data.stride+this.offset,this.normalized&&(e=oe(e,this.array),i=oe(i,this.array),s=oe(s,this.array)),this.data.array[t+0]=e,this.data.array[t+1]=i,this.data.array[t+2]=s,this}setXYZW(t,e,i,s,r){return t=t*this.data.stride+this.offset,this.normalized&&(e=oe(e,this.array),i=oe(i,this.array),s=oe(s,this.array),r=oe(r,this.array)),this.data.array[t+0]=e,this.data.array[t+1]=i,this.data.array[t+2]=s,this.data.array[t+3]=r,this}clone(t){if(t===void 0){oa("InterleavedBufferAttribute.clone(): Cloning an interleaved buffer attribute will de-interleave buffer data.");const e=[];for(let i=0;i<this.count;i++){const s=i*this.data.stride+this.offset;for(let r=0;r<this.itemSize;r++)e.push(this.data.array[s+r])}return new Ae(new this.array.constructor(e),this.itemSize,this.normalized)}else return t.interleavedBuffers===void 0&&(t.interleavedBuffers={}),t.interleavedBuffers[this.data.uuid]===void 0&&(t.interleavedBuffers[this.data.uuid]=this.data.clone(t)),new la(t.interleavedBuffers[this.data.uuid],this.itemSize,this.offset,this.normalized)}toJSON(t){if(t===void 0){oa("InterleavedBufferAttribute.toJSON(): Serializing an interleaved buffer attribute will de-interleave buffer data.");const e=[];for(let i=0;i<this.count;i++){const s=i*this.data.stride+this.offset;for(let r=0;r<this.itemSize;r++)e.push(this.data.array[s+r])}return{itemSize:this.itemSize,type:this.array.constructor.name,array:e,normalized:this.normalized}}else return t.interleavedBuffers===void 0&&(t.interleavedBuffers={}),t.interleavedBuffers[this.data.uuid]===void 0&&(t.interleavedBuffers[this.data.uuid]=this.data.toJSON(t)),{isInterleavedBufferAttribute:!0,itemSize:this.itemSize,data:this.data.uuid,offset:this.offset,normalized:this.normalized}}}let sf=0;class Ii extends fi{constructor(){super(),this.isMaterial=!0,Object.defineProperty(this,"id",{value:sf++}),this.uuid=oi(),this.name="",this.type="Material",this.blending=us,this.side=ci,this.vertexColors=!1,this.opacity=1,this.transparent=!1,this.alphaHash=!1,this.blendSrc=co,this.blendDst=ho,this.blendEquation=Si,this.blendSrcAlpha=null,this.blendDstAlpha=null,this.blendEquationAlpha=null,this.blendColor=new Bt(0,0,0),this.blendAlpha=0,this.depthFunc=ps,this.depthTest=!0,this.depthWrite=!0,this.stencilWriteMask=255,this.stencilFunc=rc,this.stencilRef=0,this.stencilFuncMask=255,this.stencilFail=ki,this.stencilZFail=ki,this.stencilZPass=ki,this.stencilWrite=!1,this.clippingPlanes=null,this.clipIntersection=!1,this.clipShadows=!1,this.shadowSide=null,this.colorWrite=!0,this.precision=null,this.polygonOffset=!1,this.polygonOffsetFactor=0,this.polygonOffsetUnits=0,this.dithering=!1,this.alphaToCoverage=!1,this.premultipliedAlpha=!1,this.forceSinglePass=!1,this.allowOverride=!0,this.visible=!0,this.toneMapped=!0,this.userData={},this.version=0,this._alphaTest=0}get alphaTest(){return this._alphaTest}set alphaTest(t){this._alphaTest>0!=t>0&&this.version++,this._alphaTest=t}onBeforeRender(){}onBeforeCompile(){}customProgramCacheKey(){return this.onBeforeCompile.toString()}setValues(t){if(t!==void 0)for(const e in t){const i=t[e];if(i===void 0){It(`Material: parameter '${e}' has value of undefined.`);continue}const s=this[e];if(s===void 0){It(`Material: '${e}' is not a property of THREE.${this.type}.`);continue}s&&s.isColor?s.set(i):s&&s.isVector3&&i&&i.isVector3?s.copy(i):this[e]=i}}toJSON(t){const e=t===void 0||typeof t=="string";e&&(t={textures:{},images:{}});const i={metadata:{version:4.7,type:"Material",generator:"Material.toJSON"}};i.uuid=this.uuid,i.type=this.type,this.name!==""&&(i.name=this.name),this.color&&this.color.isColor&&(i.color=this.color.getHex()),this.roughness!==void 0&&(i.roughness=this.roughness),this.metalness!==void 0&&(i.metalness=this.metalness),this.sheen!==void 0&&(i.sheen=this.sheen),this.sheenColor&&this.sheenColor.isColor&&(i.sheenColor=this.sheenColor.getHex()),this.sheenRoughness!==void 0&&(i.sheenRoughness=this.sheenRoughness),this.emissive&&this.emissive.isColor&&(i.emissive=this.emissive.getHex()),this.emissiveIntensity!==void 0&&this.emissiveIntensity!==1&&(i.emissiveIntensity=this.emissiveIntensity),this.specular&&this.specular.isColor&&(i.specular=this.specular.getHex()),this.specularIntensity!==void 0&&(i.specularIntensity=this.specularIntensity),this.specularColor&&this.specularColor.isColor&&(i.specularColor=this.specularColor.getHex()),this.shininess!==void 0&&(i.shininess=this.shininess),this.clearcoat!==void 0&&(i.clearcoat=this.clearcoat),this.clearcoatRoughness!==void 0&&(i.clearcoatRoughness=this.clearcoatRoughness),this.clearcoatMap&&this.clearcoatMap.isTexture&&(i.clearcoatMap=this.clearcoatMap.toJSON(t).uuid),this.clearcoatRoughnessMap&&this.clearcoatRoughnessMap.isTexture&&(i.clearcoatRoughnessMap=this.clearcoatRoughnessMap.toJSON(t).uuid),this.clearcoatNormalMap&&this.clearcoatNormalMap.isTexture&&(i.clearcoatNormalMap=this.clearcoatNormalMap.toJSON(t).uuid,i.clearcoatNormalScale=this.clearcoatNormalScale.toArray()),this.sheenColorMap&&this.sheenColorMap.isTexture&&(i.sheenColorMap=this.sheenColorMap.toJSON(t).uuid),this.sheenRoughnessMap&&this.sheenRoughnessMap.isTexture&&(i.sheenRoughnessMap=this.sheenRoughnessMap.toJSON(t).uuid),this.dispersion!==void 0&&(i.dispersion=this.dispersion),this.iridescence!==void 0&&(i.iridescence=this.iridescence),this.iridescenceIOR!==void 0&&(i.iridescenceIOR=this.iridescenceIOR),this.iridescenceThicknessRange!==void 0&&(i.iridescenceThicknessRange=this.iridescenceThicknessRange),this.iridescenceMap&&this.iridescenceMap.isTexture&&(i.iridescenceMap=this.iridescenceMap.toJSON(t).uuid),this.iridescenceThicknessMap&&this.iridescenceThicknessMap.isTexture&&(i.iridescenceThicknessMap=this.iridescenceThicknessMap.toJSON(t).uuid),this.anisotropy!==void 0&&(i.anisotropy=this.anisotropy),this.anisotropyRotation!==void 0&&(i.anisotropyRotation=this.anisotropyRotation),this.anisotropyMap&&this.anisotropyMap.isTexture&&(i.anisotropyMap=this.anisotropyMap.toJSON(t).uuid),this.map&&this.map.isTexture&&(i.map=this.map.toJSON(t).uuid),this.matcap&&this.matcap.isTexture&&(i.matcap=this.matcap.toJSON(t).uuid),this.alphaMap&&this.alphaMap.isTexture&&(i.alphaMap=this.alphaMap.toJSON(t).uuid),this.lightMap&&this.lightMap.isTexture&&(i.lightMap=this.lightMap.toJSON(t).uuid,i.lightMapIntensity=this.lightMapIntensity),this.aoMap&&this.aoMap.isTexture&&(i.aoMap=this.aoMap.toJSON(t).uuid,i.aoMapIntensity=this.aoMapIntensity),this.bumpMap&&this.bumpMap.isTexture&&(i.bumpMap=this.bumpMap.toJSON(t).uuid,i.bumpScale=this.bumpScale),this.normalMap&&this.normalMap.isTexture&&(i.normalMap=this.normalMap.toJSON(t).uuid,i.normalMapType=this.normalMapType,i.normalScale=this.normalScale.toArray()),this.displacementMap&&this.displacementMap.isTexture&&(i.displacementMap=this.displacementMap.toJSON(t).uuid,i.displacementScale=this.displacementScale,i.displacementBias=this.displacementBias),this.roughnessMap&&this.roughnessMap.isTexture&&(i.roughnessMap=this.roughnessMap.toJSON(t).uuid),this.metalnessMap&&this.metalnessMap.isTexture&&(i.metalnessMap=this.metalnessMap.toJSON(t).uuid),this.emissiveMap&&this.emissiveMap.isTexture&&(i.emissiveMap=this.emissiveMap.toJSON(t).uuid),this.specularMap&&this.specularMap.isTexture&&(i.specularMap=this.specularMap.toJSON(t).uuid),this.specularIntensityMap&&this.specularIntensityMap.isTexture&&(i.specularIntensityMap=this.specularIntensityMap.toJSON(t).uuid),this.specularColorMap&&this.specularColorMap.isTexture&&(i.specularColorMap=this.specularColorMap.toJSON(t).uuid),this.envMap&&this.envMap.isTexture&&(i.envMap=this.envMap.toJSON(t).uuid,this.combine!==void 0&&(i.combine=this.combine)),this.envMapRotation!==void 0&&(i.envMapRotation=this.envMapRotation.toArray()),this.envMapIntensity!==void 0&&(i.envMapIntensity=this.envMapIntensity),this.reflectivity!==void 0&&(i.reflectivity=this.reflectivity),this.refractionRatio!==void 0&&(i.refractionRatio=this.refractionRatio),this.gradientMap&&this.gradientMap.isTexture&&(i.gradientMap=this.gradientMap.toJSON(t).uuid),this.transmission!==void 0&&(i.transmission=this.transmission),this.transmissionMap&&this.transmissionMap.isTexture&&(i.transmissionMap=this.transmissionMap.toJSON(t).uuid),this.thickness!==void 0&&(i.thickness=this.thickness),this.thicknessMap&&this.thicknessMap.isTexture&&(i.thicknessMap=this.thicknessMap.toJSON(t).uuid),this.attenuationDistance!==void 0&&this.attenuationDistance!==1/0&&(i.attenuationDistance=this.attenuationDistance),this.attenuationColor!==void 0&&(i.attenuationColor=this.attenuationColor.getHex()),this.size!==void 0&&(i.size=this.size),this.shadowSide!==null&&(i.shadowSide=this.shadowSide),this.sizeAttenuation!==void 0&&(i.sizeAttenuation=this.sizeAttenuation),this.blending!==us&&(i.blending=this.blending),this.side!==ci&&(i.side=this.side),this.vertexColors===!0&&(i.vertexColors=!0),this.opacity<1&&(i.opacity=this.opacity),this.transparent===!0&&(i.transparent=!0),this.blendSrc!==co&&(i.blendSrc=this.blendSrc),this.blendDst!==ho&&(i.blendDst=this.blendDst),this.blendEquation!==Si&&(i.blendEquation=this.blendEquation),this.blendSrcAlpha!==null&&(i.blendSrcAlpha=this.blendSrcAlpha),this.blendDstAlpha!==null&&(i.blendDstAlpha=this.blendDstAlpha),this.blendEquationAlpha!==null&&(i.blendEquationAlpha=this.blendEquationAlpha),this.blendColor&&this.blendColor.isColor&&(i.blendColor=this.blendColor.getHex()),this.blendAlpha!==0&&(i.blendAlpha=this.blendAlpha),this.depthFunc!==ps&&(i.depthFunc=this.depthFunc),this.depthTest===!1&&(i.depthTest=this.depthTest),this.depthWrite===!1&&(i.depthWrite=this.depthWrite),this.colorWrite===!1&&(i.colorWrite=this.colorWrite),this.stencilWriteMask!==255&&(i.stencilWriteMask=this.stencilWriteMask),this.stencilFunc!==rc&&(i.stencilFunc=this.stencilFunc),this.stencilRef!==0&&(i.stencilRef=this.stencilRef),this.stencilFuncMask!==255&&(i.stencilFuncMask=this.stencilFuncMask),this.stencilFail!==ki&&(i.stencilFail=this.stencilFail),this.stencilZFail!==ki&&(i.stencilZFail=this.stencilZFail),this.stencilZPass!==ki&&(i.stencilZPass=this.stencilZPass),this.stencilWrite===!0&&(i.stencilWrite=this.stencilWrite),this.rotation!==void 0&&this.rotation!==0&&(i.rotation=this.rotation),this.polygonOffset===!0&&(i.polygonOffset=!0),this.polygonOffsetFactor!==0&&(i.polygonOffsetFactor=this.polygonOffsetFactor),this.polygonOffsetUnits!==0&&(i.polygonOffsetUnits=this.polygonOffsetUnits),this.linewidth!==void 0&&this.linewidth!==1&&(i.linewidth=this.linewidth),this.dashSize!==void 0&&(i.dashSize=this.dashSize),this.gapSize!==void 0&&(i.gapSize=this.gapSize),this.scale!==void 0&&(i.scale=this.scale),this.dithering===!0&&(i.dithering=!0),this.alphaTest>0&&(i.alphaTest=this.alphaTest),this.alphaHash===!0&&(i.alphaHash=!0),this.alphaToCoverage===!0&&(i.alphaToCoverage=!0),this.premultipliedAlpha===!0&&(i.premultipliedAlpha=!0),this.forceSinglePass===!0&&(i.forceSinglePass=!0),this.allowOverride===!1&&(i.allowOverride=!1),this.wireframe===!0&&(i.wireframe=!0),this.wireframeLinewidth>1&&(i.wireframeLinewidth=this.wireframeLinewidth),this.wireframeLinecap!=="round"&&(i.wireframeLinecap=this.wireframeLinecap),this.wireframeLinejoin!=="round"&&(i.wireframeLinejoin=this.wireframeLinejoin),this.flatShading===!0&&(i.flatShading=!0),this.visible===!1&&(i.visible=!1),this.toneMapped===!1&&(i.toneMapped=!1),this.fog===!1&&(i.fog=!1),Object.keys(this.userData).length>0&&(i.userData=this.userData);function s(r){const a=[];for(const o in r){const c=r[o];delete c.metadata,a.push(c)}return a}if(e){const r=s(t.textures),a=s(t.images);r.length>0&&(i.textures=r),a.length>0&&(i.images=a)}return i}clone(){return new this.constructor().copy(this)}copy(t){this.name=t.name,this.blending=t.blending,this.side=t.side,this.vertexColors=t.vertexColors,this.opacity=t.opacity,this.transparent=t.transparent,this.blendSrc=t.blendSrc,this.blendDst=t.blendDst,this.blendEquation=t.blendEquation,this.blendSrcAlpha=t.blendSrcAlpha,this.blendDstAlpha=t.blendDstAlpha,this.blendEquationAlpha=t.blendEquationAlpha,this.blendColor.copy(t.blendColor),this.blendAlpha=t.blendAlpha,this.depthFunc=t.depthFunc,this.depthTest=t.depthTest,this.depthWrite=t.depthWrite,this.stencilWriteMask=t.stencilWriteMask,this.stencilFunc=t.stencilFunc,this.stencilRef=t.stencilRef,this.stencilFuncMask=t.stencilFuncMask,this.stencilFail=t.stencilFail,this.stencilZFail=t.stencilZFail,this.stencilZPass=t.stencilZPass,this.stencilWrite=t.stencilWrite;const e=t.clippingPlanes;let i=null;if(e!==null){const s=e.length;i=new Array(s);for(let r=0;r!==s;++r)i[r]=e[r].clone()}return this.clippingPlanes=i,this.clipIntersection=t.clipIntersection,this.clipShadows=t.clipShadows,this.shadowSide=t.shadowSide,this.colorWrite=t.colorWrite,this.precision=t.precision,this.polygonOffset=t.polygonOffset,this.polygonOffsetFactor=t.polygonOffsetFactor,this.polygonOffsetUnits=t.polygonOffsetUnits,this.dithering=t.dithering,this.alphaTest=t.alphaTest,this.alphaHash=t.alphaHash,this.alphaToCoverage=t.alphaToCoverage,this.premultipliedAlpha=t.premultipliedAlpha,this.forceSinglePass=t.forceSinglePass,this.allowOverride=t.allowOverride,this.visible=t.visible,this.toneMapped=t.toneMapped,this.userData=JSON.parse(JSON.stringify(t.userData)),this}dispose(){this.dispatchEvent({type:"dispose"})}set needsUpdate(t){t===!0&&this.version++}}class kh extends Ii{constructor(t){super(),this.isSpriteMaterial=!0,this.type="SpriteMaterial",this.color=new Bt(16777215),this.map=null,this.alphaMap=null,this.rotation=0,this.sizeAttenuation=!0,this.transparent=!0,this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.alphaMap=t.alphaMap,this.rotation=t.rotation,this.sizeAttenuation=t.sizeAttenuation,this.fog=t.fog,this}}let Zi;const bs=new F,Ji=new F,Qi=new F,ts=new Nt,Ts=new Nt,Hh=new me,hr=new F,As=new F,ur=new F,vc=new Nt,$a=new Nt,Mc=new Nt;class rf extends ke{constructor(t=new kh){if(super(),this.isSprite=!0,this.type="Sprite",Zi===void 0){Zi=new Ue;const e=new Float32Array([-.5,-.5,0,0,0,.5,-.5,0,1,0,.5,.5,0,1,1,-.5,.5,0,0,1]),i=new nf(e,5);Zi.setIndex([0,1,2,0,2,3]),Zi.setAttribute("position",new la(i,3,0,!1)),Zi.setAttribute("uv",new la(i,2,3,!1))}this.geometry=Zi,this.material=t,this.center=new Nt(.5,.5),this.count=1}raycast(t,e){t.camera===null&&Kt('Sprite: "Raycaster.camera" needs to be set in order to raycast against sprites.'),Ji.setFromMatrixScale(this.matrixWorld),Hh.copy(t.camera.matrixWorld),this.modelViewMatrix.multiplyMatrices(t.camera.matrixWorldInverse,this.matrixWorld),Qi.setFromMatrixPosition(this.modelViewMatrix),t.camera.isPerspectiveCamera&&this.material.sizeAttenuation===!1&&Ji.multiplyScalar(-Qi.z);const i=this.material.rotation;let s,r;i!==0&&(r=Math.cos(i),s=Math.sin(i));const a=this.center;dr(hr.set(-.5,-.5,0),Qi,a,Ji,s,r),dr(As.set(.5,-.5,0),Qi,a,Ji,s,r),dr(ur.set(.5,.5,0),Qi,a,Ji,s,r),vc.set(0,0),$a.set(1,0),Mc.set(1,1);let o=t.ray.intersectTriangle(hr,As,ur,!1,bs);if(o===null&&(dr(As.set(-.5,.5,0),Qi,a,Ji,s,r),$a.set(0,1),o=t.ray.intersectTriangle(hr,ur,As,!1,bs),o===null))return;const c=t.ray.origin.distanceTo(bs);c<t.near||c>t.far||e.push({distance:c,point:bs.clone(),uv:on.getInterpolation(bs,hr,As,ur,vc,$a,Mc,new Nt),face:null,object:this})}copy(t,e){return super.copy(t,e),t.center!==void 0&&this.center.copy(t.center),this.material=t.material,this}}function dr(n,t,e,i,s,r){ts.subVectors(n,e).addScalar(.5).multiply(i),s!==void 0?(Ts.x=r*ts.x-s*ts.y,Ts.y=s*ts.x+r*ts.y):Ts.copy(ts),n.copy(t),n.x+=Ts.x,n.y+=Ts.y,n.applyMatrix4(Hh)}const On=new F,qa=new F,fr=new F,ei=new F,Ya=new F,pr=new F,ja=new F;class _a{constructor(t=new F,e=new F(0,0,-1)){this.origin=t,this.direction=e}set(t,e){return this.origin.copy(t),this.direction.copy(e),this}copy(t){return this.origin.copy(t.origin),this.direction.copy(t.direction),this}at(t,e){return e.copy(this.origin).addScaledVector(this.direction,t)}lookAt(t){return this.direction.copy(t).sub(this.origin).normalize(),this}recast(t){return this.origin.copy(this.at(t,On)),this}closestPointToPoint(t,e){e.subVectors(t,this.origin);const i=e.dot(this.direction);return i<0?e.copy(this.origin):e.copy(this.origin).addScaledVector(this.direction,i)}distanceToPoint(t){return Math.sqrt(this.distanceSqToPoint(t))}distanceSqToPoint(t){const e=On.subVectors(t,this.origin).dot(this.direction);return e<0?this.origin.distanceToSquared(t):(On.copy(this.origin).addScaledVector(this.direction,e),On.distanceToSquared(t))}distanceSqToSegment(t,e,i,s){qa.copy(t).add(e).multiplyScalar(.5),fr.copy(e).sub(t).normalize(),ei.copy(this.origin).sub(qa);const r=t.distanceTo(e)*.5,a=-this.direction.dot(fr),o=ei.dot(this.direction),c=-ei.dot(fr),l=ei.lengthSq(),u=Math.abs(1-a*a);let g,h,m,_;if(u>0)if(g=a*c-o,h=a*o-c,_=r*u,g>=0)if(h>=-_)if(h<=_){const y=1/u;g*=y,h*=y,m=g*(g+a*h+2*o)+h*(a*g+h+2*c)+l}else h=r,g=Math.max(0,-(a*h+o)),m=-g*g+h*(h+2*c)+l;else h=-r,g=Math.max(0,-(a*h+o)),m=-g*g+h*(h+2*c)+l;else h<=-_?(g=Math.max(0,-(-a*r+o)),h=g>0?-r:Math.min(Math.max(-r,-c),r),m=-g*g+h*(h+2*c)+l):h<=_?(g=0,h=Math.min(Math.max(-r,-c),r),m=h*(h+2*c)+l):(g=Math.max(0,-(a*r+o)),h=g>0?r:Math.min(Math.max(-r,-c),r),m=-g*g+h*(h+2*c)+l);else h=a>0?-r:r,g=Math.max(0,-(a*h+o)),m=-g*g+h*(h+2*c)+l;return i&&i.copy(this.origin).addScaledVector(this.direction,g),s&&s.copy(qa).addScaledVector(fr,h),m}intersectSphere(t,e){On.subVectors(t.center,this.origin);const i=On.dot(this.direction),s=On.dot(On)-i*i,r=t.radius*t.radius;if(s>r)return null;const a=Math.sqrt(r-s),o=i-a,c=i+a;return c<0?null:o<0?this.at(c,e):this.at(o,e)}intersectsSphere(t){return t.radius<0?!1:this.distanceSqToPoint(t.center)<=t.radius*t.radius}distanceToPlane(t){const e=t.normal.dot(this.direction);if(e===0)return t.distanceToPoint(this.origin)===0?0:null;const i=-(this.origin.dot(t.normal)+t.constant)/e;return i>=0?i:null}intersectPlane(t,e){const i=this.distanceToPlane(t);return i===null?null:this.at(i,e)}intersectsPlane(t){const e=t.distanceToPoint(this.origin);return e===0||t.normal.dot(this.direction)*e<0}intersectBox(t,e){let i,s,r,a,o,c;const l=1/this.direction.x,u=1/this.direction.y,g=1/this.direction.z,h=this.origin;return l>=0?(i=(t.min.x-h.x)*l,s=(t.max.x-h.x)*l):(i=(t.max.x-h.x)*l,s=(t.min.x-h.x)*l),u>=0?(r=(t.min.y-h.y)*u,a=(t.max.y-h.y)*u):(r=(t.max.y-h.y)*u,a=(t.min.y-h.y)*u),i>a||r>s||((r>i||isNaN(i))&&(i=r),(a<s||isNaN(s))&&(s=a),g>=0?(o=(t.min.z-h.z)*g,c=(t.max.z-h.z)*g):(o=(t.max.z-h.z)*g,c=(t.min.z-h.z)*g),i>c||o>s)||((o>i||i!==i)&&(i=o),(c<s||s!==s)&&(s=c),s<0)?null:this.at(i>=0?i:s,e)}intersectsBox(t){return this.intersectBox(t,On)!==null}intersectTriangle(t,e,i,s,r){Ya.subVectors(e,t),pr.subVectors(i,t),ja.crossVectors(Ya,pr);let a=this.direction.dot(ja),o;if(a>0){if(s)return null;o=1}else if(a<0)o=-1,a=-a;else return null;ei.subVectors(this.origin,t);const c=o*this.direction.dot(pr.crossVectors(ei,pr));if(c<0)return null;const l=o*this.direction.dot(Ya.cross(ei));if(l<0||c+l>a)return null;const u=-o*ei.dot(ja);return u<0?null:this.at(u/a,r)}applyMatrix4(t){return this.origin.applyMatrix4(t),this.direction.transformDirection(t),this}equals(t){return t.origin.equals(this.origin)&&t.direction.equals(this.direction)}clone(){return new this.constructor().copy(this)}}class Gh extends Ii{constructor(t){super(),this.isMeshBasicMaterial=!0,this.type="MeshBasicMaterial",this.color=new Bt(16777215),this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.specularMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Di,this.combine=xh,this.reflectivity=1,this.refractionRatio=.98,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap="round",this.wireframeLinejoin="round",this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.lightMap=t.lightMap,this.lightMapIntensity=t.lightMapIntensity,this.aoMap=t.aoMap,this.aoMapIntensity=t.aoMapIntensity,this.specularMap=t.specularMap,this.alphaMap=t.alphaMap,this.envMap=t.envMap,this.envMapRotation.copy(t.envMapRotation),this.combine=t.combine,this.reflectivity=t.reflectivity,this.refractionRatio=t.refractionRatio,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.wireframeLinecap=t.wireframeLinecap,this.wireframeLinejoin=t.wireframeLinejoin,this.fog=t.fog,this}}const Sc=new me,xi=new _a,mr=new js,yc=new F,gr=new F,_r=new F,xr=new F,Ka=new F,vr=new F,Ec=new F,Mr=new F;class Xn extends ke{constructor(t=new Ue,e=new Gh){super(),this.isMesh=!0,this.type="Mesh",this.geometry=t,this.material=e,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.count=1,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),t.morphTargetInfluences!==void 0&&(this.morphTargetInfluences=t.morphTargetInfluences.slice()),t.morphTargetDictionary!==void 0&&(this.morphTargetDictionary=Object.assign({},t.morphTargetDictionary)),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const s=e[i[0]];if(s!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let r=0,a=s.length;r<a;r++){const o=s[r].name||String(r);this.morphTargetInfluences.push(0),this.morphTargetDictionary[o]=r}}}}getVertexPosition(t,e){const i=this.geometry,s=i.attributes.position,r=i.morphAttributes.position,a=i.morphTargetsRelative;e.fromBufferAttribute(s,t);const o=this.morphTargetInfluences;if(r&&o){vr.set(0,0,0);for(let c=0,l=r.length;c<l;c++){const u=o[c],g=r[c];u!==0&&(Ka.fromBufferAttribute(g,t),a?vr.addScaledVector(Ka,u):vr.addScaledVector(Ka.sub(e),u))}e.add(vr)}return e}raycast(t,e){const i=this.geometry,s=this.material,r=this.matrixWorld;s!==void 0&&(i.boundingSphere===null&&i.computeBoundingSphere(),mr.copy(i.boundingSphere),mr.applyMatrix4(r),xi.copy(t.ray).recast(t.near),!(mr.containsPoint(xi.origin)===!1&&(xi.intersectSphere(mr,yc)===null||xi.origin.distanceToSquared(yc)>(t.far-t.near)**2))&&(Sc.copy(r).invert(),xi.copy(t.ray).applyMatrix4(Sc),!(i.boundingBox!==null&&xi.intersectsBox(i.boundingBox)===!1)&&this._computeIntersections(t,e,xi)))}_computeIntersections(t,e,i){let s;const r=this.geometry,a=this.material,o=r.index,c=r.attributes.position,l=r.attributes.uv,u=r.attributes.uv1,g=r.attributes.normal,h=r.groups,m=r.drawRange;if(o!==null)if(Array.isArray(a))for(let _=0,y=h.length;_<y;_++){const f=h[_],d=a[f.materialIndex],E=Math.max(f.start,m.start),A=Math.min(o.count,Math.min(f.start+f.count,m.start+m.count));for(let T=E,D=A;T<D;T+=3){const b=o.getX(T),N=o.getX(T+1),M=o.getX(T+2);s=Sr(this,d,t,i,l,u,g,b,N,M),s&&(s.faceIndex=Math.floor(T/3),s.face.materialIndex=f.materialIndex,e.push(s))}}else{const _=Math.max(0,m.start),y=Math.min(o.count,m.start+m.count);for(let f=_,d=y;f<d;f+=3){const E=o.getX(f),A=o.getX(f+1),T=o.getX(f+2);s=Sr(this,a,t,i,l,u,g,E,A,T),s&&(s.faceIndex=Math.floor(f/3),e.push(s))}}else if(c!==void 0)if(Array.isArray(a))for(let _=0,y=h.length;_<y;_++){const f=h[_],d=a[f.materialIndex],E=Math.max(f.start,m.start),A=Math.min(c.count,Math.min(f.start+f.count,m.start+m.count));for(let T=E,D=A;T<D;T+=3){const b=T,N=T+1,M=T+2;s=Sr(this,d,t,i,l,u,g,b,N,M),s&&(s.faceIndex=Math.floor(T/3),s.face.materialIndex=f.materialIndex,e.push(s))}}else{const _=Math.max(0,m.start),y=Math.min(c.count,m.start+m.count);for(let f=_,d=y;f<d;f+=3){const E=f,A=f+1,T=f+2;s=Sr(this,a,t,i,l,u,g,E,A,T),s&&(s.faceIndex=Math.floor(f/3),e.push(s))}}}}function af(n,t,e,i,s,r,a,o){let c;if(t.side===Ke?c=i.intersectTriangle(a,r,s,!0,o):c=i.intersectTriangle(s,r,a,t.side===ci,o),c===null)return null;Mr.copy(o),Mr.applyMatrix4(n.matrixWorld);const l=e.ray.origin.distanceTo(Mr);return l<e.near||l>e.far?null:{distance:l,point:Mr.clone(),object:n}}function Sr(n,t,e,i,s,r,a,o,c,l){n.getVertexPosition(o,gr),n.getVertexPosition(c,_r),n.getVertexPosition(l,xr);const u=af(n,t,e,i,gr,_r,xr,Ec);if(u){const g=new F;on.getBarycoord(Ec,gr,_r,xr,g),s&&(u.uv=on.getInterpolatedAttribute(s,o,c,l,g,new Nt)),r&&(u.uv1=on.getInterpolatedAttribute(r,o,c,l,g,new Nt)),a&&(u.normal=on.getInterpolatedAttribute(a,o,c,l,g,new F),u.normal.dot(i.direction)>0&&u.normal.multiplyScalar(-1));const h={a:o,b:c,c:l,normal:new F,materialIndex:0};on.getNormal(gr,_r,xr,h.normal),u.face=h,u.barycoord=g}return u}class of extends ze{constructor(t=null,e=1,i=1,s,r,a,o,c,l=Le,u=Le,g,h){super(null,a,o,c,l,u,s,r,g,h),this.isDataTexture=!0,this.image={data:t,width:e,height:i},this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}}const Za=new F,lf=new F,cf=new zt;class ii{constructor(t=new F(1,0,0),e=0){this.isPlane=!0,this.normal=t,this.constant=e}set(t,e){return this.normal.copy(t),this.constant=e,this}setComponents(t,e,i,s){return this.normal.set(t,e,i),this.constant=s,this}setFromNormalAndCoplanarPoint(t,e){return this.normal.copy(t),this.constant=-e.dot(this.normal),this}setFromCoplanarPoints(t,e,i){const s=Za.subVectors(i,e).cross(lf.subVectors(t,e)).normalize();return this.setFromNormalAndCoplanarPoint(s,t),this}copy(t){return this.normal.copy(t.normal),this.constant=t.constant,this}normalize(){const t=1/this.normal.length();return this.normal.multiplyScalar(t),this.constant*=t,this}negate(){return this.constant*=-1,this.normal.negate(),this}distanceToPoint(t){return this.normal.dot(t)+this.constant}distanceToSphere(t){return this.distanceToPoint(t.center)-t.radius}projectPoint(t,e){return e.copy(t).addScaledVector(this.normal,-this.distanceToPoint(t))}intersectLine(t,e,i=!0){const s=t.delta(Za),r=this.normal.dot(s);if(r===0)return this.distanceToPoint(t.start)===0?e.copy(t.start):null;const a=-(t.start.dot(this.normal)+this.constant)/r;return i===!0&&(a<0||a>1)?null:e.copy(t.start).addScaledVector(s,a)}intersectsLine(t){const e=this.distanceToPoint(t.start),i=this.distanceToPoint(t.end);return e<0&&i>0||i<0&&e>0}intersectsBox(t){return t.intersectsPlane(this)}intersectsSphere(t){return t.intersectsPlane(this)}coplanarPoint(t){return t.copy(this.normal).multiplyScalar(-this.constant)}applyMatrix4(t,e){const i=e||cf.getNormalMatrix(t),s=this.coplanarPoint(Za).applyMatrix4(t),r=this.normal.applyMatrix3(i).normalize();return this.constant=-s.dot(r),this}translate(t){return this.constant-=t.dot(this.normal),this}equals(t){return t.normal.equals(this.normal)&&t.constant===this.constant}clone(){return new this.constructor().copy(this)}}const vi=new js,hf=new Nt(.5,.5),yr=new F;class Vh{constructor(t=new ii,e=new ii,i=new ii,s=new ii,r=new ii,a=new ii){this.planes=[t,e,i,s,r,a]}set(t,e,i,s,r,a){const o=this.planes;return o[0].copy(t),o[1].copy(e),o[2].copy(i),o[3].copy(s),o[4].copy(r),o[5].copy(a),this}copy(t){const e=this.planes;for(let i=0;i<6;i++)e[i].copy(t.planes[i]);return this}setFromProjectionMatrix(t,e=wn,i=!1){const s=this.planes,r=t.elements,a=r[0],o=r[1],c=r[2],l=r[3],u=r[4],g=r[5],h=r[6],m=r[7],_=r[8],y=r[9],f=r[10],d=r[11],E=r[12],A=r[13],T=r[14],D=r[15];if(s[0].setComponents(l-a,m-u,d-_,D-E).normalize(),s[1].setComponents(l+a,m+u,d+_,D+E).normalize(),s[2].setComponents(l+o,m+g,d+y,D+A).normalize(),s[3].setComponents(l-o,m-g,d-y,D-A).normalize(),i)s[4].setComponents(c,h,f,T).normalize(),s[5].setComponents(l-c,m-h,d-f,D-T).normalize();else if(s[4].setComponents(l-c,m-h,d-f,D-T).normalize(),e===wn)s[5].setComponents(l+c,m+h,d+f,D+T).normalize();else if(e===ra)s[5].setComponents(c,h,f,T).normalize();else throw new Error("THREE.Frustum.setFromProjectionMatrix(): Invalid coordinate system: "+e);return this}intersectsObject(t){if(t.boundingSphere!==void 0)t.boundingSphere===null&&t.computeBoundingSphere(),vi.copy(t.boundingSphere).applyMatrix4(t.matrixWorld);else{const e=t.geometry;e.boundingSphere===null&&e.computeBoundingSphere(),vi.copy(e.boundingSphere).applyMatrix4(t.matrixWorld)}return this.intersectsSphere(vi)}intersectsSprite(t){vi.center.set(0,0,0);const e=hf.distanceTo(t.center);return vi.radius=.7071067811865476+e,vi.applyMatrix4(t.matrixWorld),this.intersectsSphere(vi)}intersectsSphere(t){const e=this.planes,i=t.center,s=-t.radius;for(let r=0;r<6;r++)if(e[r].distanceToPoint(i)<s)return!1;return!0}intersectsBox(t){const e=this.planes;for(let i=0;i<6;i++){const s=e[i];if(yr.x=s.normal.x>0?t.max.x:t.min.x,yr.y=s.normal.y>0?t.max.y:t.min.y,yr.z=s.normal.z>0?t.max.z:t.min.z,s.distanceToPoint(yr)<0)return!1}return!0}containsPoint(t){const e=this.planes;for(let i=0;i<6;i++)if(e[i].distanceToPoint(t)<0)return!1;return!0}clone(){return new this.constructor().copy(this)}}class wl extends Ii{constructor(t){super(),this.isLineBasicMaterial=!0,this.type="LineBasicMaterial",this.color=new Bt(16777215),this.map=null,this.linewidth=1,this.linecap="round",this.linejoin="round",this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.linewidth=t.linewidth,this.linecap=t.linecap,this.linejoin=t.linejoin,this.fog=t.fog,this}}const ca=new F,ha=new F,bc=new me,ws=new _a,Er=new js,Ja=new F,Tc=new F;class uf extends ke{constructor(t=new Ue,e=new wl){super(),this.isLine=!0,this.type="Line",this.geometry=t,this.material=e,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}computeLineDistances(){const t=this.geometry;if(t.index===null){const e=t.attributes.position,i=[0];for(let s=1,r=e.count;s<r;s++)ca.fromBufferAttribute(e,s-1),ha.fromBufferAttribute(e,s),i[s]=i[s-1],i[s]+=ca.distanceTo(ha);t.setAttribute("lineDistance",new mn(i,1))}else It("Line.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.");return this}raycast(t,e){const i=this.geometry,s=this.matrixWorld,r=t.params.Line.threshold,a=i.drawRange;if(i.boundingSphere===null&&i.computeBoundingSphere(),Er.copy(i.boundingSphere),Er.applyMatrix4(s),Er.radius+=r,t.ray.intersectsSphere(Er)===!1)return;bc.copy(s).invert(),ws.copy(t.ray).applyMatrix4(bc);const o=r/((this.scale.x+this.scale.y+this.scale.z)/3),c=o*o,l=this.isLineSegments?2:1,u=i.index,h=i.attributes.position;if(u!==null){const m=Math.max(0,a.start),_=Math.min(u.count,a.start+a.count);for(let y=m,f=_-1;y<f;y+=l){const d=u.getX(y),E=u.getX(y+1),A=br(this,t,ws,c,d,E,y);A&&e.push(A)}if(this.isLineLoop){const y=u.getX(_-1),f=u.getX(m),d=br(this,t,ws,c,y,f,_-1);d&&e.push(d)}}else{const m=Math.max(0,a.start),_=Math.min(h.count,a.start+a.count);for(let y=m,f=_-1;y<f;y+=l){const d=br(this,t,ws,c,y,y+1,y);d&&e.push(d)}if(this.isLineLoop){const y=br(this,t,ws,c,_-1,m,_-1);y&&e.push(y)}}}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const s=e[i[0]];if(s!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let r=0,a=s.length;r<a;r++){const o=s[r].name||String(r);this.morphTargetInfluences.push(0),this.morphTargetDictionary[o]=r}}}}}function br(n,t,e,i,s,r,a){const o=n.geometry.attributes.position;if(ca.fromBufferAttribute(o,s),ha.fromBufferAttribute(o,r),e.distanceSqToSegment(ca,ha,Ja,Tc)>i)return;Ja.applyMatrix4(n.matrixWorld);const l=t.ray.origin.distanceTo(Ja);if(!(l<t.near||l>t.far))return{distance:l,point:Tc.clone().applyMatrix4(n.matrixWorld),index:a,face:null,faceIndex:null,barycoord:null,object:n}}const Ac=new F,wc=new F;class Wh extends uf{constructor(t,e){super(t,e),this.isLineSegments=!0,this.type="LineSegments"}computeLineDistances(){const t=this.geometry;if(t.index===null){const e=t.attributes.position,i=[];for(let s=0,r=e.count;s<r;s+=2)Ac.fromBufferAttribute(e,s),wc.fromBufferAttribute(e,s+1),i[s]=s===0?0:i[s-1],i[s+1]=i[s]+Ac.distanceTo(wc);t.setAttribute("lineDistance",new mn(i,1))}else It("LineSegments.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.");return this}}class Rl extends Ii{constructor(t){super(),this.isPointsMaterial=!0,this.type="PointsMaterial",this.color=new Bt(16777215),this.map=null,this.alphaMap=null,this.size=1,this.sizeAttenuation=!0,this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.alphaMap=t.alphaMap,this.size=t.size,this.sizeAttenuation=t.sizeAttenuation,this.fog=t.fog,this}}const Rc=new me,tl=new _a,Tr=new js,Ar=new F;class Cl extends ke{constructor(t=new Ue,e=new Rl){super(),this.isPoints=!0,this.type="Points",this.geometry=t,this.material=e,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}raycast(t,e){const i=this.geometry,s=this.matrixWorld,r=t.params.Points.threshold,a=i.drawRange;if(i.boundingSphere===null&&i.computeBoundingSphere(),Tr.copy(i.boundingSphere),Tr.applyMatrix4(s),Tr.radius+=r,t.ray.intersectsSphere(Tr)===!1)return;Rc.copy(s).invert(),tl.copy(t.ray).applyMatrix4(Rc);const o=r/((this.scale.x+this.scale.y+this.scale.z)/3),c=o*o,l=i.index,g=i.attributes.position;if(l!==null){const h=Math.max(0,a.start),m=Math.min(l.count,a.start+a.count);for(let _=h,y=m;_<y;_++){const f=l.getX(_);Ar.fromBufferAttribute(g,f),Cc(Ar,f,c,s,t,e,this)}}else{const h=Math.max(0,a.start),m=Math.min(g.count,a.start+a.count);for(let _=h,y=m;_<y;_++)Ar.fromBufferAttribute(g,_),Cc(Ar,_,c,s,t,e,this)}}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const s=e[i[0]];if(s!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let r=0,a=s.length;r<a;r++){const o=s[r].name||String(r);this.morphTargetInfluences.push(0),this.morphTargetDictionary[o]=r}}}}}function Cc(n,t,e,i,s,r,a){const o=tl.distanceSqToPoint(n);if(o<e){const c=new F;tl.closestPointToPoint(n,c),c.applyMatrix4(i);const l=s.ray.origin.distanceTo(c);if(l<s.near||l>s.far)return;r.push({distance:l,distanceToRay:Math.sqrt(o),point:c,index:t,face:null,faceIndex:null,barycoord:null,object:a})}}class Xh extends ze{constructor(t=[],e=Pi,i,s,r,a,o,c,l,u){super(t,e,i,s,r,a,o,c,l,u),this.isCubeTexture=!0,this.flipY=!1}get images(){return this.image}set images(t){this.image=t}}class $h extends ze{constructor(t,e,i,s,r,a,o,c,l){super(t,e,i,s,r,a,o,c,l),this.isCanvasTexture=!0,this.needsUpdate=!0}}class gs extends ze{constructor(t,e,i=Ln,s,r,a,o=Le,c=Le,l,u=Wn,g=1){if(u!==Wn&&u!==Ai)throw new Error("DepthTexture format must be either THREE.DepthFormat or THREE.DepthStencilFormat");const h={width:t,height:e,depth:g};super(h,s,r,a,o,c,u,i,l),this.isDepthTexture=!0,this.flipY=!1,this.generateMipmaps=!1,this.compareFunction=null}copy(t){return super.copy(t),this.source=new Tl(Object.assign({},t.image)),this.compareFunction=t.compareFunction,this}toJSON(t){const e=super.toJSON(t);return this.compareFunction!==null&&(e.compareFunction=this.compareFunction),e}}class df extends gs{constructor(t,e=Ln,i=Pi,s,r,a=Le,o=Le,c,l=Wn){const u={width:t,height:t,depth:1},g=[u,u,u,u,u,u];super(t,t,e,i,s,r,a,o,c,l),this.image=g,this.isCubeDepthTexture=!0,this.isCubeTexture=!0}get images(){return this.image}set images(t){this.image=t}}class qh extends ze{constructor(t=null){super(),this.sourceTexture=t,this.isExternalTexture=!0}copy(t){return super.copy(t),this.sourceTexture=t.sourceTexture,this}}class Ks extends Ue{constructor(t=1,e=1,i=1,s=1,r=1,a=1){super(),this.type="BoxGeometry",this.parameters={width:t,height:e,depth:i,widthSegments:s,heightSegments:r,depthSegments:a};const o=this;s=Math.floor(s),r=Math.floor(r),a=Math.floor(a);const c=[],l=[],u=[],g=[];let h=0,m=0;_("z","y","x",-1,-1,i,e,t,a,r,0),_("z","y","x",1,-1,i,e,-t,a,r,1),_("x","z","y",1,1,t,i,e,s,a,2),_("x","z","y",1,-1,t,i,-e,s,a,3),_("x","y","z",1,-1,t,e,i,s,r,4),_("x","y","z",-1,-1,t,e,-i,s,r,5),this.setIndex(c),this.setAttribute("position",new mn(l,3)),this.setAttribute("normal",new mn(u,3)),this.setAttribute("uv",new mn(g,2));function _(y,f,d,E,A,T,D,b,N,M,w){const z=T/N,P=D/M,V=T/2,J=D/2,K=b/2,B=N+1,W=M+1;let $=0,et=0;const at=new F;for(let ct=0;ct<W;ct++){const yt=ct*P-J;for(let wt=0;wt<B;wt++){const kt=wt*z-V;at[y]=kt*E,at[f]=yt*A,at[d]=K,l.push(at.x,at.y,at.z),at[y]=0,at[f]=0,at[d]=b>0?1:-1,u.push(at.x,at.y,at.z),g.push(wt/N),g.push(1-ct/M),$+=1}}for(let ct=0;ct<M;ct++)for(let yt=0;yt<N;yt++){const wt=h+yt+B*ct,kt=h+yt+B*(ct+1),Qt=h+(yt+1)+B*(ct+1),C=h+(yt+1)+B*ct;c.push(wt,kt,C),c.push(kt,Qt,C),et+=6}o.addGroup(m,et,w),m+=et,h+=$}}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new Ks(t.width,t.height,t.depth,t.widthSegments,t.heightSegments,t.depthSegments)}}class xa extends Ue{constructor(t=1,e=1,i=1,s=1){super(),this.type="PlaneGeometry",this.parameters={width:t,height:e,widthSegments:i,heightSegments:s};const r=t/2,a=e/2,o=Math.floor(i),c=Math.floor(s),l=o+1,u=c+1,g=t/o,h=e/c,m=[],_=[],y=[],f=[];for(let d=0;d<u;d++){const E=d*h-a;for(let A=0;A<l;A++){const T=A*g-r;_.push(T,-E,0),y.push(0,0,1),f.push(A/o),f.push(1-d/c)}}for(let d=0;d<c;d++)for(let E=0;E<o;E++){const A=E+l*d,T=E+l*(d+1),D=E+1+l*(d+1),b=E+1+l*d;m.push(A,T,b),m.push(T,D,b)}this.setIndex(m),this.setAttribute("position",new mn(_,3)),this.setAttribute("normal",new mn(y,3)),this.setAttribute("uv",new mn(f,2))}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new xa(t.width,t.height,t.widthSegments,t.heightSegments)}}function _s(n){const t={};for(const e in n){t[e]={};for(const i in n[e]){const s=n[e][i];if(Pc(s))s.isRenderTargetTexture?(It("UniformsUtils: Textures of render targets cannot be cloned via cloneUniforms() or mergeUniforms()."),t[e][i]=null):t[e][i]=s.clone();else if(Array.isArray(s))if(Pc(s[0])){const r=[];for(let a=0,o=s.length;a<o;a++)r[a]=s[a].clone();t[e][i]=r}else t[e][i]=s.slice();else t[e][i]=s}}return t}function Ge(n){const t={};for(let e=0;e<n.length;e++){const i=_s(n[e]);for(const s in i)t[s]=i[s]}return t}function Pc(n){return n&&(n.isColor||n.isMatrix3||n.isMatrix4||n.isVector2||n.isVector3||n.isVector4||n.isTexture||n.isQuaternion)}function ff(n){const t=[];for(let e=0;e<n.length;e++)t.push(n[e].clone());return t}function Yh(n){const t=n.getRenderTarget();return t===null?n.outputColorSpace:t.isXRRenderTarget===!0?t.texture.colorSpace:jt.workingColorSpace}const pf={clone:_s,merge:Ge};var mf=`void main() {
	gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
}`,gf=`void main() {
	gl_FragColor = vec4( 1.0, 0.0, 0.0, 1.0 );
}`;class gn extends Ii{constructor(t){super(),this.isShaderMaterial=!0,this.type="ShaderMaterial",this.defines={},this.uniforms={},this.uniformsGroups=[],this.vertexShader=mf,this.fragmentShader=gf,this.linewidth=1,this.wireframe=!1,this.wireframeLinewidth=1,this.fog=!1,this.lights=!1,this.clipping=!1,this.forceSinglePass=!0,this.extensions={clipCullDistance:!1,multiDraw:!1},this.defaultAttributeValues={color:[1,1,1],uv:[0,0],uv1:[0,0]},this.index0AttributeName=void 0,this.uniformsNeedUpdate=!1,this.glslVersion=null,t!==void 0&&this.setValues(t)}copy(t){return super.copy(t),this.fragmentShader=t.fragmentShader,this.vertexShader=t.vertexShader,this.uniforms=_s(t.uniforms),this.uniformsGroups=ff(t.uniformsGroups),this.defines=Object.assign({},t.defines),this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.fog=t.fog,this.lights=t.lights,this.clipping=t.clipping,this.extensions=Object.assign({},t.extensions),this.glslVersion=t.glslVersion,this.defaultAttributeValues=Object.assign({},t.defaultAttributeValues),this.index0AttributeName=t.index0AttributeName,this.uniformsNeedUpdate=t.uniformsNeedUpdate,this}toJSON(t){const e=super.toJSON(t);e.glslVersion=this.glslVersion,e.uniforms={};for(const s in this.uniforms){const a=this.uniforms[s].value;a&&a.isTexture?e.uniforms[s]={type:"t",value:a.toJSON(t).uuid}:a&&a.isColor?e.uniforms[s]={type:"c",value:a.getHex()}:a&&a.isVector2?e.uniforms[s]={type:"v2",value:a.toArray()}:a&&a.isVector3?e.uniforms[s]={type:"v3",value:a.toArray()}:a&&a.isVector4?e.uniforms[s]={type:"v4",value:a.toArray()}:a&&a.isMatrix3?e.uniforms[s]={type:"m3",value:a.toArray()}:a&&a.isMatrix4?e.uniforms[s]={type:"m4",value:a.toArray()}:e.uniforms[s]={value:a}}Object.keys(this.defines).length>0&&(e.defines=this.defines),e.vertexShader=this.vertexShader,e.fragmentShader=this.fragmentShader,e.lights=this.lights,e.clipping=this.clipping;const i={};for(const s in this.extensions)this.extensions[s]===!0&&(i[s]=!0);return Object.keys(i).length>0&&(e.extensions=i),e}}class _f extends gn{constructor(t){super(t),this.isRawShaderMaterial=!0,this.type="RawShaderMaterial"}}class xf extends Ii{constructor(t){super(),this.isMeshDepthMaterial=!0,this.type="MeshDepthMaterial",this.depthPacking=Td,this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.wireframe=!1,this.wireframeLinewidth=1,this.setValues(t)}copy(t){return super.copy(t),this.depthPacking=t.depthPacking,this.map=t.map,this.alphaMap=t.alphaMap,this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this}}class vf extends Ii{constructor(t){super(),this.isMeshDistanceMaterial=!0,this.type="MeshDistanceMaterial",this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.setValues(t)}copy(t){return super.copy(t),this.map=t.map,this.alphaMap=t.alphaMap,this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this}}const wr=new F,Rr=new hi,Sn=new F;class jh extends ke{constructor(){super(),this.isCamera=!0,this.type="Camera",this.matrixWorldInverse=new me,this.projectionMatrix=new me,this.projectionMatrixInverse=new me,this.coordinateSystem=wn,this._reversedDepth=!1}get reversedDepth(){return this._reversedDepth}copy(t,e){return super.copy(t,e),this.matrixWorldInverse.copy(t.matrixWorldInverse),this.projectionMatrix.copy(t.projectionMatrix),this.projectionMatrixInverse.copy(t.projectionMatrixInverse),this.coordinateSystem=t.coordinateSystem,this}getWorldDirection(t){return super.getWorldDirection(t).negate()}updateMatrixWorld(t){super.updateMatrixWorld(t),this.matrixWorld.decompose(wr,Rr,Sn),Sn.x===1&&Sn.y===1&&Sn.z===1?this.matrixWorldInverse.copy(this.matrixWorld).invert():this.matrixWorldInverse.compose(wr,Rr,Sn.set(1,1,1)).invert()}updateWorldMatrix(t,e){super.updateWorldMatrix(t,e),this.matrixWorld.decompose(wr,Rr,Sn),Sn.x===1&&Sn.y===1&&Sn.z===1?this.matrixWorldInverse.copy(this.matrixWorld).invert():this.matrixWorldInverse.compose(wr,Rr,Sn.set(1,1,1)).invert()}clone(){return new this.constructor().copy(this)}}const ni=new F,Lc=new Nt,Dc=new Nt;class rn extends jh{constructor(t=50,e=1,i=.1,s=2e3){super(),this.isPerspectiveCamera=!0,this.type="PerspectiveCamera",this.fov=t,this.zoom=1,this.near=i,this.far=s,this.focus=10,this.aspect=e,this.view=null,this.filmGauge=35,this.filmOffset=0,this.updateProjectionMatrix()}copy(t,e){return super.copy(t,e),this.fov=t.fov,this.zoom=t.zoom,this.near=t.near,this.far=t.far,this.focus=t.focus,this.aspect=t.aspect,this.view=t.view===null?null:Object.assign({},t.view),this.filmGauge=t.filmGauge,this.filmOffset=t.filmOffset,this}setFocalLength(t){const e=.5*this.getFilmHeight()/t;this.fov=Qo*2*Math.atan(e),this.updateProjectionMatrix()}getFocalLength(){const t=Math.tan(Wr*.5*this.fov);return .5*this.getFilmHeight()/t}getEffectiveFOV(){return Qo*2*Math.atan(Math.tan(Wr*.5*this.fov)/this.zoom)}getFilmWidth(){return this.filmGauge*Math.min(this.aspect,1)}getFilmHeight(){return this.filmGauge/Math.max(this.aspect,1)}getViewBounds(t,e,i){ni.set(-1,-1,.5).applyMatrix4(this.projectionMatrixInverse),e.set(ni.x,ni.y).multiplyScalar(-t/ni.z),ni.set(1,1,.5).applyMatrix4(this.projectionMatrixInverse),i.set(ni.x,ni.y).multiplyScalar(-t/ni.z)}getViewSize(t,e){return this.getViewBounds(t,Lc,Dc),e.subVectors(Dc,Lc)}setViewOffset(t,e,i,s,r,a){this.aspect=t/e,this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=t,this.view.fullHeight=e,this.view.offsetX=i,this.view.offsetY=s,this.view.width=r,this.view.height=a,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const t=this.near;let e=t*Math.tan(Wr*.5*this.fov)/this.zoom,i=2*e,s=this.aspect*i,r=-.5*s;const a=this.view;if(this.view!==null&&this.view.enabled){const c=a.fullWidth,l=a.fullHeight;r+=a.offsetX*s/c,e-=a.offsetY*i/l,s*=a.width/c,i*=a.height/l}const o=this.filmOffset;o!==0&&(r+=t*o/this.getFilmWidth()),this.projectionMatrix.makePerspective(r,r+s,e,e-i,t,this.far,this.coordinateSystem,this.reversedDepth),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(t){const e=super.toJSON(t);return e.object.fov=this.fov,e.object.zoom=this.zoom,e.object.near=this.near,e.object.far=this.far,e.object.focus=this.focus,e.object.aspect=this.aspect,this.view!==null&&(e.object.view=Object.assign({},this.view)),e.object.filmGauge=this.filmGauge,e.object.filmOffset=this.filmOffset,e}}class Kh extends jh{constructor(t=-1,e=1,i=1,s=-1,r=.1,a=2e3){super(),this.isOrthographicCamera=!0,this.type="OrthographicCamera",this.zoom=1,this.view=null,this.left=t,this.right=e,this.top=i,this.bottom=s,this.near=r,this.far=a,this.updateProjectionMatrix()}copy(t,e){return super.copy(t,e),this.left=t.left,this.right=t.right,this.top=t.top,this.bottom=t.bottom,this.near=t.near,this.far=t.far,this.zoom=t.zoom,this.view=t.view===null?null:Object.assign({},t.view),this}setViewOffset(t,e,i,s,r,a){this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=t,this.view.fullHeight=e,this.view.offsetX=i,this.view.offsetY=s,this.view.width=r,this.view.height=a,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const t=(this.right-this.left)/(2*this.zoom),e=(this.top-this.bottom)/(2*this.zoom),i=(this.right+this.left)/2,s=(this.top+this.bottom)/2;let r=i-t,a=i+t,o=s+e,c=s-e;if(this.view!==null&&this.view.enabled){const l=(this.right-this.left)/this.view.fullWidth/this.zoom,u=(this.top-this.bottom)/this.view.fullHeight/this.zoom;r+=l*this.view.offsetX,a=r+l*this.view.width,o-=u*this.view.offsetY,c=o-u*this.view.height}this.projectionMatrix.makeOrthographic(r,a,o,c,this.near,this.far,this.coordinateSystem,this.reversedDepth),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(t){const e=super.toJSON(t);return e.object.zoom=this.zoom,e.object.left=this.left,e.object.right=this.right,e.object.top=this.top,e.object.bottom=this.bottom,e.object.near=this.near,e.object.far=this.far,this.view!==null&&(e.object.view=Object.assign({},this.view)),e}}const es=-90,ns=1;class Mf extends ke{constructor(t,e,i){super(),this.type="CubeCamera",this.renderTarget=i,this.coordinateSystem=null,this.activeMipmapLevel=0;const s=new rn(es,ns,t,e);s.layers=this.layers,this.add(s);const r=new rn(es,ns,t,e);r.layers=this.layers,this.add(r);const a=new rn(es,ns,t,e);a.layers=this.layers,this.add(a);const o=new rn(es,ns,t,e);o.layers=this.layers,this.add(o);const c=new rn(es,ns,t,e);c.layers=this.layers,this.add(c);const l=new rn(es,ns,t,e);l.layers=this.layers,this.add(l)}updateCoordinateSystem(){const t=this.coordinateSystem,e=this.children.concat(),[i,s,r,a,o,c]=e;for(const l of e)this.remove(l);if(t===wn)i.up.set(0,1,0),i.lookAt(1,0,0),s.up.set(0,1,0),s.lookAt(-1,0,0),r.up.set(0,0,-1),r.lookAt(0,1,0),a.up.set(0,0,1),a.lookAt(0,-1,0),o.up.set(0,1,0),o.lookAt(0,0,1),c.up.set(0,1,0),c.lookAt(0,0,-1);else if(t===ra)i.up.set(0,-1,0),i.lookAt(-1,0,0),s.up.set(0,-1,0),s.lookAt(1,0,0),r.up.set(0,0,1),r.lookAt(0,1,0),a.up.set(0,0,-1),a.lookAt(0,-1,0),o.up.set(0,-1,0),o.lookAt(0,0,1),c.up.set(0,-1,0),c.lookAt(0,0,-1);else throw new Error("THREE.CubeCamera.updateCoordinateSystem(): Invalid coordinate system: "+t);for(const l of e)this.add(l),l.updateMatrixWorld()}update(t,e){this.parent===null&&this.updateMatrixWorld();const{renderTarget:i,activeMipmapLevel:s}=this;this.coordinateSystem!==t.coordinateSystem&&(this.coordinateSystem=t.coordinateSystem,this.updateCoordinateSystem());const[r,a,o,c,l,u]=this.children,g=t.getRenderTarget(),h=t.getActiveCubeFace(),m=t.getActiveMipmapLevel(),_=t.xr.enabled;t.xr.enabled=!1;const y=i.texture.generateMipmaps;i.texture.generateMipmaps=!1;let f=!1;t.isWebGLRenderer===!0?f=t.state.buffers.depth.getReversed():f=t.reversedDepthBuffer,t.setRenderTarget(i,0,s),f&&t.autoClear===!1&&t.clearDepth(),t.render(e,r),t.setRenderTarget(i,1,s),f&&t.autoClear===!1&&t.clearDepth(),t.render(e,a),t.setRenderTarget(i,2,s),f&&t.autoClear===!1&&t.clearDepth(),t.render(e,o),t.setRenderTarget(i,3,s),f&&t.autoClear===!1&&t.clearDepth(),t.render(e,c),t.setRenderTarget(i,4,s),f&&t.autoClear===!1&&t.clearDepth(),t.render(e,l),i.texture.generateMipmaps=y,t.setRenderTarget(i,5,s),f&&t.autoClear===!1&&t.clearDepth(),t.render(e,u),t.setRenderTarget(g,h,m),t.xr.enabled=_,i.texture.needsPMREMUpdate=!0}}class Sf extends rn{constructor(t=[]){super(),this.isArrayCamera=!0,this.isMultiViewCamera=!1,this.cameras=t}}class Ic{constructor(t=1,e=0,i=0){this.radius=t,this.phi=e,this.theta=i}set(t,e,i){return this.radius=t,this.phi=e,this.theta=i,this}copy(t){return this.radius=t.radius,this.phi=t.phi,this.theta=t.theta,this}makeSafe(){return this.phi=Yt(this.phi,1e-6,Math.PI-1e-6),this}setFromVector3(t){return this.setFromCartesianCoords(t.x,t.y,t.z)}setFromCartesianCoords(t,e,i){return this.radius=Math.sqrt(t*t+e*e+i*i),this.radius===0?(this.theta=0,this.phi=0):(this.theta=Math.atan2(t,i),this.phi=Math.acos(Yt(e/this.radius,-1,1))),this}clone(){return new this.constructor().copy(this)}}class Zh{static{Zh.prototype.isMatrix2=!0}constructor(t,e,i,s){this.elements=[1,0,0,1],t!==void 0&&this.set(t,e,i,s)}identity(){return this.set(1,0,0,1),this}fromArray(t,e=0){for(let i=0;i<4;i++)this.elements[i]=t[i+e];return this}set(t,e,i,s){const r=this.elements;return r[0]=t,r[2]=e,r[1]=i,r[3]=s,this}}class yf extends fi{constructor(t,e=null){super(),this.object=t,this.domElement=e,this.enabled=!0,this.state=-1,this.keys={},this.mouseButtons={LEFT:null,MIDDLE:null,RIGHT:null},this.touches={ONE:null,TWO:null}}connect(t){if(t===void 0){It("Controls: connect() now requires an element.");return}this.domElement!==null&&this.disconnect(),this.domElement=t}disconnect(){}dispose(){}update(){}}function Uc(n,t,e,i){const s=Ef(i);switch(e){case Lh:return n*t;case Ih:return n*t/s.components*s.byteLength;case Ml:return n*t/s.components*s.byteLength;case Li:return n*t*2/s.components*s.byteLength;case Sl:return n*t*2/s.components*s.byteLength;case Dh:return n*t*3/s.components*s.byteLength;case pn:return n*t*4/s.components*s.byteLength;case yl:return n*t*4/s.components*s.byteLength;case kr:case Hr:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*8;case Gr:case Vr:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*16;case yo:case bo:return Math.max(n,16)*Math.max(t,8)/4;case So:case Eo:return Math.max(n,8)*Math.max(t,8)/2;case To:case Ao:case Ro:case Co:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*8;case wo:case ea:case Po:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*16;case Lo:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*16;case Do:return Math.floor((n+4)/5)*Math.floor((t+3)/4)*16;case Io:return Math.floor((n+4)/5)*Math.floor((t+4)/5)*16;case Uo:return Math.floor((n+5)/6)*Math.floor((t+4)/5)*16;case No:return Math.floor((n+5)/6)*Math.floor((t+5)/6)*16;case Fo:return Math.floor((n+7)/8)*Math.floor((t+4)/5)*16;case Oo:return Math.floor((n+7)/8)*Math.floor((t+5)/6)*16;case Bo:return Math.floor((n+7)/8)*Math.floor((t+7)/8)*16;case zo:return Math.floor((n+9)/10)*Math.floor((t+4)/5)*16;case ko:return Math.floor((n+9)/10)*Math.floor((t+5)/6)*16;case Ho:return Math.floor((n+9)/10)*Math.floor((t+7)/8)*16;case Go:return Math.floor((n+9)/10)*Math.floor((t+9)/10)*16;case Vo:return Math.floor((n+11)/12)*Math.floor((t+9)/10)*16;case Wo:return Math.floor((n+11)/12)*Math.floor((t+11)/12)*16;case Xo:case $o:case qo:return Math.ceil(n/4)*Math.ceil(t/4)*16;case Yo:case jo:return Math.ceil(n/4)*Math.ceil(t/4)*8;case na:case Ko:return Math.ceil(n/4)*Math.ceil(t/4)*16}throw new Error(`Unable to determine texture byte length for ${e} format.`)}function Ef(n){switch(n){case an:case wh:return{byteLength:1,components:1};case ks:case Rh:case Vn:return{byteLength:2,components:1};case xl:case vl:return{byteLength:2,components:4};case Ln:case _l:case An:return{byteLength:4,components:1};case Ch:case Ph:return{byteLength:4,components:3}}throw new Error(`Unknown texture type ${n}.`)}typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("register",{detail:{revision:gl}}));typeof window<"u"&&(window.__THREE__?It("WARNING: Multiple instances of Three.js being imported."):window.__THREE__=gl);/**
 * @license
 * Copyright 2010-2026 Three.js Authors
 * SPDX-License-Identifier: MIT
 */function Jh(){let n=null,t=!1,e=null,i=null;function s(r,a){e(r,a),i=n.requestAnimationFrame(s)}return{start:function(){t!==!0&&e!==null&&n!==null&&(i=n.requestAnimationFrame(s),t=!0)},stop:function(){n!==null&&n.cancelAnimationFrame(i),t=!1},setAnimationLoop:function(r){e=r},setContext:function(r){n=r}}}function bf(n){const t=new WeakMap;function e(o,c){const l=o.array,u=o.usage,g=l.byteLength,h=n.createBuffer();n.bindBuffer(c,h),n.bufferData(c,l,u),o.onUploadCallback();let m;if(l instanceof Float32Array)m=n.FLOAT;else if(typeof Float16Array<"u"&&l instanceof Float16Array)m=n.HALF_FLOAT;else if(l instanceof Uint16Array)o.isFloat16BufferAttribute?m=n.HALF_FLOAT:m=n.UNSIGNED_SHORT;else if(l instanceof Int16Array)m=n.SHORT;else if(l instanceof Uint32Array)m=n.UNSIGNED_INT;else if(l instanceof Int32Array)m=n.INT;else if(l instanceof Int8Array)m=n.BYTE;else if(l instanceof Uint8Array)m=n.UNSIGNED_BYTE;else if(l instanceof Uint8ClampedArray)m=n.UNSIGNED_BYTE;else throw new Error("THREE.WebGLAttributes: Unsupported buffer data format: "+l);return{buffer:h,type:m,bytesPerElement:l.BYTES_PER_ELEMENT,version:o.version,size:g}}function i(o,c,l){const u=c.array,g=c.updateRanges;if(n.bindBuffer(l,o),g.length===0)n.bufferSubData(l,0,u);else{g.sort((m,_)=>m.start-_.start);let h=0;for(let m=1;m<g.length;m++){const _=g[h],y=g[m];y.start<=_.start+_.count+1?_.count=Math.max(_.count,y.start+y.count-_.start):(++h,g[h]=y)}g.length=h+1;for(let m=0,_=g.length;m<_;m++){const y=g[m];n.bufferSubData(l,y.start*u.BYTES_PER_ELEMENT,u,y.start,y.count)}c.clearUpdateRanges()}c.onUploadCallback()}function s(o){return o.isInterleavedBufferAttribute&&(o=o.data),t.get(o)}function r(o){o.isInterleavedBufferAttribute&&(o=o.data);const c=t.get(o);c&&(n.deleteBuffer(c.buffer),t.delete(o))}function a(o,c){if(o.isInterleavedBufferAttribute&&(o=o.data),o.isGLBufferAttribute){const u=t.get(o);(!u||u.version<o.version)&&t.set(o,{buffer:o.buffer,type:o.type,bytesPerElement:o.elementSize,version:o.version});return}const l=t.get(o);if(l===void 0)t.set(o,e(o,c));else if(l.version<o.version){if(l.size!==o.array.byteLength)throw new Error("THREE.WebGLAttributes: The size of the buffer attribute's array buffer does not match the original size. Resizing buffer attributes is not supported.");i(l.buffer,o,c),l.version=o.version}}return{get:s,remove:r,update:a}}var Tf=`#ifdef USE_ALPHAHASH
	if ( diffuseColor.a < getAlphaHashThreshold( vPosition ) ) discard;
#endif`,Af=`#ifdef USE_ALPHAHASH
	const float ALPHA_HASH_SCALE = 0.05;
	float hash2D( vec2 value ) {
		return fract( 1.0e4 * sin( 17.0 * value.x + 0.1 * value.y ) * ( 0.1 + abs( sin( 13.0 * value.y + value.x ) ) ) );
	}
	float hash3D( vec3 value ) {
		return hash2D( vec2( hash2D( value.xy ), value.z ) );
	}
	float getAlphaHashThreshold( vec3 position ) {
		float maxDeriv = max(
			length( dFdx( position.xyz ) ),
			length( dFdy( position.xyz ) )
		);
		float pixScale = 1.0 / ( ALPHA_HASH_SCALE * maxDeriv );
		vec2 pixScales = vec2(
			exp2( floor( log2( pixScale ) ) ),
			exp2( ceil( log2( pixScale ) ) )
		);
		vec2 alpha = vec2(
			hash3D( floor( pixScales.x * position.xyz ) ),
			hash3D( floor( pixScales.y * position.xyz ) )
		);
		float lerpFactor = fract( log2( pixScale ) );
		float x = ( 1.0 - lerpFactor ) * alpha.x + lerpFactor * alpha.y;
		float a = min( lerpFactor, 1.0 - lerpFactor );
		vec3 cases = vec3(
			x * x / ( 2.0 * a * ( 1.0 - a ) ),
			( x - 0.5 * a ) / ( 1.0 - a ),
			1.0 - ( ( 1.0 - x ) * ( 1.0 - x ) / ( 2.0 * a * ( 1.0 - a ) ) )
		);
		float threshold = ( x < ( 1.0 - a ) )
			? ( ( x < a ) ? cases.x : cases.y )
			: cases.z;
		return clamp( threshold , 1.0e-6, 1.0 );
	}
#endif`,wf=`#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, vAlphaMapUv ).g;
#endif`,Rf=`#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,Cf=`#ifdef USE_ALPHATEST
	#ifdef ALPHA_TO_COVERAGE
	diffuseColor.a = smoothstep( alphaTest, alphaTest + fwidth( diffuseColor.a ), diffuseColor.a );
	if ( diffuseColor.a == 0.0 ) discard;
	#else
	if ( diffuseColor.a < alphaTest ) discard;
	#endif
#endif`,Pf=`#ifdef USE_ALPHATEST
	uniform float alphaTest;
#endif`,Lf=`#ifdef USE_AOMAP
	float ambientOcclusion = ( texture2D( aoMap, vAoMapUv ).r - 1.0 ) * aoMapIntensity + 1.0;
	reflectedLight.indirectDiffuse *= ambientOcclusion;
	#if defined( USE_CLEARCOAT ) 
		clearcoatSpecularIndirect *= ambientOcclusion;
	#endif
	#if defined( USE_SHEEN ) 
		sheenSpecularIndirect *= ambientOcclusion;
	#endif
	#if defined( USE_ENVMAP ) && defined( STANDARD )
		float dotNV = saturate( dot( geometryNormal, geometryViewDir ) );
		reflectedLight.indirectSpecular *= computeSpecularOcclusion( dotNV, ambientOcclusion, material.roughness );
	#endif
#endif`,Df=`#ifdef USE_AOMAP
	uniform sampler2D aoMap;
	uniform float aoMapIntensity;
#endif`,If=`#ifdef USE_BATCHING
	#if ! defined( GL_ANGLE_multi_draw )
	#define gl_DrawID _gl_DrawID
	uniform int _gl_DrawID;
	#endif
	uniform highp sampler2D batchingTexture;
	uniform highp usampler2D batchingIdTexture;
	mat4 getBatchingMatrix( const in float i ) {
		int size = textureSize( batchingTexture, 0 ).x;
		int j = int( i ) * 4;
		int x = j % size;
		int y = j / size;
		vec4 v1 = texelFetch( batchingTexture, ivec2( x, y ), 0 );
		vec4 v2 = texelFetch( batchingTexture, ivec2( x + 1, y ), 0 );
		vec4 v3 = texelFetch( batchingTexture, ivec2( x + 2, y ), 0 );
		vec4 v4 = texelFetch( batchingTexture, ivec2( x + 3, y ), 0 );
		return mat4( v1, v2, v3, v4 );
	}
	float getIndirectIndex( const in int i ) {
		int size = textureSize( batchingIdTexture, 0 ).x;
		int x = i % size;
		int y = i / size;
		return float( texelFetch( batchingIdTexture, ivec2( x, y ), 0 ).r );
	}
#endif
#ifdef USE_BATCHING_COLOR
	uniform sampler2D batchingColorTexture;
	vec4 getBatchingColor( const in float i ) {
		int size = textureSize( batchingColorTexture, 0 ).x;
		int j = int( i );
		int x = j % size;
		int y = j / size;
		return texelFetch( batchingColorTexture, ivec2( x, y ), 0 );
	}
#endif`,Uf=`#ifdef USE_BATCHING
	mat4 batchingMatrix = getBatchingMatrix( getIndirectIndex( gl_DrawID ) );
#endif`,Nf=`vec3 transformed = vec3( position );
#ifdef USE_ALPHAHASH
	vPosition = vec3( position );
#endif`,Ff=`vec3 objectNormal = vec3( normal );
#ifdef USE_TANGENT
	vec3 objectTangent = vec3( tangent.xyz );
#endif`,Of=`float G_BlinnPhong_Implicit( ) {
	return 0.25;
}
float D_BlinnPhong( const in float shininess, const in float dotNH ) {
	return RECIPROCAL_PI * ( shininess * 0.5 + 1.0 ) * pow( dotNH, shininess );
}
vec3 BRDF_BlinnPhong( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in vec3 specularColor, const in float shininess ) {
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNH = saturate( dot( normal, halfDir ) );
	float dotVH = saturate( dot( viewDir, halfDir ) );
	vec3 F = F_Schlick( specularColor, 1.0, dotVH );
	float G = G_BlinnPhong_Implicit( );
	float D = D_BlinnPhong( shininess, dotNH );
	return F * ( G * D );
} // validated`,Bf=`#ifdef USE_IRIDESCENCE
	const mat3 XYZ_TO_REC709 = mat3(
		 3.2404542, -0.9692660,  0.0556434,
		-1.5371385,  1.8760108, -0.2040259,
		-0.4985314,  0.0415560,  1.0572252
	);
	vec3 Fresnel0ToIor( vec3 fresnel0 ) {
		vec3 sqrtF0 = sqrt( fresnel0 );
		return ( vec3( 1.0 ) + sqrtF0 ) / ( vec3( 1.0 ) - sqrtF0 );
	}
	vec3 IorToFresnel0( vec3 transmittedIor, float incidentIor ) {
		return pow2( ( transmittedIor - vec3( incidentIor ) ) / ( transmittedIor + vec3( incidentIor ) ) );
	}
	float IorToFresnel0( float transmittedIor, float incidentIor ) {
		return pow2( ( transmittedIor - incidentIor ) / ( transmittedIor + incidentIor ));
	}
	vec3 evalSensitivity( float OPD, vec3 shift ) {
		float phase = 2.0 * PI * OPD * 1.0e-9;
		vec3 val = vec3( 5.4856e-13, 4.4201e-13, 5.2481e-13 );
		vec3 pos = vec3( 1.6810e+06, 1.7953e+06, 2.2084e+06 );
		vec3 var = vec3( 4.3278e+09, 9.3046e+09, 6.6121e+09 );
		vec3 xyz = val * sqrt( 2.0 * PI * var ) * cos( pos * phase + shift ) * exp( - pow2( phase ) * var );
		xyz.x += 9.7470e-14 * sqrt( 2.0 * PI * 4.5282e+09 ) * cos( 2.2399e+06 * phase + shift[ 0 ] ) * exp( - 4.5282e+09 * pow2( phase ) );
		xyz /= 1.0685e-7;
		vec3 rgb = XYZ_TO_REC709 * xyz;
		return rgb;
	}
	vec3 evalIridescence( float outsideIOR, float eta2, float cosTheta1, float thinFilmThickness, vec3 baseF0 ) {
		vec3 I;
		float iridescenceIOR = mix( outsideIOR, eta2, smoothstep( 0.0, 0.03, thinFilmThickness ) );
		float sinTheta2Sq = pow2( outsideIOR / iridescenceIOR ) * ( 1.0 - pow2( cosTheta1 ) );
		float cosTheta2Sq = 1.0 - sinTheta2Sq;
		if ( cosTheta2Sq < 0.0 ) {
			return vec3( 1.0 );
		}
		float cosTheta2 = sqrt( cosTheta2Sq );
		float R0 = IorToFresnel0( iridescenceIOR, outsideIOR );
		float R12 = F_Schlick( R0, 1.0, cosTheta1 );
		float T121 = 1.0 - R12;
		float phi12 = 0.0;
		if ( iridescenceIOR < outsideIOR ) phi12 = PI;
		float phi21 = PI - phi12;
		vec3 baseIOR = Fresnel0ToIor( clamp( baseF0, 0.0, 0.9999 ) );		vec3 R1 = IorToFresnel0( baseIOR, iridescenceIOR );
		vec3 R23 = F_Schlick( R1, 1.0, cosTheta2 );
		vec3 phi23 = vec3( 0.0 );
		if ( baseIOR[ 0 ] < iridescenceIOR ) phi23[ 0 ] = PI;
		if ( baseIOR[ 1 ] < iridescenceIOR ) phi23[ 1 ] = PI;
		if ( baseIOR[ 2 ] < iridescenceIOR ) phi23[ 2 ] = PI;
		float OPD = 2.0 * iridescenceIOR * thinFilmThickness * cosTheta2;
		vec3 phi = vec3( phi21 ) + phi23;
		vec3 R123 = clamp( R12 * R23, 1e-5, 0.9999 );
		vec3 r123 = sqrt( R123 );
		vec3 Rs = pow2( T121 ) * R23 / ( vec3( 1.0 ) - R123 );
		vec3 C0 = R12 + Rs;
		I = C0;
		vec3 Cm = Rs - T121;
		for ( int m = 1; m <= 2; ++ m ) {
			Cm *= r123;
			vec3 Sm = 2.0 * evalSensitivity( float( m ) * OPD, float( m ) * phi );
			I += Cm * Sm;
		}
		return max( I, vec3( 0.0 ) );
	}
#endif`,zf=`#ifdef USE_BUMPMAP
	uniform sampler2D bumpMap;
	uniform float bumpScale;
	vec2 dHdxy_fwd() {
		vec2 dSTdx = dFdx( vBumpMapUv );
		vec2 dSTdy = dFdy( vBumpMapUv );
		float Hll = bumpScale * texture2D( bumpMap, vBumpMapUv ).x;
		float dBx = bumpScale * texture2D( bumpMap, vBumpMapUv + dSTdx ).x - Hll;
		float dBy = bumpScale * texture2D( bumpMap, vBumpMapUv + dSTdy ).x - Hll;
		return vec2( dBx, dBy );
	}
	vec3 perturbNormalArb( vec3 surf_pos, vec3 surf_norm, vec2 dHdxy, float faceDirection ) {
		vec3 vSigmaX = normalize( dFdx( surf_pos.xyz ) );
		vec3 vSigmaY = normalize( dFdy( surf_pos.xyz ) );
		vec3 vN = surf_norm;
		vec3 R1 = cross( vSigmaY, vN );
		vec3 R2 = cross( vN, vSigmaX );
		float fDet = dot( vSigmaX, R1 ) * faceDirection;
		vec3 vGrad = sign( fDet ) * ( dHdxy.x * R1 + dHdxy.y * R2 );
		return normalize( abs( fDet ) * surf_norm - vGrad );
	}
#endif`,kf=`#if NUM_CLIPPING_PLANES > 0
	vec4 plane;
	#ifdef ALPHA_TO_COVERAGE
		float distanceToPlane, distanceGradient;
		float clipOpacity = 1.0;
		#pragma unroll_loop_start
		for ( int i = 0; i < UNION_CLIPPING_PLANES; i ++ ) {
			plane = clippingPlanes[ i ];
			distanceToPlane = - dot( vClipPosition, plane.xyz ) + plane.w;
			distanceGradient = fwidth( distanceToPlane ) / 2.0;
			clipOpacity *= smoothstep( - distanceGradient, distanceGradient, distanceToPlane );
			if ( clipOpacity == 0.0 ) discard;
		}
		#pragma unroll_loop_end
		#if UNION_CLIPPING_PLANES < NUM_CLIPPING_PLANES
			float unionClipOpacity = 1.0;
			#pragma unroll_loop_start
			for ( int i = UNION_CLIPPING_PLANES; i < NUM_CLIPPING_PLANES; i ++ ) {
				plane = clippingPlanes[ i ];
				distanceToPlane = - dot( vClipPosition, plane.xyz ) + plane.w;
				distanceGradient = fwidth( distanceToPlane ) / 2.0;
				unionClipOpacity *= 1.0 - smoothstep( - distanceGradient, distanceGradient, distanceToPlane );
			}
			#pragma unroll_loop_end
			clipOpacity *= 1.0 - unionClipOpacity;
		#endif
		diffuseColor.a *= clipOpacity;
		if ( diffuseColor.a == 0.0 ) discard;
	#else
		#pragma unroll_loop_start
		for ( int i = 0; i < UNION_CLIPPING_PLANES; i ++ ) {
			plane = clippingPlanes[ i ];
			if ( dot( vClipPosition, plane.xyz ) > plane.w ) discard;
		}
		#pragma unroll_loop_end
		#if UNION_CLIPPING_PLANES < NUM_CLIPPING_PLANES
			bool clipped = true;
			#pragma unroll_loop_start
			for ( int i = UNION_CLIPPING_PLANES; i < NUM_CLIPPING_PLANES; i ++ ) {
				plane = clippingPlanes[ i ];
				clipped = ( dot( vClipPosition, plane.xyz ) > plane.w ) && clipped;
			}
			#pragma unroll_loop_end
			if ( clipped ) discard;
		#endif
	#endif
#endif`,Hf=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
	uniform vec4 clippingPlanes[ NUM_CLIPPING_PLANES ];
#endif`,Gf=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
#endif`,Vf=`#if NUM_CLIPPING_PLANES > 0
	vClipPosition = - mvPosition.xyz;
#endif`,Wf=`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )
	diffuseColor *= vColor;
#endif`,Xf=`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )
	varying vec4 vColor;
#endif`,$f=`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA ) || defined( USE_INSTANCING_COLOR ) || defined( USE_BATCHING_COLOR )
	varying vec4 vColor;
#endif`,qf=`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA ) || defined( USE_INSTANCING_COLOR ) || defined( USE_BATCHING_COLOR )
	vColor = vec4( 1.0 );
#endif
#ifdef USE_COLOR_ALPHA
	vColor *= color;
#elif defined( USE_COLOR )
	vColor.rgb *= color;
#endif
#ifdef USE_INSTANCING_COLOR
	vColor.rgb *= instanceColor.rgb;
#endif
#ifdef USE_BATCHING_COLOR
	vColor *= getBatchingColor( getIndirectIndex( gl_DrawID ) );
#endif`,Yf=`#define PI 3.141592653589793
#define PI2 6.283185307179586
#define PI_HALF 1.5707963267948966
#define RECIPROCAL_PI 0.3183098861837907
#define RECIPROCAL_PI2 0.15915494309189535
#define EPSILON 1e-6
#ifndef saturate
#define saturate( a ) clamp( a, 0.0, 1.0 )
#endif
#define whiteComplement( a ) ( 1.0 - saturate( a ) )
float pow2( const in float x ) { return x*x; }
vec3 pow2( const in vec3 x ) { return x*x; }
float pow3( const in float x ) { return x*x*x; }
float pow4( const in float x ) { float x2 = x*x; return x2*x2; }
float max3( const in vec3 v ) { return max( max( v.x, v.y ), v.z ); }
float average( const in vec3 v ) { return dot( v, vec3( 0.3333333 ) ); }
highp float rand( const in vec2 uv ) {
	const highp float a = 12.9898, b = 78.233, c = 43758.5453;
	highp float dt = dot( uv.xy, vec2( a,b ) ), sn = mod( dt, PI );
	return fract( sin( sn ) * c );
}
#ifdef HIGH_PRECISION
	float precisionSafeLength( vec3 v ) { return length( v ); }
#else
	float precisionSafeLength( vec3 v ) {
		float maxComponent = max3( abs( v ) );
		return length( v / maxComponent ) * maxComponent;
	}
#endif
struct IncidentLight {
	vec3 color;
	vec3 direction;
	bool visible;
};
struct ReflectedLight {
	vec3 directDiffuse;
	vec3 directSpecular;
	vec3 indirectDiffuse;
	vec3 indirectSpecular;
};
#ifdef USE_ALPHAHASH
	varying vec3 vPosition;
#endif
vec3 transformDirection( in vec3 dir, in mat4 matrix ) {
	return normalize( ( matrix * vec4( dir, 0.0 ) ).xyz );
}
vec3 inverseTransformDirection( in vec3 dir, in mat4 matrix ) {
	return normalize( ( vec4( dir, 0.0 ) * matrix ).xyz );
}
bool isPerspectiveMatrix( mat4 m ) {
	return m[ 2 ][ 3 ] == - 1.0;
}
vec2 equirectUv( in vec3 dir ) {
	float u = atan( dir.z, dir.x ) * RECIPROCAL_PI2 + 0.5;
	float v = asin( clamp( dir.y, - 1.0, 1.0 ) ) * RECIPROCAL_PI + 0.5;
	return vec2( u, v );
}
vec3 BRDF_Lambert( const in vec3 diffuseColor ) {
	return RECIPROCAL_PI * diffuseColor;
}
vec3 F_Schlick( const in vec3 f0, const in float f90, const in float dotVH ) {
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );
	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );
}
float F_Schlick( const in float f0, const in float f90, const in float dotVH ) {
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );
	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );
} // validated`,jf=`#ifdef ENVMAP_TYPE_CUBE_UV
	#define cubeUV_minMipLevel 4.0
	#define cubeUV_minTileSize 16.0
	float getFace( vec3 direction ) {
		vec3 absDirection = abs( direction );
		float face = - 1.0;
		if ( absDirection.x > absDirection.z ) {
			if ( absDirection.x > absDirection.y )
				face = direction.x > 0.0 ? 0.0 : 3.0;
			else
				face = direction.y > 0.0 ? 1.0 : 4.0;
		} else {
			if ( absDirection.z > absDirection.y )
				face = direction.z > 0.0 ? 2.0 : 5.0;
			else
				face = direction.y > 0.0 ? 1.0 : 4.0;
		}
		return face;
	}
	vec2 getUV( vec3 direction, float face ) {
		vec2 uv;
		if ( face == 0.0 ) {
			uv = vec2( direction.z, direction.y ) / abs( direction.x );
		} else if ( face == 1.0 ) {
			uv = vec2( - direction.x, - direction.z ) / abs( direction.y );
		} else if ( face == 2.0 ) {
			uv = vec2( - direction.x, direction.y ) / abs( direction.z );
		} else if ( face == 3.0 ) {
			uv = vec2( - direction.z, direction.y ) / abs( direction.x );
		} else if ( face == 4.0 ) {
			uv = vec2( - direction.x, direction.z ) / abs( direction.y );
		} else {
			uv = vec2( direction.x, direction.y ) / abs( direction.z );
		}
		return 0.5 * ( uv + 1.0 );
	}
	vec3 bilinearCubeUV( sampler2D envMap, vec3 direction, float mipInt ) {
		float face = getFace( direction );
		float filterInt = max( cubeUV_minMipLevel - mipInt, 0.0 );
		mipInt = max( mipInt, cubeUV_minMipLevel );
		float faceSize = exp2( mipInt );
		highp vec2 uv = getUV( direction, face ) * ( faceSize - 2.0 ) + 1.0;
		if ( face > 2.0 ) {
			uv.y += faceSize;
			face -= 3.0;
		}
		uv.x += face * faceSize;
		uv.x += filterInt * 3.0 * cubeUV_minTileSize;
		uv.y += 4.0 * ( exp2( CUBEUV_MAX_MIP ) - faceSize );
		uv.x *= CUBEUV_TEXEL_WIDTH;
		uv.y *= CUBEUV_TEXEL_HEIGHT;
		#ifdef texture2DGradEXT
			return texture2DGradEXT( envMap, uv, vec2( 0.0 ), vec2( 0.0 ) ).rgb;
		#else
			return texture2D( envMap, uv ).rgb;
		#endif
	}
	#define cubeUV_r0 1.0
	#define cubeUV_m0 - 2.0
	#define cubeUV_r1 0.8
	#define cubeUV_m1 - 1.0
	#define cubeUV_r4 0.4
	#define cubeUV_m4 2.0
	#define cubeUV_r5 0.305
	#define cubeUV_m5 3.0
	#define cubeUV_r6 0.21
	#define cubeUV_m6 4.0
	float roughnessToMip( float roughness ) {
		float mip = 0.0;
		if ( roughness >= cubeUV_r1 ) {
			mip = ( cubeUV_r0 - roughness ) * ( cubeUV_m1 - cubeUV_m0 ) / ( cubeUV_r0 - cubeUV_r1 ) + cubeUV_m0;
		} else if ( roughness >= cubeUV_r4 ) {
			mip = ( cubeUV_r1 - roughness ) * ( cubeUV_m4 - cubeUV_m1 ) / ( cubeUV_r1 - cubeUV_r4 ) + cubeUV_m1;
		} else if ( roughness >= cubeUV_r5 ) {
			mip = ( cubeUV_r4 - roughness ) * ( cubeUV_m5 - cubeUV_m4 ) / ( cubeUV_r4 - cubeUV_r5 ) + cubeUV_m4;
		} else if ( roughness >= cubeUV_r6 ) {
			mip = ( cubeUV_r5 - roughness ) * ( cubeUV_m6 - cubeUV_m5 ) / ( cubeUV_r5 - cubeUV_r6 ) + cubeUV_m5;
		} else {
			mip = - 2.0 * log2( 1.16 * roughness );		}
		return mip;
	}
	vec4 textureCubeUV( sampler2D envMap, vec3 sampleDir, float roughness ) {
		float mip = clamp( roughnessToMip( roughness ), cubeUV_m0, CUBEUV_MAX_MIP );
		float mipF = fract( mip );
		float mipInt = floor( mip );
		vec3 color0 = bilinearCubeUV( envMap, sampleDir, mipInt );
		if ( mipF == 0.0 ) {
			return vec4( color0, 1.0 );
		} else {
			vec3 color1 = bilinearCubeUV( envMap, sampleDir, mipInt + 1.0 );
			return vec4( mix( color0, color1, mipF ), 1.0 );
		}
	}
#endif`,Kf=`vec3 transformedNormal = objectNormal;
#ifdef USE_TANGENT
	vec3 transformedTangent = objectTangent;
#endif
#ifdef USE_BATCHING
	mat3 bm = mat3( batchingMatrix );
	transformedNormal /= vec3( dot( bm[ 0 ], bm[ 0 ] ), dot( bm[ 1 ], bm[ 1 ] ), dot( bm[ 2 ], bm[ 2 ] ) );
	transformedNormal = bm * transformedNormal;
	#ifdef USE_TANGENT
		transformedTangent = bm * transformedTangent;
	#endif
#endif
#ifdef USE_INSTANCING
	mat3 im = mat3( instanceMatrix );
	transformedNormal /= vec3( dot( im[ 0 ], im[ 0 ] ), dot( im[ 1 ], im[ 1 ] ), dot( im[ 2 ], im[ 2 ] ) );
	transformedNormal = im * transformedNormal;
	#ifdef USE_TANGENT
		transformedTangent = im * transformedTangent;
	#endif
#endif
transformedNormal = normalMatrix * transformedNormal;
#ifdef FLIP_SIDED
	transformedNormal = - transformedNormal;
#endif
#ifdef USE_TANGENT
	transformedTangent = ( modelViewMatrix * vec4( transformedTangent, 0.0 ) ).xyz;
	#ifdef FLIP_SIDED
		transformedTangent = - transformedTangent;
	#endif
#endif`,Zf=`#ifdef USE_DISPLACEMENTMAP
	uniform sampler2D displacementMap;
	uniform float displacementScale;
	uniform float displacementBias;
#endif`,Jf=`#ifdef USE_DISPLACEMENTMAP
	transformed += normalize( objectNormal ) * ( texture2D( displacementMap, vDisplacementMapUv ).x * displacementScale + displacementBias );
#endif`,Qf=`#ifdef USE_EMISSIVEMAP
	vec4 emissiveColor = texture2D( emissiveMap, vEmissiveMapUv );
	#ifdef DECODE_VIDEO_TEXTURE_EMISSIVE
		emissiveColor = sRGBTransferEOTF( emissiveColor );
	#endif
	totalEmissiveRadiance *= emissiveColor.rgb;
#endif`,tp=`#ifdef USE_EMISSIVEMAP
	uniform sampler2D emissiveMap;
#endif`,ep="gl_FragColor = linearToOutputTexel( gl_FragColor );",np=`vec4 LinearTransferOETF( in vec4 value ) {
	return value;
}
vec4 sRGBTransferEOTF( in vec4 value ) {
	return vec4( mix( pow( value.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), value.rgb * 0.0773993808, vec3( lessThanEqual( value.rgb, vec3( 0.04045 ) ) ) ), value.a );
}
vec4 sRGBTransferOETF( in vec4 value ) {
	return vec4( mix( pow( value.rgb, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), value.rgb * 12.92, vec3( lessThanEqual( value.rgb, vec3( 0.0031308 ) ) ) ), value.a );
}`,ip=`#ifdef USE_ENVMAP
	#ifdef ENV_WORLDPOS
		vec3 cameraToFrag;
		if ( isOrthographic ) {
			cameraToFrag = normalize( vec3( - viewMatrix[ 0 ][ 2 ], - viewMatrix[ 1 ][ 2 ], - viewMatrix[ 2 ][ 2 ] ) );
		} else {
			cameraToFrag = normalize( vWorldPosition - cameraPosition );
		}
		vec3 worldNormal = inverseTransformDirection( normal, viewMatrix );
		#ifdef ENVMAP_MODE_REFLECTION
			vec3 reflectVec = reflect( cameraToFrag, worldNormal );
		#else
			vec3 reflectVec = refract( cameraToFrag, worldNormal, refractionRatio );
		#endif
	#else
		vec3 reflectVec = vReflect;
	#endif
	#ifdef ENVMAP_TYPE_CUBE
		vec4 envColor = textureCube( envMap, envMapRotation * reflectVec );
		#ifdef ENVMAP_BLENDING_MULTIPLY
			outgoingLight = mix( outgoingLight, outgoingLight * envColor.xyz, specularStrength * reflectivity );
		#elif defined( ENVMAP_BLENDING_MIX )
			outgoingLight = mix( outgoingLight, envColor.xyz, specularStrength * reflectivity );
		#elif defined( ENVMAP_BLENDING_ADD )
			outgoingLight += envColor.xyz * specularStrength * reflectivity;
		#endif
	#endif
#endif`,sp=`#ifdef USE_ENVMAP
	uniform float envMapIntensity;
	uniform mat3 envMapRotation;
	#ifdef ENVMAP_TYPE_CUBE
		uniform samplerCube envMap;
	#else
		uniform sampler2D envMap;
	#endif
#endif`,rp=`#ifdef USE_ENVMAP
	uniform float reflectivity;
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		varying vec3 vWorldPosition;
		uniform float refractionRatio;
	#else
		varying vec3 vReflect;
	#endif
#endif`,ap=`#ifdef USE_ENVMAP
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		
		varying vec3 vWorldPosition;
	#else
		varying vec3 vReflect;
		uniform float refractionRatio;
	#endif
#endif`,op=`#ifdef USE_ENVMAP
	#ifdef ENV_WORLDPOS
		vWorldPosition = worldPosition.xyz;
	#else
		vec3 cameraToVertex;
		if ( isOrthographic ) {
			cameraToVertex = normalize( vec3( - viewMatrix[ 0 ][ 2 ], - viewMatrix[ 1 ][ 2 ], - viewMatrix[ 2 ][ 2 ] ) );
		} else {
			cameraToVertex = normalize( worldPosition.xyz - cameraPosition );
		}
		vec3 worldNormal = inverseTransformDirection( transformedNormal, viewMatrix );
		#ifdef ENVMAP_MODE_REFLECTION
			vReflect = reflect( cameraToVertex, worldNormal );
		#else
			vReflect = refract( cameraToVertex, worldNormal, refractionRatio );
		#endif
	#endif
#endif`,lp=`#ifdef USE_FOG
	vFogDepth = - mvPosition.z;
#endif`,cp=`#ifdef USE_FOG
	varying float vFogDepth;
#endif`,hp=`#ifdef USE_FOG
	#ifdef FOG_EXP2
		float fogFactor = 1.0 - exp( - fogDensity * fogDensity * vFogDepth * vFogDepth );
	#else
		float fogFactor = smoothstep( fogNear, fogFar, vFogDepth );
	#endif
	gl_FragColor.rgb = mix( gl_FragColor.rgb, fogColor, fogFactor );
#endif`,up=`#ifdef USE_FOG
	uniform vec3 fogColor;
	varying float vFogDepth;
	#ifdef FOG_EXP2
		uniform float fogDensity;
	#else
		uniform float fogNear;
		uniform float fogFar;
	#endif
#endif`,dp=`#ifdef USE_GRADIENTMAP
	uniform sampler2D gradientMap;
#endif
vec3 getGradientIrradiance( vec3 normal, vec3 lightDirection ) {
	float dotNL = dot( normal, lightDirection );
	vec2 coord = vec2( dotNL * 0.5 + 0.5, 0.0 );
	#ifdef USE_GRADIENTMAP
		return vec3( texture2D( gradientMap, coord ).r );
	#else
		vec2 fw = fwidth( coord ) * 0.5;
		return mix( vec3( 0.7 ), vec3( 1.0 ), smoothstep( 0.7 - fw.x, 0.7 + fw.x, coord.x ) );
	#endif
}`,fp=`#ifdef USE_LIGHTMAP
	uniform sampler2D lightMap;
	uniform float lightMapIntensity;
#endif`,pp=`LambertMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularStrength = specularStrength;`,mp=`varying vec3 vViewPosition;
struct LambertMaterial {
	vec3 diffuseColor;
	float specularStrength;
};
void RE_Direct_Lambert( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in LambertMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Lambert( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in LambertMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_Lambert
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Lambert`,gp=`uniform bool receiveShadow;
uniform vec3 ambientLightColor;
#if defined( USE_LIGHT_PROBES )
	uniform vec3 lightProbe[ 9 ];
#endif
vec3 shGetIrradianceAt( in vec3 normal, in vec3 shCoefficients[ 9 ] ) {
	float x = normal.x, y = normal.y, z = normal.z;
	vec3 result = shCoefficients[ 0 ] * 0.886227;
	result += shCoefficients[ 1 ] * 2.0 * 0.511664 * y;
	result += shCoefficients[ 2 ] * 2.0 * 0.511664 * z;
	result += shCoefficients[ 3 ] * 2.0 * 0.511664 * x;
	result += shCoefficients[ 4 ] * 2.0 * 0.429043 * x * y;
	result += shCoefficients[ 5 ] * 2.0 * 0.429043 * y * z;
	result += shCoefficients[ 6 ] * ( 0.743125 * z * z - 0.247708 );
	result += shCoefficients[ 7 ] * 2.0 * 0.429043 * x * z;
	result += shCoefficients[ 8 ] * 0.429043 * ( x * x - y * y );
	return result;
}
vec3 getLightProbeIrradiance( const in vec3 lightProbe[ 9 ], const in vec3 normal ) {
	vec3 worldNormal = inverseTransformDirection( normal, viewMatrix );
	vec3 irradiance = shGetIrradianceAt( worldNormal, lightProbe );
	return irradiance;
}
vec3 getAmbientLightIrradiance( const in vec3 ambientLightColor ) {
	vec3 irradiance = ambientLightColor;
	return irradiance;
}
float getDistanceAttenuation( const in float lightDistance, const in float cutoffDistance, const in float decayExponent ) {
	float distanceFalloff = 1.0 / max( pow( lightDistance, decayExponent ), 0.01 );
	if ( cutoffDistance > 0.0 ) {
		distanceFalloff *= pow2( saturate( 1.0 - pow4( lightDistance / cutoffDistance ) ) );
	}
	return distanceFalloff;
}
float getSpotAttenuation( const in float coneCosine, const in float penumbraCosine, const in float angleCosine ) {
	return smoothstep( coneCosine, penumbraCosine, angleCosine );
}
#if NUM_DIR_LIGHTS > 0
	struct DirectionalLight {
		vec3 direction;
		vec3 color;
	};
	uniform DirectionalLight directionalLights[ NUM_DIR_LIGHTS ];
	void getDirectionalLightInfo( const in DirectionalLight directionalLight, out IncidentLight light ) {
		light.color = directionalLight.color;
		light.direction = directionalLight.direction;
		light.visible = true;
	}
#endif
#if NUM_POINT_LIGHTS > 0
	struct PointLight {
		vec3 position;
		vec3 color;
		float distance;
		float decay;
	};
	uniform PointLight pointLights[ NUM_POINT_LIGHTS ];
	void getPointLightInfo( const in PointLight pointLight, const in vec3 geometryPosition, out IncidentLight light ) {
		vec3 lVector = pointLight.position - geometryPosition;
		light.direction = normalize( lVector );
		float lightDistance = length( lVector );
		light.color = pointLight.color;
		light.color *= getDistanceAttenuation( lightDistance, pointLight.distance, pointLight.decay );
		light.visible = ( light.color != vec3( 0.0 ) );
	}
#endif
#if NUM_SPOT_LIGHTS > 0
	struct SpotLight {
		vec3 position;
		vec3 direction;
		vec3 color;
		float distance;
		float decay;
		float coneCos;
		float penumbraCos;
	};
	uniform SpotLight spotLights[ NUM_SPOT_LIGHTS ];
	void getSpotLightInfo( const in SpotLight spotLight, const in vec3 geometryPosition, out IncidentLight light ) {
		vec3 lVector = spotLight.position - geometryPosition;
		light.direction = normalize( lVector );
		float angleCos = dot( light.direction, spotLight.direction );
		float spotAttenuation = getSpotAttenuation( spotLight.coneCos, spotLight.penumbraCos, angleCos );
		if ( spotAttenuation > 0.0 ) {
			float lightDistance = length( lVector );
			light.color = spotLight.color * spotAttenuation;
			light.color *= getDistanceAttenuation( lightDistance, spotLight.distance, spotLight.decay );
			light.visible = ( light.color != vec3( 0.0 ) );
		} else {
			light.color = vec3( 0.0 );
			light.visible = false;
		}
	}
#endif
#if NUM_RECT_AREA_LIGHTS > 0
	struct RectAreaLight {
		vec3 color;
		vec3 position;
		vec3 halfWidth;
		vec3 halfHeight;
	};
	uniform sampler2D ltc_1;	uniform sampler2D ltc_2;
	uniform RectAreaLight rectAreaLights[ NUM_RECT_AREA_LIGHTS ];
#endif
#if NUM_HEMI_LIGHTS > 0
	struct HemisphereLight {
		vec3 direction;
		vec3 skyColor;
		vec3 groundColor;
	};
	uniform HemisphereLight hemisphereLights[ NUM_HEMI_LIGHTS ];
	vec3 getHemisphereLightIrradiance( const in HemisphereLight hemiLight, const in vec3 normal ) {
		float dotNL = dot( normal, hemiLight.direction );
		float hemiDiffuseWeight = 0.5 * dotNL + 0.5;
		vec3 irradiance = mix( hemiLight.groundColor, hemiLight.skyColor, hemiDiffuseWeight );
		return irradiance;
	}
#endif
#include <lightprobes_pars_fragment>`,_p=`#ifdef USE_ENVMAP
	vec3 getIBLIrradiance( const in vec3 normal ) {
		#ifdef ENVMAP_TYPE_CUBE_UV
			vec3 worldNormal = inverseTransformDirection( normal, viewMatrix );
			vec4 envMapColor = textureCubeUV( envMap, envMapRotation * worldNormal, 1.0 );
			return PI * envMapColor.rgb * envMapIntensity;
		#else
			return vec3( 0.0 );
		#endif
	}
	vec3 getIBLRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness ) {
		#ifdef ENVMAP_TYPE_CUBE_UV
			vec3 reflectVec = reflect( - viewDir, normal );
			reflectVec = normalize( mix( reflectVec, normal, pow4( roughness ) ) );
			reflectVec = inverseTransformDirection( reflectVec, viewMatrix );
			vec4 envMapColor = textureCubeUV( envMap, envMapRotation * reflectVec, roughness );
			return envMapColor.rgb * envMapIntensity;
		#else
			return vec3( 0.0 );
		#endif
	}
	#ifdef USE_ANISOTROPY
		vec3 getIBLAnisotropyRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness, const in vec3 bitangent, const in float anisotropy ) {
			#ifdef ENVMAP_TYPE_CUBE_UV
				vec3 bentNormal = cross( bitangent, viewDir );
				bentNormal = normalize( cross( bentNormal, bitangent ) );
				bentNormal = normalize( mix( bentNormal, normal, pow2( pow2( 1.0 - anisotropy * ( 1.0 - roughness ) ) ) ) );
				return getIBLRadiance( viewDir, bentNormal, roughness );
			#else
				return vec3( 0.0 );
			#endif
		}
	#endif
#endif`,xp=`ToonMaterial material;
material.diffuseColor = diffuseColor.rgb;`,vp=`varying vec3 vViewPosition;
struct ToonMaterial {
	vec3 diffuseColor;
};
void RE_Direct_Toon( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in ToonMaterial material, inout ReflectedLight reflectedLight ) {
	vec3 irradiance = getGradientIrradiance( geometryNormal, directLight.direction ) * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Toon( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in ToonMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_Toon
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Toon`,Mp=`BlinnPhongMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularColor = specular;
material.specularShininess = shininess;
material.specularStrength = specularStrength;`,Sp=`varying vec3 vViewPosition;
struct BlinnPhongMaterial {
	vec3 diffuseColor;
	vec3 specularColor;
	float specularShininess;
	float specularStrength;
};
void RE_Direct_BlinnPhong( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
	reflectedLight.directSpecular += irradiance * BRDF_BlinnPhong( directLight.direction, geometryViewDir, geometryNormal, material.specularColor, material.specularShininess ) * material.specularStrength;
}
void RE_IndirectDiffuse_BlinnPhong( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_BlinnPhong
#define RE_IndirectDiffuse		RE_IndirectDiffuse_BlinnPhong`,yp=`PhysicalMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.diffuseContribution = diffuseColor.rgb * ( 1.0 - metalnessFactor );
material.metalness = metalnessFactor;
vec3 dxy = max( abs( dFdx( nonPerturbedNormal ) ), abs( dFdy( nonPerturbedNormal ) ) );
float geometryRoughness = max( max( dxy.x, dxy.y ), dxy.z );
material.roughness = max( roughnessFactor, 0.0525 );material.roughness += geometryRoughness;
material.roughness = min( material.roughness, 1.0 );
#ifdef IOR
	material.ior = ior;
	#ifdef USE_SPECULAR
		float specularIntensityFactor = specularIntensity;
		vec3 specularColorFactor = specularColor;
		#ifdef USE_SPECULAR_COLORMAP
			specularColorFactor *= texture2D( specularColorMap, vSpecularColorMapUv ).rgb;
		#endif
		#ifdef USE_SPECULAR_INTENSITYMAP
			specularIntensityFactor *= texture2D( specularIntensityMap, vSpecularIntensityMapUv ).a;
		#endif
		material.specularF90 = mix( specularIntensityFactor, 1.0, metalnessFactor );
	#else
		float specularIntensityFactor = 1.0;
		vec3 specularColorFactor = vec3( 1.0 );
		material.specularF90 = 1.0;
	#endif
	material.specularColor = min( pow2( ( material.ior - 1.0 ) / ( material.ior + 1.0 ) ) * specularColorFactor, vec3( 1.0 ) ) * specularIntensityFactor;
	material.specularColorBlended = mix( material.specularColor, diffuseColor.rgb, metalnessFactor );
#else
	material.specularColor = vec3( 0.04 );
	material.specularColorBlended = mix( material.specularColor, diffuseColor.rgb, metalnessFactor );
	material.specularF90 = 1.0;
#endif
#ifdef USE_CLEARCOAT
	material.clearcoat = clearcoat;
	material.clearcoatRoughness = clearcoatRoughness;
	material.clearcoatF0 = vec3( 0.04 );
	material.clearcoatF90 = 1.0;
	#ifdef USE_CLEARCOATMAP
		material.clearcoat *= texture2D( clearcoatMap, vClearcoatMapUv ).x;
	#endif
	#ifdef USE_CLEARCOAT_ROUGHNESSMAP
		material.clearcoatRoughness *= texture2D( clearcoatRoughnessMap, vClearcoatRoughnessMapUv ).y;
	#endif
	material.clearcoat = saturate( material.clearcoat );	material.clearcoatRoughness = max( material.clearcoatRoughness, 0.0525 );
	material.clearcoatRoughness += geometryRoughness;
	material.clearcoatRoughness = min( material.clearcoatRoughness, 1.0 );
#endif
#ifdef USE_DISPERSION
	material.dispersion = dispersion;
#endif
#ifdef USE_IRIDESCENCE
	material.iridescence = iridescence;
	material.iridescenceIOR = iridescenceIOR;
	#ifdef USE_IRIDESCENCEMAP
		material.iridescence *= texture2D( iridescenceMap, vIridescenceMapUv ).r;
	#endif
	#ifdef USE_IRIDESCENCE_THICKNESSMAP
		material.iridescenceThickness = (iridescenceThicknessMaximum - iridescenceThicknessMinimum) * texture2D( iridescenceThicknessMap, vIridescenceThicknessMapUv ).g + iridescenceThicknessMinimum;
	#else
		material.iridescenceThickness = iridescenceThicknessMaximum;
	#endif
#endif
#ifdef USE_SHEEN
	material.sheenColor = sheenColor;
	#ifdef USE_SHEEN_COLORMAP
		material.sheenColor *= texture2D( sheenColorMap, vSheenColorMapUv ).rgb;
	#endif
	material.sheenRoughness = clamp( sheenRoughness, 0.0001, 1.0 );
	#ifdef USE_SHEEN_ROUGHNESSMAP
		material.sheenRoughness *= texture2D( sheenRoughnessMap, vSheenRoughnessMapUv ).a;
	#endif
#endif
#ifdef USE_ANISOTROPY
	#ifdef USE_ANISOTROPYMAP
		mat2 anisotropyMat = mat2( anisotropyVector.x, anisotropyVector.y, - anisotropyVector.y, anisotropyVector.x );
		vec3 anisotropyPolar = texture2D( anisotropyMap, vAnisotropyMapUv ).rgb;
		vec2 anisotropyV = anisotropyMat * normalize( 2.0 * anisotropyPolar.rg - vec2( 1.0 ) ) * anisotropyPolar.b;
	#else
		vec2 anisotropyV = anisotropyVector;
	#endif
	material.anisotropy = length( anisotropyV );
	if( material.anisotropy == 0.0 ) {
		anisotropyV = vec2( 1.0, 0.0 );
	} else {
		anisotropyV /= material.anisotropy;
		material.anisotropy = saturate( material.anisotropy );
	}
	material.alphaT = mix( pow2( material.roughness ), 1.0, pow2( material.anisotropy ) );
	material.anisotropyT = tbn[ 0 ] * anisotropyV.x + tbn[ 1 ] * anisotropyV.y;
	material.anisotropyB = tbn[ 1 ] * anisotropyV.x - tbn[ 0 ] * anisotropyV.y;
#endif`,Ep=`uniform sampler2D dfgLUT;
struct PhysicalMaterial {
	vec3 diffuseColor;
	vec3 diffuseContribution;
	vec3 specularColor;
	vec3 specularColorBlended;
	float roughness;
	float metalness;
	float specularF90;
	float dispersion;
	#ifdef USE_CLEARCOAT
		float clearcoat;
		float clearcoatRoughness;
		vec3 clearcoatF0;
		float clearcoatF90;
	#endif
	#ifdef USE_IRIDESCENCE
		float iridescence;
		float iridescenceIOR;
		float iridescenceThickness;
		vec3 iridescenceFresnel;
		vec3 iridescenceF0;
		vec3 iridescenceFresnelDielectric;
		vec3 iridescenceFresnelMetallic;
	#endif
	#ifdef USE_SHEEN
		vec3 sheenColor;
		float sheenRoughness;
	#endif
	#ifdef IOR
		float ior;
	#endif
	#ifdef USE_TRANSMISSION
		float transmission;
		float transmissionAlpha;
		float thickness;
		float attenuationDistance;
		vec3 attenuationColor;
	#endif
	#ifdef USE_ANISOTROPY
		float anisotropy;
		float alphaT;
		vec3 anisotropyT;
		vec3 anisotropyB;
	#endif
};
vec3 clearcoatSpecularDirect = vec3( 0.0 );
vec3 clearcoatSpecularIndirect = vec3( 0.0 );
vec3 sheenSpecularDirect = vec3( 0.0 );
vec3 sheenSpecularIndirect = vec3(0.0 );
vec3 Schlick_to_F0( const in vec3 f, const in float f90, const in float dotVH ) {
    float x = clamp( 1.0 - dotVH, 0.0, 1.0 );
    float x2 = x * x;
    float x5 = clamp( x * x2 * x2, 0.0, 0.9999 );
    return ( f - vec3( f90 ) * x5 ) / ( 1.0 - x5 );
}
float V_GGX_SmithCorrelated( const in float alpha, const in float dotNL, const in float dotNV ) {
	float a2 = pow2( alpha );
	float gv = dotNL * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNV ) );
	float gl = dotNV * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNL ) );
	return 0.5 / max( gv + gl, EPSILON );
}
float D_GGX( const in float alpha, const in float dotNH ) {
	float a2 = pow2( alpha );
	float denom = pow2( dotNH ) * ( a2 - 1.0 ) + 1.0;
	return RECIPROCAL_PI * a2 / pow2( denom );
}
#ifdef USE_ANISOTROPY
	float V_GGX_SmithCorrelated_Anisotropic( const in float alphaT, const in float alphaB, const in float dotTV, const in float dotBV, const in float dotTL, const in float dotBL, const in float dotNV, const in float dotNL ) {
		float gv = dotNL * length( vec3( alphaT * dotTV, alphaB * dotBV, dotNV ) );
		float gl = dotNV * length( vec3( alphaT * dotTL, alphaB * dotBL, dotNL ) );
		return 0.5 / max( gv + gl, EPSILON );
	}
	float D_GGX_Anisotropic( const in float alphaT, const in float alphaB, const in float dotNH, const in float dotTH, const in float dotBH ) {
		float a2 = alphaT * alphaB;
		highp vec3 v = vec3( alphaB * dotTH, alphaT * dotBH, a2 * dotNH );
		highp float v2 = dot( v, v );
		float w2 = a2 / v2;
		return RECIPROCAL_PI * a2 * pow2 ( w2 );
	}
#endif
#ifdef USE_CLEARCOAT
	vec3 BRDF_GGX_Clearcoat( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in PhysicalMaterial material) {
		vec3 f0 = material.clearcoatF0;
		float f90 = material.clearcoatF90;
		float roughness = material.clearcoatRoughness;
		float alpha = pow2( roughness );
		vec3 halfDir = normalize( lightDir + viewDir );
		float dotNL = saturate( dot( normal, lightDir ) );
		float dotNV = saturate( dot( normal, viewDir ) );
		float dotNH = saturate( dot( normal, halfDir ) );
		float dotVH = saturate( dot( viewDir, halfDir ) );
		vec3 F = F_Schlick( f0, f90, dotVH );
		float V = V_GGX_SmithCorrelated( alpha, dotNL, dotNV );
		float D = D_GGX( alpha, dotNH );
		return F * ( V * D );
	}
#endif
vec3 BRDF_GGX( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in PhysicalMaterial material ) {
	vec3 f0 = material.specularColorBlended;
	float f90 = material.specularF90;
	float roughness = material.roughness;
	float alpha = pow2( roughness );
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNL = saturate( dot( normal, lightDir ) );
	float dotNV = saturate( dot( normal, viewDir ) );
	float dotNH = saturate( dot( normal, halfDir ) );
	float dotVH = saturate( dot( viewDir, halfDir ) );
	vec3 F = F_Schlick( f0, f90, dotVH );
	#ifdef USE_IRIDESCENCE
		F = mix( F, material.iridescenceFresnel, material.iridescence );
	#endif
	#ifdef USE_ANISOTROPY
		float dotTL = dot( material.anisotropyT, lightDir );
		float dotTV = dot( material.anisotropyT, viewDir );
		float dotTH = dot( material.anisotropyT, halfDir );
		float dotBL = dot( material.anisotropyB, lightDir );
		float dotBV = dot( material.anisotropyB, viewDir );
		float dotBH = dot( material.anisotropyB, halfDir );
		float V = V_GGX_SmithCorrelated_Anisotropic( material.alphaT, alpha, dotTV, dotBV, dotTL, dotBL, dotNV, dotNL );
		float D = D_GGX_Anisotropic( material.alphaT, alpha, dotNH, dotTH, dotBH );
	#else
		float V = V_GGX_SmithCorrelated( alpha, dotNL, dotNV );
		float D = D_GGX( alpha, dotNH );
	#endif
	return F * ( V * D );
}
vec2 LTC_Uv( const in vec3 N, const in vec3 V, const in float roughness ) {
	const float LUT_SIZE = 64.0;
	const float LUT_SCALE = ( LUT_SIZE - 1.0 ) / LUT_SIZE;
	const float LUT_BIAS = 0.5 / LUT_SIZE;
	float dotNV = saturate( dot( N, V ) );
	vec2 uv = vec2( roughness, sqrt( 1.0 - dotNV ) );
	uv = uv * LUT_SCALE + LUT_BIAS;
	return uv;
}
float LTC_ClippedSphereFormFactor( const in vec3 f ) {
	float l = length( f );
	return max( ( l * l + f.z ) / ( l + 1.0 ), 0.0 );
}
vec3 LTC_EdgeVectorFormFactor( const in vec3 v1, const in vec3 v2 ) {
	float x = dot( v1, v2 );
	float y = abs( x );
	float a = 0.8543985 + ( 0.4965155 + 0.0145206 * y ) * y;
	float b = 3.4175940 + ( 4.1616724 + y ) * y;
	float v = a / b;
	float theta_sintheta = ( x > 0.0 ) ? v : 0.5 * inversesqrt( max( 1.0 - x * x, 1e-7 ) ) - v;
	return cross( v1, v2 ) * theta_sintheta;
}
vec3 LTC_Evaluate( const in vec3 N, const in vec3 V, const in vec3 P, const in mat3 mInv, const in vec3 rectCoords[ 4 ] ) {
	vec3 v1 = rectCoords[ 1 ] - rectCoords[ 0 ];
	vec3 v2 = rectCoords[ 3 ] - rectCoords[ 0 ];
	vec3 lightNormal = cross( v1, v2 );
	if( dot( lightNormal, P - rectCoords[ 0 ] ) < 0.0 ) return vec3( 0.0 );
	vec3 T1, T2;
	T1 = normalize( V - N * dot( V, N ) );
	T2 = - cross( N, T1 );
	mat3 mat = mInv * transpose( mat3( T1, T2, N ) );
	vec3 coords[ 4 ];
	coords[ 0 ] = mat * ( rectCoords[ 0 ] - P );
	coords[ 1 ] = mat * ( rectCoords[ 1 ] - P );
	coords[ 2 ] = mat * ( rectCoords[ 2 ] - P );
	coords[ 3 ] = mat * ( rectCoords[ 3 ] - P );
	coords[ 0 ] = normalize( coords[ 0 ] );
	coords[ 1 ] = normalize( coords[ 1 ] );
	coords[ 2 ] = normalize( coords[ 2 ] );
	coords[ 3 ] = normalize( coords[ 3 ] );
	vec3 vectorFormFactor = vec3( 0.0 );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 0 ], coords[ 1 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 1 ], coords[ 2 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 2 ], coords[ 3 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 3 ], coords[ 0 ] );
	float result = LTC_ClippedSphereFormFactor( vectorFormFactor );
	return vec3( result );
}
#if defined( USE_SHEEN )
float D_Charlie( float roughness, float dotNH ) {
	float alpha = pow2( roughness );
	float invAlpha = 1.0 / alpha;
	float cos2h = dotNH * dotNH;
	float sin2h = max( 1.0 - cos2h, 0.0078125 );
	return ( 2.0 + invAlpha ) * pow( sin2h, invAlpha * 0.5 ) / ( 2.0 * PI );
}
float V_Neubelt( float dotNV, float dotNL ) {
	return saturate( 1.0 / ( 4.0 * ( dotNL + dotNV - dotNL * dotNV ) ) );
}
vec3 BRDF_Sheen( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, vec3 sheenColor, const in float sheenRoughness ) {
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNL = saturate( dot( normal, lightDir ) );
	float dotNV = saturate( dot( normal, viewDir ) );
	float dotNH = saturate( dot( normal, halfDir ) );
	float D = D_Charlie( sheenRoughness, dotNH );
	float V = V_Neubelt( dotNV, dotNL );
	return sheenColor * ( D * V );
}
#endif
float IBLSheenBRDF( const in vec3 normal, const in vec3 viewDir, const in float roughness ) {
	float dotNV = saturate( dot( normal, viewDir ) );
	float r2 = roughness * roughness;
	float rInv = 1.0 / ( roughness + 0.1 );
	float a = -1.9362 + 1.0678 * roughness + 0.4573 * r2 - 0.8469 * rInv;
	float b = -0.6014 + 0.5538 * roughness - 0.4670 * r2 - 0.1255 * rInv;
	float DG = exp( a * dotNV + b );
	return saturate( DG );
}
vec3 EnvironmentBRDF( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float roughness ) {
	float dotNV = saturate( dot( normal, viewDir ) );
	vec2 fab = texture2D( dfgLUT, vec2( roughness, dotNV ) ).rg;
	return specularColor * fab.x + specularF90 * fab.y;
}
#ifdef USE_IRIDESCENCE
void computeMultiscatteringIridescence( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float iridescence, const in vec3 iridescenceF0, const in float roughness, inout vec3 singleScatter, inout vec3 multiScatter ) {
#else
void computeMultiscattering( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float roughness, inout vec3 singleScatter, inout vec3 multiScatter ) {
#endif
	float dotNV = saturate( dot( normal, viewDir ) );
	vec2 fab = texture2D( dfgLUT, vec2( roughness, dotNV ) ).rg;
	#ifdef USE_IRIDESCENCE
		vec3 Fr = mix( specularColor, iridescenceF0, iridescence );
	#else
		vec3 Fr = specularColor;
	#endif
	vec3 FssEss = Fr * fab.x + specularF90 * fab.y;
	float Ess = fab.x + fab.y;
	float Ems = 1.0 - Ess;
	vec3 Favg = Fr + ( 1.0 - Fr ) * 0.047619;	vec3 Fms = FssEss * Favg / ( 1.0 - Ems * Favg );
	singleScatter += FssEss;
	multiScatter += Fms * Ems;
}
vec3 BRDF_GGX_Multiscatter( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in PhysicalMaterial material ) {
	vec3 singleScatter = BRDF_GGX( lightDir, viewDir, normal, material );
	float dotNL = saturate( dot( normal, lightDir ) );
	float dotNV = saturate( dot( normal, viewDir ) );
	vec2 dfgV = texture2D( dfgLUT, vec2( material.roughness, dotNV ) ).rg;
	vec2 dfgL = texture2D( dfgLUT, vec2( material.roughness, dotNL ) ).rg;
	vec3 FssEss_V = material.specularColorBlended * dfgV.x + material.specularF90 * dfgV.y;
	vec3 FssEss_L = material.specularColorBlended * dfgL.x + material.specularF90 * dfgL.y;
	float Ess_V = dfgV.x + dfgV.y;
	float Ess_L = dfgL.x + dfgL.y;
	float Ems_V = 1.0 - Ess_V;
	float Ems_L = 1.0 - Ess_L;
	vec3 Favg = material.specularColorBlended + ( 1.0 - material.specularColorBlended ) * 0.047619;
	vec3 Fms = FssEss_V * FssEss_L * Favg / ( 1.0 - Ems_V * Ems_L * Favg + EPSILON );
	float compensationFactor = Ems_V * Ems_L;
	vec3 multiScatter = Fms * compensationFactor;
	return singleScatter + multiScatter;
}
#if NUM_RECT_AREA_LIGHTS > 0
	void RE_Direct_RectArea_Physical( const in RectAreaLight rectAreaLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
		vec3 normal = geometryNormal;
		vec3 viewDir = geometryViewDir;
		vec3 position = geometryPosition;
		vec3 lightPos = rectAreaLight.position;
		vec3 halfWidth = rectAreaLight.halfWidth;
		vec3 halfHeight = rectAreaLight.halfHeight;
		vec3 lightColor = rectAreaLight.color;
		float roughness = material.roughness;
		vec3 rectCoords[ 4 ];
		rectCoords[ 0 ] = lightPos + halfWidth - halfHeight;		rectCoords[ 1 ] = lightPos - halfWidth - halfHeight;
		rectCoords[ 2 ] = lightPos - halfWidth + halfHeight;
		rectCoords[ 3 ] = lightPos + halfWidth + halfHeight;
		vec2 uv = LTC_Uv( normal, viewDir, roughness );
		vec4 t1 = texture2D( ltc_1, uv );
		vec4 t2 = texture2D( ltc_2, uv );
		mat3 mInv = mat3(
			vec3( t1.x, 0, t1.y ),
			vec3(    0, 1,    0 ),
			vec3( t1.z, 0, t1.w )
		);
		vec3 fresnel = ( material.specularColorBlended * t2.x + ( material.specularF90 - material.specularColorBlended ) * t2.y );
		reflectedLight.directSpecular += lightColor * fresnel * LTC_Evaluate( normal, viewDir, position, mInv, rectCoords );
		reflectedLight.directDiffuse += lightColor * material.diffuseContribution * LTC_Evaluate( normal, viewDir, position, mat3( 1.0 ), rectCoords );
		#ifdef USE_CLEARCOAT
			vec3 Ncc = geometryClearcoatNormal;
			vec2 uvClearcoat = LTC_Uv( Ncc, viewDir, material.clearcoatRoughness );
			vec4 t1Clearcoat = texture2D( ltc_1, uvClearcoat );
			vec4 t2Clearcoat = texture2D( ltc_2, uvClearcoat );
			mat3 mInvClearcoat = mat3(
				vec3( t1Clearcoat.x, 0, t1Clearcoat.y ),
				vec3(             0, 1,             0 ),
				vec3( t1Clearcoat.z, 0, t1Clearcoat.w )
			);
			vec3 fresnelClearcoat = material.clearcoatF0 * t2Clearcoat.x + ( material.clearcoatF90 - material.clearcoatF0 ) * t2Clearcoat.y;
			clearcoatSpecularDirect += lightColor * fresnelClearcoat * LTC_Evaluate( Ncc, viewDir, position, mInvClearcoat, rectCoords );
		#endif
	}
#endif
void RE_Direct_Physical( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	#ifdef USE_CLEARCOAT
		float dotNLcc = saturate( dot( geometryClearcoatNormal, directLight.direction ) );
		vec3 ccIrradiance = dotNLcc * directLight.color;
		clearcoatSpecularDirect += ccIrradiance * BRDF_GGX_Clearcoat( directLight.direction, geometryViewDir, geometryClearcoatNormal, material );
	#endif
	#ifdef USE_SHEEN
 
 		sheenSpecularDirect += irradiance * BRDF_Sheen( directLight.direction, geometryViewDir, geometryNormal, material.sheenColor, material.sheenRoughness );
 
 		float sheenAlbedoV = IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness );
 		float sheenAlbedoL = IBLSheenBRDF( geometryNormal, directLight.direction, material.sheenRoughness );
 
 		float sheenEnergyComp = 1.0 - max3( material.sheenColor ) * max( sheenAlbedoV, sheenAlbedoL );
 
 		irradiance *= sheenEnergyComp;
 
 	#endif
	reflectedLight.directSpecular += irradiance * BRDF_GGX_Multiscatter( directLight.direction, geometryViewDir, geometryNormal, material );
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseContribution );
}
void RE_IndirectDiffuse_Physical( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
	vec3 diffuse = irradiance * BRDF_Lambert( material.diffuseContribution );
	#ifdef USE_SHEEN
		float sheenAlbedo = IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness );
		float sheenEnergyComp = 1.0 - max3( material.sheenColor ) * sheenAlbedo;
		diffuse *= sheenEnergyComp;
	#endif
	reflectedLight.indirectDiffuse += diffuse;
}
void RE_IndirectSpecular_Physical( const in vec3 radiance, const in vec3 irradiance, const in vec3 clearcoatRadiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight) {
	#ifdef USE_CLEARCOAT
		clearcoatSpecularIndirect += clearcoatRadiance * EnvironmentBRDF( geometryClearcoatNormal, geometryViewDir, material.clearcoatF0, material.clearcoatF90, material.clearcoatRoughness );
	#endif
	#ifdef USE_SHEEN
		sheenSpecularIndirect += irradiance * material.sheenColor * IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness ) * RECIPROCAL_PI;
 	#endif
	vec3 singleScatteringDielectric = vec3( 0.0 );
	vec3 multiScatteringDielectric = vec3( 0.0 );
	vec3 singleScatteringMetallic = vec3( 0.0 );
	vec3 multiScatteringMetallic = vec3( 0.0 );
	#ifdef USE_IRIDESCENCE
		computeMultiscatteringIridescence( geometryNormal, geometryViewDir, material.specularColor, material.specularF90, material.iridescence, material.iridescenceFresnelDielectric, material.roughness, singleScatteringDielectric, multiScatteringDielectric );
		computeMultiscatteringIridescence( geometryNormal, geometryViewDir, material.diffuseColor, material.specularF90, material.iridescence, material.iridescenceFresnelMetallic, material.roughness, singleScatteringMetallic, multiScatteringMetallic );
	#else
		computeMultiscattering( geometryNormal, geometryViewDir, material.specularColor, material.specularF90, material.roughness, singleScatteringDielectric, multiScatteringDielectric );
		computeMultiscattering( geometryNormal, geometryViewDir, material.diffuseColor, material.specularF90, material.roughness, singleScatteringMetallic, multiScatteringMetallic );
	#endif
	vec3 singleScattering = mix( singleScatteringDielectric, singleScatteringMetallic, material.metalness );
	vec3 multiScattering = mix( multiScatteringDielectric, multiScatteringMetallic, material.metalness );
	vec3 totalScatteringDielectric = singleScatteringDielectric + multiScatteringDielectric;
	vec3 diffuse = material.diffuseContribution * ( 1.0 - totalScatteringDielectric );
	vec3 cosineWeightedIrradiance = irradiance * RECIPROCAL_PI;
	vec3 indirectSpecular = radiance * singleScattering;
	indirectSpecular += multiScattering * cosineWeightedIrradiance;
	vec3 indirectDiffuse = diffuse * cosineWeightedIrradiance;
	#ifdef USE_SHEEN
		float sheenAlbedo = IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness );
		float sheenEnergyComp = 1.0 - max3( material.sheenColor ) * sheenAlbedo;
		indirectSpecular *= sheenEnergyComp;
		indirectDiffuse *= sheenEnergyComp;
	#endif
	reflectedLight.indirectSpecular += indirectSpecular;
	reflectedLight.indirectDiffuse += indirectDiffuse;
}
#define RE_Direct				RE_Direct_Physical
#define RE_Direct_RectArea		RE_Direct_RectArea_Physical
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Physical
#define RE_IndirectSpecular		RE_IndirectSpecular_Physical
float computeSpecularOcclusion( const in float dotNV, const in float ambientOcclusion, const in float roughness ) {
	return saturate( pow( dotNV + ambientOcclusion, exp2( - 16.0 * roughness - 1.0 ) ) - 1.0 + ambientOcclusion );
}`,bp=`
vec3 geometryPosition = - vViewPosition;
vec3 geometryNormal = normal;
vec3 geometryViewDir = ( isOrthographic ) ? vec3( 0, 0, 1 ) : normalize( vViewPosition );
vec3 geometryClearcoatNormal = vec3( 0.0 );
#ifdef USE_CLEARCOAT
	geometryClearcoatNormal = clearcoatNormal;
#endif
#ifdef USE_IRIDESCENCE
	float dotNVi = saturate( dot( normal, geometryViewDir ) );
	if ( material.iridescenceThickness == 0.0 ) {
		material.iridescence = 0.0;
	} else {
		material.iridescence = saturate( material.iridescence );
	}
	if ( material.iridescence > 0.0 ) {
		material.iridescenceFresnelDielectric = evalIridescence( 1.0, material.iridescenceIOR, dotNVi, material.iridescenceThickness, material.specularColor );
		material.iridescenceFresnelMetallic = evalIridescence( 1.0, material.iridescenceIOR, dotNVi, material.iridescenceThickness, material.diffuseColor );
		material.iridescenceFresnel = mix( material.iridescenceFresnelDielectric, material.iridescenceFresnelMetallic, material.metalness );
		material.iridescenceF0 = Schlick_to_F0( material.iridescenceFresnel, 1.0, dotNVi );
	}
#endif
IncidentLight directLight;
#if ( NUM_POINT_LIGHTS > 0 ) && defined( RE_Direct )
	PointLight pointLight;
	#if defined( USE_SHADOWMAP ) && NUM_POINT_LIGHT_SHADOWS > 0
	PointLightShadow pointLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_POINT_LIGHTS; i ++ ) {
		pointLight = pointLights[ i ];
		getPointLightInfo( pointLight, geometryPosition, directLight );
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_POINT_LIGHT_SHADOWS ) && ( defined( SHADOWMAP_TYPE_PCF ) || defined( SHADOWMAP_TYPE_BASIC ) )
		pointLightShadow = pointLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getPointShadow( pointShadowMap[ i ], pointLightShadow.shadowMapSize, pointLightShadow.shadowIntensity, pointLightShadow.shadowBias, pointLightShadow.shadowRadius, vPointShadowCoord[ i ], pointLightShadow.shadowCameraNear, pointLightShadow.shadowCameraFar ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_SPOT_LIGHTS > 0 ) && defined( RE_Direct )
	SpotLight spotLight;
	vec4 spotColor;
	vec3 spotLightCoord;
	bool inSpotLightMap;
	#if defined( USE_SHADOWMAP ) && NUM_SPOT_LIGHT_SHADOWS > 0
	SpotLightShadow spotLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHTS; i ++ ) {
		spotLight = spotLights[ i ];
		getSpotLightInfo( spotLight, geometryPosition, directLight );
		#if ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS )
		#define SPOT_LIGHT_MAP_INDEX UNROLLED_LOOP_INDEX
		#elif ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
		#define SPOT_LIGHT_MAP_INDEX NUM_SPOT_LIGHT_MAPS
		#else
		#define SPOT_LIGHT_MAP_INDEX ( UNROLLED_LOOP_INDEX - NUM_SPOT_LIGHT_SHADOWS + NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS )
		#endif
		#if ( SPOT_LIGHT_MAP_INDEX < NUM_SPOT_LIGHT_MAPS )
			spotLightCoord = vSpotLightCoord[ i ].xyz / vSpotLightCoord[ i ].w;
			inSpotLightMap = all( lessThan( abs( spotLightCoord * 2. - 1. ), vec3( 1.0 ) ) );
			spotColor = texture2D( spotLightMap[ SPOT_LIGHT_MAP_INDEX ], spotLightCoord.xy );
			directLight.color = inSpotLightMap ? directLight.color * spotColor.rgb : directLight.color;
		#endif
		#undef SPOT_LIGHT_MAP_INDEX
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
		spotLightShadow = spotLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getShadow( spotShadowMap[ i ], spotLightShadow.shadowMapSize, spotLightShadow.shadowIntensity, spotLightShadow.shadowBias, spotLightShadow.shadowRadius, vSpotLightCoord[ i ] ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_DIR_LIGHTS > 0 ) && defined( RE_Direct )
	DirectionalLight directionalLight;
	#if defined( USE_SHADOWMAP ) && NUM_DIR_LIGHT_SHADOWS > 0
	DirectionalLightShadow directionalLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_DIR_LIGHTS; i ++ ) {
		directionalLight = directionalLights[ i ];
		getDirectionalLightInfo( directionalLight, directLight );
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_DIR_LIGHT_SHADOWS )
		directionalLightShadow = directionalLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getShadow( directionalShadowMap[ i ], directionalLightShadow.shadowMapSize, directionalLightShadow.shadowIntensity, directionalLightShadow.shadowBias, directionalLightShadow.shadowRadius, vDirectionalShadowCoord[ i ] ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_RECT_AREA_LIGHTS > 0 ) && defined( RE_Direct_RectArea )
	RectAreaLight rectAreaLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_RECT_AREA_LIGHTS; i ++ ) {
		rectAreaLight = rectAreaLights[ i ];
		RE_Direct_RectArea( rectAreaLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if defined( RE_IndirectDiffuse )
	vec3 iblIrradiance = vec3( 0.0 );
	vec3 irradiance = getAmbientLightIrradiance( ambientLightColor );
	#if defined( USE_LIGHT_PROBES )
		irradiance += getLightProbeIrradiance( lightProbe, geometryNormal );
	#endif
	#if ( NUM_HEMI_LIGHTS > 0 )
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_HEMI_LIGHTS; i ++ ) {
			irradiance += getHemisphereLightIrradiance( hemisphereLights[ i ], geometryNormal );
		}
		#pragma unroll_loop_end
	#endif
	#ifdef USE_LIGHT_PROBES_GRID
		vec3 probeWorldPos = ( ( vec4( geometryPosition, 1.0 ) - viewMatrix[ 3 ] ) * viewMatrix ).xyz;
		vec3 probeWorldNormal = inverseTransformDirection( geometryNormal, viewMatrix );
		irradiance += getLightProbeGridIrradiance( probeWorldPos, probeWorldNormal );
	#endif
#endif
#if defined( RE_IndirectSpecular )
	vec3 radiance = vec3( 0.0 );
	vec3 clearcoatRadiance = vec3( 0.0 );
#endif`,Tp=`#if defined( RE_IndirectDiffuse )
	#ifdef USE_LIGHTMAP
		vec4 lightMapTexel = texture2D( lightMap, vLightMapUv );
		vec3 lightMapIrradiance = lightMapTexel.rgb * lightMapIntensity;
		irradiance += lightMapIrradiance;
	#endif
	#if defined( USE_ENVMAP ) && defined( ENVMAP_TYPE_CUBE_UV )
		#if defined( STANDARD ) || defined( LAMBERT ) || defined( PHONG )
			iblIrradiance += getIBLIrradiance( geometryNormal );
		#endif
	#endif
#endif
#if defined( USE_ENVMAP ) && defined( RE_IndirectSpecular )
	#ifdef USE_ANISOTROPY
		radiance += getIBLAnisotropyRadiance( geometryViewDir, geometryNormal, material.roughness, material.anisotropyB, material.anisotropy );
	#else
		radiance += getIBLRadiance( geometryViewDir, geometryNormal, material.roughness );
	#endif
	#ifdef USE_CLEARCOAT
		clearcoatRadiance += getIBLRadiance( geometryViewDir, geometryClearcoatNormal, material.clearcoatRoughness );
	#endif
#endif`,Ap=`#if defined( RE_IndirectDiffuse )
	#if defined( LAMBERT ) || defined( PHONG )
		irradiance += iblIrradiance;
	#endif
	RE_IndirectDiffuse( irradiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif
#if defined( RE_IndirectSpecular )
	RE_IndirectSpecular( radiance, iblIrradiance, clearcoatRadiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif`,wp=`#ifdef USE_LIGHT_PROBES_GRID
uniform highp sampler3D probesSH;
uniform vec3 probesMin;
uniform vec3 probesMax;
uniform vec3 probesResolution;
vec3 getLightProbeGridIrradiance( vec3 worldPos, vec3 worldNormal ) {
	vec3 res = probesResolution;
	vec3 gridRange = probesMax - probesMin;
	vec3 resMinusOne = res - 1.0;
	vec3 probeSpacing = gridRange / resMinusOne;
	vec3 samplePos = worldPos + worldNormal * probeSpacing * 0.5;
	vec3 uvw = clamp( ( samplePos - probesMin ) / gridRange, 0.0, 1.0 );
	uvw = uvw * resMinusOne / res + 0.5 / res;
	float nz          = res.z;
	float paddedSlices = nz + 2.0;
	float atlasDepth  = 7.0 * paddedSlices;
	float uvZBase     = uvw.z * nz + 1.0;
	vec4 s0 = texture( probesSH, vec3( uvw.xy, ( uvZBase                       ) / atlasDepth ) );
	vec4 s1 = texture( probesSH, vec3( uvw.xy, ( uvZBase +       paddedSlices   ) / atlasDepth ) );
	vec4 s2 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 2.0 * paddedSlices   ) / atlasDepth ) );
	vec4 s3 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 3.0 * paddedSlices   ) / atlasDepth ) );
	vec4 s4 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 4.0 * paddedSlices   ) / atlasDepth ) );
	vec4 s5 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 5.0 * paddedSlices   ) / atlasDepth ) );
	vec4 s6 = texture( probesSH, vec3( uvw.xy, ( uvZBase + 6.0 * paddedSlices   ) / atlasDepth ) );
	vec3 c0 = s0.xyz;
	vec3 c1 = vec3( s0.w, s1.xy );
	vec3 c2 = vec3( s1.zw, s2.x );
	vec3 c3 = s2.yzw;
	vec3 c4 = s3.xyz;
	vec3 c5 = vec3( s3.w, s4.xy );
	vec3 c6 = vec3( s4.zw, s5.x );
	vec3 c7 = s5.yzw;
	vec3 c8 = s6.xyz;
	float x = worldNormal.x, y = worldNormal.y, z = worldNormal.z;
	vec3 result = c0 * 0.886227;
	result += c1 * 2.0 * 0.511664 * y;
	result += c2 * 2.0 * 0.511664 * z;
	result += c3 * 2.0 * 0.511664 * x;
	result += c4 * 2.0 * 0.429043 * x * y;
	result += c5 * 2.0 * 0.429043 * y * z;
	result += c6 * ( 0.743125 * z * z - 0.247708 );
	result += c7 * 2.0 * 0.429043 * x * z;
	result += c8 * 0.429043 * ( x * x - y * y );
	return max( result, vec3( 0.0 ) );
}
#endif`,Rp=`#if defined( USE_LOGARITHMIC_DEPTH_BUFFER )
	gl_FragDepth = vIsPerspective == 0.0 ? gl_FragCoord.z : log2( vFragDepth ) * logDepthBufFC * 0.5;
#endif`,Cp=`#if defined( USE_LOGARITHMIC_DEPTH_BUFFER )
	uniform float logDepthBufFC;
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,Pp=`#ifdef USE_LOGARITHMIC_DEPTH_BUFFER
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,Lp=`#ifdef USE_LOGARITHMIC_DEPTH_BUFFER
	vFragDepth = 1.0 + gl_Position.w;
	vIsPerspective = float( isPerspectiveMatrix( projectionMatrix ) );
#endif`,Dp=`#ifdef USE_MAP
	vec4 sampledDiffuseColor = texture2D( map, vMapUv );
	#ifdef DECODE_VIDEO_TEXTURE
		sampledDiffuseColor = sRGBTransferEOTF( sampledDiffuseColor );
	#endif
	diffuseColor *= sampledDiffuseColor;
#endif`,Ip=`#ifdef USE_MAP
	uniform sampler2D map;
#endif`,Up=`#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
	#if defined( USE_POINTS_UV )
		vec2 uv = vUv;
	#else
		vec2 uv = ( uvTransform * vec3( gl_PointCoord.x, 1.0 - gl_PointCoord.y, 1 ) ).xy;
	#endif
#endif
#ifdef USE_MAP
	diffuseColor *= texture2D( map, uv );
#endif
#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, uv ).g;
#endif`,Np=`#if defined( USE_POINTS_UV )
	varying vec2 vUv;
#else
	#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
		uniform mat3 uvTransform;
	#endif
#endif
#ifdef USE_MAP
	uniform sampler2D map;
#endif
#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,Fp=`float metalnessFactor = metalness;
#ifdef USE_METALNESSMAP
	vec4 texelMetalness = texture2D( metalnessMap, vMetalnessMapUv );
	metalnessFactor *= texelMetalness.b;
#endif`,Op=`#ifdef USE_METALNESSMAP
	uniform sampler2D metalnessMap;
#endif`,Bp=`#ifdef USE_INSTANCING_MORPH
	float morphTargetInfluences[ MORPHTARGETS_COUNT ];
	float morphTargetBaseInfluence = texelFetch( morphTexture, ivec2( 0, gl_InstanceID ), 0 ).r;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		morphTargetInfluences[i] =  texelFetch( morphTexture, ivec2( i + 1, gl_InstanceID ), 0 ).r;
	}
#endif`,zp=`#if defined( USE_MORPHCOLORS )
	vColor *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		#if defined( USE_COLOR_ALPHA )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ) * morphTargetInfluences[ i ];
		#elif defined( USE_COLOR )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ).rgb * morphTargetInfluences[ i ];
		#endif
	}
#endif`,kp=`#ifdef USE_MORPHNORMALS
	objectNormal *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) objectNormal += getMorph( gl_VertexID, i, 1 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,Hp=`#ifdef USE_MORPHTARGETS
	#ifndef USE_INSTANCING_MORPH
		uniform float morphTargetBaseInfluence;
		uniform float morphTargetInfluences[ MORPHTARGETS_COUNT ];
	#endif
	uniform sampler2DArray morphTargetsTexture;
	uniform ivec2 morphTargetsTextureSize;
	vec4 getMorph( const in int vertexIndex, const in int morphTargetIndex, const in int offset ) {
		int texelIndex = vertexIndex * MORPHTARGETS_TEXTURE_STRIDE + offset;
		int y = texelIndex / morphTargetsTextureSize.x;
		int x = texelIndex - y * morphTargetsTextureSize.x;
		ivec3 morphUV = ivec3( x, y, morphTargetIndex );
		return texelFetch( morphTargetsTexture, morphUV, 0 );
	}
#endif`,Gp=`#ifdef USE_MORPHTARGETS
	transformed *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) transformed += getMorph( gl_VertexID, i, 0 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,Vp=`float faceDirection = gl_FrontFacing ? 1.0 : - 1.0;
#ifdef FLAT_SHADED
	vec3 fdx = dFdx( vViewPosition );
	vec3 fdy = dFdy( vViewPosition );
	vec3 normal = normalize( cross( fdx, fdy ) );
#else
	vec3 normal = normalize( vNormal );
	#ifdef DOUBLE_SIDED
		normal *= faceDirection;
	#endif
#endif
#if defined( USE_NORMALMAP_TANGENTSPACE ) || defined( USE_CLEARCOAT_NORMALMAP ) || defined( USE_ANISOTROPY )
	#ifdef USE_TANGENT
		mat3 tbn = mat3( normalize( vTangent ), normalize( vBitangent ), normal );
	#else
		mat3 tbn = getTangentFrame( - vViewPosition, normal,
		#if defined( USE_NORMALMAP )
			vNormalMapUv
		#elif defined( USE_CLEARCOAT_NORMALMAP )
			vClearcoatNormalMapUv
		#else
			vUv
		#endif
		);
	#endif
	#if defined( DOUBLE_SIDED ) && ! defined( FLAT_SHADED )
		tbn[0] *= faceDirection;
		tbn[1] *= faceDirection;
	#endif
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	#ifdef USE_TANGENT
		mat3 tbn2 = mat3( normalize( vTangent ), normalize( vBitangent ), normal );
	#else
		mat3 tbn2 = getTangentFrame( - vViewPosition, normal, vClearcoatNormalMapUv );
	#endif
	#if defined( DOUBLE_SIDED ) && ! defined( FLAT_SHADED )
		tbn2[0] *= faceDirection;
		tbn2[1] *= faceDirection;
	#endif
#endif
vec3 nonPerturbedNormal = normal;`,Wp=`#ifdef USE_NORMALMAP_OBJECTSPACE
	normal = texture2D( normalMap, vNormalMapUv ).xyz * 2.0 - 1.0;
	#ifdef FLIP_SIDED
		normal = - normal;
	#endif
	#ifdef DOUBLE_SIDED
		normal = normal * faceDirection;
	#endif
	normal = normalize( normalMatrix * normal );
#elif defined( USE_NORMALMAP_TANGENTSPACE )
	vec3 mapN = texture2D( normalMap, vNormalMapUv ).xyz * 2.0 - 1.0;
	#if defined( USE_PACKED_NORMALMAP )
		mapN = vec3( mapN.xy, sqrt( saturate( 1.0 - dot( mapN.xy, mapN.xy ) ) ) );
	#endif
	mapN.xy *= normalScale;
	normal = normalize( tbn * mapN );
#elif defined( USE_BUMPMAP )
	normal = perturbNormalArb( - vViewPosition, normal, dHdxy_fwd(), faceDirection );
#endif`,Xp=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,$p=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,qp=`#ifndef FLAT_SHADED
	vNormal = normalize( transformedNormal );
	#ifdef USE_TANGENT
		vTangent = normalize( transformedTangent );
		vBitangent = normalize( cross( vNormal, vTangent ) * tangent.w );
	#endif
#endif`,Yp=`#ifdef USE_NORMALMAP
	uniform sampler2D normalMap;
	uniform vec2 normalScale;
#endif
#ifdef USE_NORMALMAP_OBJECTSPACE
	uniform mat3 normalMatrix;
#endif
#if ! defined ( USE_TANGENT ) && ( defined ( USE_NORMALMAP_TANGENTSPACE ) || defined ( USE_CLEARCOAT_NORMALMAP ) || defined( USE_ANISOTROPY ) )
	mat3 getTangentFrame( vec3 eye_pos, vec3 surf_norm, vec2 uv ) {
		vec3 q0 = dFdx( eye_pos.xyz );
		vec3 q1 = dFdy( eye_pos.xyz );
		vec2 st0 = dFdx( uv.st );
		vec2 st1 = dFdy( uv.st );
		vec3 N = surf_norm;
		vec3 q1perp = cross( q1, N );
		vec3 q0perp = cross( N, q0 );
		vec3 T = q1perp * st0.x + q0perp * st1.x;
		vec3 B = q1perp * st0.y + q0perp * st1.y;
		float det = max( dot( T, T ), dot( B, B ) );
		float scale = ( det == 0.0 ) ? 0.0 : inversesqrt( det );
		return mat3( T * scale, B * scale, N );
	}
#endif`,jp=`#ifdef USE_CLEARCOAT
	vec3 clearcoatNormal = nonPerturbedNormal;
#endif`,Kp=`#ifdef USE_CLEARCOAT_NORMALMAP
	vec3 clearcoatMapN = texture2D( clearcoatNormalMap, vClearcoatNormalMapUv ).xyz * 2.0 - 1.0;
	clearcoatMapN.xy *= clearcoatNormalScale;
	clearcoatNormal = normalize( tbn2 * clearcoatMapN );
#endif`,Zp=`#ifdef USE_CLEARCOATMAP
	uniform sampler2D clearcoatMap;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform sampler2D clearcoatNormalMap;
	uniform vec2 clearcoatNormalScale;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform sampler2D clearcoatRoughnessMap;
#endif`,Jp=`#ifdef USE_IRIDESCENCEMAP
	uniform sampler2D iridescenceMap;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform sampler2D iridescenceThicknessMap;
#endif`,Qp=`#ifdef OPAQUE
diffuseColor.a = 1.0;
#endif
#ifdef USE_TRANSMISSION
diffuseColor.a *= material.transmissionAlpha;
#endif
gl_FragColor = vec4( outgoingLight, diffuseColor.a );`,tm=`vec3 packNormalToRGB( const in vec3 normal ) {
	return normalize( normal ) * 0.5 + 0.5;
}
vec3 unpackRGBToNormal( const in vec3 rgb ) {
	return 2.0 * rgb.xyz - 1.0;
}
const float PackUpscale = 256. / 255.;const float UnpackDownscale = 255. / 256.;const float ShiftRight8 = 1. / 256.;
const float Inv255 = 1. / 255.;
const vec4 PackFactors = vec4( 1.0, 256.0, 256.0 * 256.0, 256.0 * 256.0 * 256.0 );
const vec2 UnpackFactors2 = vec2( UnpackDownscale, 1.0 / PackFactors.g );
const vec3 UnpackFactors3 = vec3( UnpackDownscale / PackFactors.rg, 1.0 / PackFactors.b );
const vec4 UnpackFactors4 = vec4( UnpackDownscale / PackFactors.rgb, 1.0 / PackFactors.a );
vec4 packDepthToRGBA( const in float v ) {
	if( v <= 0.0 )
		return vec4( 0., 0., 0., 0. );
	if( v >= 1.0 )
		return vec4( 1., 1., 1., 1. );
	float vuf;
	float af = modf( v * PackFactors.a, vuf );
	float bf = modf( vuf * ShiftRight8, vuf );
	float gf = modf( vuf * ShiftRight8, vuf );
	return vec4( vuf * Inv255, gf * PackUpscale, bf * PackUpscale, af );
}
vec3 packDepthToRGB( const in float v ) {
	if( v <= 0.0 )
		return vec3( 0., 0., 0. );
	if( v >= 1.0 )
		return vec3( 1., 1., 1. );
	float vuf;
	float bf = modf( v * PackFactors.b, vuf );
	float gf = modf( vuf * ShiftRight8, vuf );
	return vec3( vuf * Inv255, gf * PackUpscale, bf );
}
vec2 packDepthToRG( const in float v ) {
	if( v <= 0.0 )
		return vec2( 0., 0. );
	if( v >= 1.0 )
		return vec2( 1., 1. );
	float vuf;
	float gf = modf( v * 256., vuf );
	return vec2( vuf * Inv255, gf );
}
float unpackRGBAToDepth( const in vec4 v ) {
	return dot( v, UnpackFactors4 );
}
float unpackRGBToDepth( const in vec3 v ) {
	return dot( v, UnpackFactors3 );
}
float unpackRGToDepth( const in vec2 v ) {
	return v.r * UnpackFactors2.r + v.g * UnpackFactors2.g;
}
vec4 pack2HalfToRGBA( const in vec2 v ) {
	vec4 r = vec4( v.x, fract( v.x * 255.0 ), v.y, fract( v.y * 255.0 ) );
	return vec4( r.x - r.y / 255.0, r.y, r.z - r.w / 255.0, r.w );
}
vec2 unpackRGBATo2Half( const in vec4 v ) {
	return vec2( v.x + ( v.y / 255.0 ), v.z + ( v.w / 255.0 ) );
}
float viewZToOrthographicDepth( const in float viewZ, const in float near, const in float far ) {
	return ( viewZ + near ) / ( near - far );
}
float orthographicDepthToViewZ( const in float depth, const in float near, const in float far ) {
	#ifdef USE_REVERSED_DEPTH_BUFFER
	
		return depth * ( far - near ) - far;
	#else
		return depth * ( near - far ) - near;
	#endif
}
float viewZToPerspectiveDepth( const in float viewZ, const in float near, const in float far ) {
	return ( ( near + viewZ ) * far ) / ( ( far - near ) * viewZ );
}
float perspectiveDepthToViewZ( const in float depth, const in float near, const in float far ) {
	
	#ifdef USE_REVERSED_DEPTH_BUFFER
		return ( near * far ) / ( ( near - far ) * depth - near );
	#else
		return ( near * far ) / ( ( far - near ) * depth - far );
	#endif
}`,em=`#ifdef PREMULTIPLIED_ALPHA
	gl_FragColor.rgb *= gl_FragColor.a;
#endif`,nm=`vec4 mvPosition = vec4( transformed, 1.0 );
#ifdef USE_BATCHING
	mvPosition = batchingMatrix * mvPosition;
#endif
#ifdef USE_INSTANCING
	mvPosition = instanceMatrix * mvPosition;
#endif
mvPosition = modelViewMatrix * mvPosition;
gl_Position = projectionMatrix * mvPosition;`,im=`#ifdef DITHERING
	gl_FragColor.rgb = dithering( gl_FragColor.rgb );
#endif`,sm=`#ifdef DITHERING
	vec3 dithering( vec3 color ) {
		float grid_position = rand( gl_FragCoord.xy );
		vec3 dither_shift_RGB = vec3( 0.25 / 255.0, -0.25 / 255.0, 0.25 / 255.0 );
		dither_shift_RGB = mix( 2.0 * dither_shift_RGB, -2.0 * dither_shift_RGB, grid_position );
		return color + dither_shift_RGB;
	}
#endif`,rm=`float roughnessFactor = roughness;
#ifdef USE_ROUGHNESSMAP
	vec4 texelRoughness = texture2D( roughnessMap, vRoughnessMapUv );
	roughnessFactor *= texelRoughness.g;
#endif`,am=`#ifdef USE_ROUGHNESSMAP
	uniform sampler2D roughnessMap;
#endif`,om=`#if NUM_SPOT_LIGHT_COORDS > 0
	varying vec4 vSpotLightCoord[ NUM_SPOT_LIGHT_COORDS ];
#endif
#if NUM_SPOT_LIGHT_MAPS > 0
	uniform sampler2D spotLightMap[ NUM_SPOT_LIGHT_MAPS ];
#endif
#ifdef USE_SHADOWMAP
	#if NUM_DIR_LIGHT_SHADOWS > 0
		#if defined( SHADOWMAP_TYPE_PCF )
			uniform sampler2DShadow directionalShadowMap[ NUM_DIR_LIGHT_SHADOWS ];
		#else
			uniform sampler2D directionalShadowMap[ NUM_DIR_LIGHT_SHADOWS ];
		#endif
		varying vec4 vDirectionalShadowCoord[ NUM_DIR_LIGHT_SHADOWS ];
		struct DirectionalLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform DirectionalLightShadow directionalLightShadows[ NUM_DIR_LIGHT_SHADOWS ];
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
		#if defined( SHADOWMAP_TYPE_PCF )
			uniform sampler2DShadow spotShadowMap[ NUM_SPOT_LIGHT_SHADOWS ];
		#else
			uniform sampler2D spotShadowMap[ NUM_SPOT_LIGHT_SHADOWS ];
		#endif
		struct SpotLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform SpotLightShadow spotLightShadows[ NUM_SPOT_LIGHT_SHADOWS ];
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		#if defined( SHADOWMAP_TYPE_PCF )
			uniform samplerCubeShadow pointShadowMap[ NUM_POINT_LIGHT_SHADOWS ];
		#elif defined( SHADOWMAP_TYPE_BASIC )
			uniform samplerCube pointShadowMap[ NUM_POINT_LIGHT_SHADOWS ];
		#endif
		varying vec4 vPointShadowCoord[ NUM_POINT_LIGHT_SHADOWS ];
		struct PointLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
			float shadowCameraNear;
			float shadowCameraFar;
		};
		uniform PointLightShadow pointLightShadows[ NUM_POINT_LIGHT_SHADOWS ];
	#endif
	#if defined( SHADOWMAP_TYPE_PCF )
		float interleavedGradientNoise( vec2 position ) {
			return fract( 52.9829189 * fract( dot( position, vec2( 0.06711056, 0.00583715 ) ) ) );
		}
		vec2 vogelDiskSample( int sampleIndex, int samplesCount, float phi ) {
			const float goldenAngle = 2.399963229728653;
			float r = sqrt( ( float( sampleIndex ) + 0.5 ) / float( samplesCount ) );
			float theta = float( sampleIndex ) * goldenAngle + phi;
			return vec2( cos( theta ), sin( theta ) ) * r;
		}
	#endif
	#if defined( SHADOWMAP_TYPE_PCF )
		float getShadow( sampler2DShadow shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord ) {
			float shadow = 1.0;
			shadowCoord.xyz /= shadowCoord.w;
			shadowCoord.z += shadowBias;
			bool inFrustum = shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0 && shadowCoord.y >= 0.0 && shadowCoord.y <= 1.0;
			bool frustumTest = inFrustum && shadowCoord.z <= 1.0;
			if ( frustumTest ) {
				vec2 texelSize = vec2( 1.0 ) / shadowMapSize;
				float radius = shadowRadius * texelSize.x;
				float phi = interleavedGradientNoise( gl_FragCoord.xy ) * PI2;
				shadow = (
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 0, 5, phi ) * radius, shadowCoord.z ) ) +
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 1, 5, phi ) * radius, shadowCoord.z ) ) +
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 2, 5, phi ) * radius, shadowCoord.z ) ) +
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 3, 5, phi ) * radius, shadowCoord.z ) ) +
					texture( shadowMap, vec3( shadowCoord.xy + vogelDiskSample( 4, 5, phi ) * radius, shadowCoord.z ) )
				) * 0.2;
			}
			return mix( 1.0, shadow, shadowIntensity );
		}
	#elif defined( SHADOWMAP_TYPE_VSM )
		float getShadow( sampler2D shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord ) {
			float shadow = 1.0;
			shadowCoord.xyz /= shadowCoord.w;
			#ifdef USE_REVERSED_DEPTH_BUFFER
				shadowCoord.z -= shadowBias;
			#else
				shadowCoord.z += shadowBias;
			#endif
			bool inFrustum = shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0 && shadowCoord.y >= 0.0 && shadowCoord.y <= 1.0;
			bool frustumTest = inFrustum && shadowCoord.z <= 1.0;
			if ( frustumTest ) {
				vec2 distribution = texture2D( shadowMap, shadowCoord.xy ).rg;
				float mean = distribution.x;
				float variance = distribution.y * distribution.y;
				#ifdef USE_REVERSED_DEPTH_BUFFER
					float hard_shadow = step( mean, shadowCoord.z );
				#else
					float hard_shadow = step( shadowCoord.z, mean );
				#endif
				
				if ( hard_shadow == 1.0 ) {
					shadow = 1.0;
				} else {
					variance = max( variance, 0.0000001 );
					float d = shadowCoord.z - mean;
					float p_max = variance / ( variance + d * d );
					p_max = clamp( ( p_max - 0.3 ) / 0.65, 0.0, 1.0 );
					shadow = max( hard_shadow, p_max );
				}
			}
			return mix( 1.0, shadow, shadowIntensity );
		}
	#else
		float getShadow( sampler2D shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord ) {
			float shadow = 1.0;
			shadowCoord.xyz /= shadowCoord.w;
			#ifdef USE_REVERSED_DEPTH_BUFFER
				shadowCoord.z -= shadowBias;
			#else
				shadowCoord.z += shadowBias;
			#endif
			bool inFrustum = shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0 && shadowCoord.y >= 0.0 && shadowCoord.y <= 1.0;
			bool frustumTest = inFrustum && shadowCoord.z <= 1.0;
			if ( frustumTest ) {
				float depth = texture2D( shadowMap, shadowCoord.xy ).r;
				#ifdef USE_REVERSED_DEPTH_BUFFER
					shadow = step( depth, shadowCoord.z );
				#else
					shadow = step( shadowCoord.z, depth );
				#endif
			}
			return mix( 1.0, shadow, shadowIntensity );
		}
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
	#if defined( SHADOWMAP_TYPE_PCF )
	float getPointShadow( samplerCubeShadow shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord, float shadowCameraNear, float shadowCameraFar ) {
		float shadow = 1.0;
		vec3 lightToPosition = shadowCoord.xyz;
		vec3 bd3D = normalize( lightToPosition );
		vec3 absVec = abs( lightToPosition );
		float viewSpaceZ = max( max( absVec.x, absVec.y ), absVec.z );
		if ( viewSpaceZ - shadowCameraFar <= 0.0 && viewSpaceZ - shadowCameraNear >= 0.0 ) {
			#ifdef USE_REVERSED_DEPTH_BUFFER
				float dp = ( shadowCameraNear * ( shadowCameraFar - viewSpaceZ ) ) / ( viewSpaceZ * ( shadowCameraFar - shadowCameraNear ) );
				dp -= shadowBias;
			#else
				float dp = ( shadowCameraFar * ( viewSpaceZ - shadowCameraNear ) ) / ( viewSpaceZ * ( shadowCameraFar - shadowCameraNear ) );
				dp += shadowBias;
			#endif
			float texelSize = shadowRadius / shadowMapSize.x;
			vec3 absDir = abs( bd3D );
			vec3 tangent = absDir.x > absDir.z ? vec3( 0.0, 1.0, 0.0 ) : vec3( 1.0, 0.0, 0.0 );
			tangent = normalize( cross( bd3D, tangent ) );
			vec3 bitangent = cross( bd3D, tangent );
			float phi = interleavedGradientNoise( gl_FragCoord.xy ) * PI2;
			vec2 sample0 = vogelDiskSample( 0, 5, phi );
			vec2 sample1 = vogelDiskSample( 1, 5, phi );
			vec2 sample2 = vogelDiskSample( 2, 5, phi );
			vec2 sample3 = vogelDiskSample( 3, 5, phi );
			vec2 sample4 = vogelDiskSample( 4, 5, phi );
			shadow = (
				texture( shadowMap, vec4( bd3D + ( tangent * sample0.x + bitangent * sample0.y ) * texelSize, dp ) ) +
				texture( shadowMap, vec4( bd3D + ( tangent * sample1.x + bitangent * sample1.y ) * texelSize, dp ) ) +
				texture( shadowMap, vec4( bd3D + ( tangent * sample2.x + bitangent * sample2.y ) * texelSize, dp ) ) +
				texture( shadowMap, vec4( bd3D + ( tangent * sample3.x + bitangent * sample3.y ) * texelSize, dp ) ) +
				texture( shadowMap, vec4( bd3D + ( tangent * sample4.x + bitangent * sample4.y ) * texelSize, dp ) )
			) * 0.2;
		}
		return mix( 1.0, shadow, shadowIntensity );
	}
	#elif defined( SHADOWMAP_TYPE_BASIC )
	float getPointShadow( samplerCube shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord, float shadowCameraNear, float shadowCameraFar ) {
		float shadow = 1.0;
		vec3 lightToPosition = shadowCoord.xyz;
		vec3 absVec = abs( lightToPosition );
		float viewSpaceZ = max( max( absVec.x, absVec.y ), absVec.z );
		if ( viewSpaceZ - shadowCameraFar <= 0.0 && viewSpaceZ - shadowCameraNear >= 0.0 ) {
			float dp = ( shadowCameraFar * ( viewSpaceZ - shadowCameraNear ) ) / ( viewSpaceZ * ( shadowCameraFar - shadowCameraNear ) );
			dp += shadowBias;
			vec3 bd3D = normalize( lightToPosition );
			float depth = textureCube( shadowMap, bd3D ).r;
			#ifdef USE_REVERSED_DEPTH_BUFFER
				depth = 1.0 - depth;
			#endif
			shadow = step( dp, depth );
		}
		return mix( 1.0, shadow, shadowIntensity );
	}
	#endif
	#endif
#endif`,lm=`#if NUM_SPOT_LIGHT_COORDS > 0
	uniform mat4 spotLightMatrix[ NUM_SPOT_LIGHT_COORDS ];
	varying vec4 vSpotLightCoord[ NUM_SPOT_LIGHT_COORDS ];
#endif
#ifdef USE_SHADOWMAP
	#if NUM_DIR_LIGHT_SHADOWS > 0
		uniform mat4 directionalShadowMatrix[ NUM_DIR_LIGHT_SHADOWS ];
		varying vec4 vDirectionalShadowCoord[ NUM_DIR_LIGHT_SHADOWS ];
		struct DirectionalLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform DirectionalLightShadow directionalLightShadows[ NUM_DIR_LIGHT_SHADOWS ];
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
		struct SpotLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform SpotLightShadow spotLightShadows[ NUM_SPOT_LIGHT_SHADOWS ];
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		uniform mat4 pointShadowMatrix[ NUM_POINT_LIGHT_SHADOWS ];
		varying vec4 vPointShadowCoord[ NUM_POINT_LIGHT_SHADOWS ];
		struct PointLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
			float shadowCameraNear;
			float shadowCameraFar;
		};
		uniform PointLightShadow pointLightShadows[ NUM_POINT_LIGHT_SHADOWS ];
	#endif
#endif`,cm=`#if ( defined( USE_SHADOWMAP ) && ( NUM_DIR_LIGHT_SHADOWS > 0 || NUM_POINT_LIGHT_SHADOWS > 0 ) ) || ( NUM_SPOT_LIGHT_COORDS > 0 )
	#ifdef HAS_NORMAL
		vec3 shadowWorldNormal = inverseTransformDirection( transformedNormal, viewMatrix );
	#else
		vec3 shadowWorldNormal = vec3( 0.0 );
	#endif
	vec4 shadowWorldPosition;
#endif
#if defined( USE_SHADOWMAP )
	#if NUM_DIR_LIGHT_SHADOWS > 0
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_DIR_LIGHT_SHADOWS; i ++ ) {
			shadowWorldPosition = worldPosition + vec4( shadowWorldNormal * directionalLightShadows[ i ].shadowNormalBias, 0 );
			vDirectionalShadowCoord[ i ] = directionalShadowMatrix[ i ] * shadowWorldPosition;
		}
		#pragma unroll_loop_end
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_POINT_LIGHT_SHADOWS; i ++ ) {
			shadowWorldPosition = worldPosition + vec4( shadowWorldNormal * pointLightShadows[ i ].shadowNormalBias, 0 );
			vPointShadowCoord[ i ] = pointShadowMatrix[ i ] * shadowWorldPosition;
		}
		#pragma unroll_loop_end
	#endif
#endif
#if NUM_SPOT_LIGHT_COORDS > 0
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHT_COORDS; i ++ ) {
		shadowWorldPosition = worldPosition;
		#if ( defined( USE_SHADOWMAP ) && UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
			shadowWorldPosition.xyz += shadowWorldNormal * spotLightShadows[ i ].shadowNormalBias;
		#endif
		vSpotLightCoord[ i ] = spotLightMatrix[ i ] * shadowWorldPosition;
	}
	#pragma unroll_loop_end
#endif`,hm=`float getShadowMask() {
	float shadow = 1.0;
	#ifdef USE_SHADOWMAP
	#if NUM_DIR_LIGHT_SHADOWS > 0
	DirectionalLightShadow directionalLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_DIR_LIGHT_SHADOWS; i ++ ) {
		directionalLight = directionalLightShadows[ i ];
		shadow *= receiveShadow ? getShadow( directionalShadowMap[ i ], directionalLight.shadowMapSize, directionalLight.shadowIntensity, directionalLight.shadowBias, directionalLight.shadowRadius, vDirectionalShadowCoord[ i ] ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
	SpotLightShadow spotLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHT_SHADOWS; i ++ ) {
		spotLight = spotLightShadows[ i ];
		shadow *= receiveShadow ? getShadow( spotShadowMap[ i ], spotLight.shadowMapSize, spotLight.shadowIntensity, spotLight.shadowBias, spotLight.shadowRadius, vSpotLightCoord[ i ] ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0 && ( defined( SHADOWMAP_TYPE_PCF ) || defined( SHADOWMAP_TYPE_BASIC ) )
	PointLightShadow pointLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_POINT_LIGHT_SHADOWS; i ++ ) {
		pointLight = pointLightShadows[ i ];
		shadow *= receiveShadow ? getPointShadow( pointShadowMap[ i ], pointLight.shadowMapSize, pointLight.shadowIntensity, pointLight.shadowBias, pointLight.shadowRadius, vPointShadowCoord[ i ], pointLight.shadowCameraNear, pointLight.shadowCameraFar ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#endif
	return shadow;
}`,um=`#ifdef USE_SKINNING
	mat4 boneMatX = getBoneMatrix( skinIndex.x );
	mat4 boneMatY = getBoneMatrix( skinIndex.y );
	mat4 boneMatZ = getBoneMatrix( skinIndex.z );
	mat4 boneMatW = getBoneMatrix( skinIndex.w );
#endif`,dm=`#ifdef USE_SKINNING
	uniform mat4 bindMatrix;
	uniform mat4 bindMatrixInverse;
	uniform highp sampler2D boneTexture;
	mat4 getBoneMatrix( const in float i ) {
		int size = textureSize( boneTexture, 0 ).x;
		int j = int( i ) * 4;
		int x = j % size;
		int y = j / size;
		vec4 v1 = texelFetch( boneTexture, ivec2( x, y ), 0 );
		vec4 v2 = texelFetch( boneTexture, ivec2( x + 1, y ), 0 );
		vec4 v3 = texelFetch( boneTexture, ivec2( x + 2, y ), 0 );
		vec4 v4 = texelFetch( boneTexture, ivec2( x + 3, y ), 0 );
		return mat4( v1, v2, v3, v4 );
	}
#endif`,fm=`#ifdef USE_SKINNING
	vec4 skinVertex = bindMatrix * vec4( transformed, 1.0 );
	vec4 skinned = vec4( 0.0 );
	skinned += boneMatX * skinVertex * skinWeight.x;
	skinned += boneMatY * skinVertex * skinWeight.y;
	skinned += boneMatZ * skinVertex * skinWeight.z;
	skinned += boneMatW * skinVertex * skinWeight.w;
	transformed = ( bindMatrixInverse * skinned ).xyz;
#endif`,pm=`#ifdef USE_SKINNING
	mat4 skinMatrix = mat4( 0.0 );
	skinMatrix += skinWeight.x * boneMatX;
	skinMatrix += skinWeight.y * boneMatY;
	skinMatrix += skinWeight.z * boneMatZ;
	skinMatrix += skinWeight.w * boneMatW;
	skinMatrix = bindMatrixInverse * skinMatrix * bindMatrix;
	objectNormal = vec4( skinMatrix * vec4( objectNormal, 0.0 ) ).xyz;
	#ifdef USE_TANGENT
		objectTangent = vec4( skinMatrix * vec4( objectTangent, 0.0 ) ).xyz;
	#endif
#endif`,mm=`float specularStrength;
#ifdef USE_SPECULARMAP
	vec4 texelSpecular = texture2D( specularMap, vSpecularMapUv );
	specularStrength = texelSpecular.r;
#else
	specularStrength = 1.0;
#endif`,gm=`#ifdef USE_SPECULARMAP
	uniform sampler2D specularMap;
#endif`,_m=`#if defined( TONE_MAPPING )
	gl_FragColor.rgb = toneMapping( gl_FragColor.rgb );
#endif`,xm=`#ifndef saturate
#define saturate( a ) clamp( a, 0.0, 1.0 )
#endif
uniform float toneMappingExposure;
vec3 LinearToneMapping( vec3 color ) {
	return saturate( toneMappingExposure * color );
}
vec3 ReinhardToneMapping( vec3 color ) {
	color *= toneMappingExposure;
	return saturate( color / ( vec3( 1.0 ) + color ) );
}
vec3 CineonToneMapping( vec3 color ) {
	color *= toneMappingExposure;
	color = max( vec3( 0.0 ), color - 0.004 );
	return pow( ( color * ( 6.2 * color + 0.5 ) ) / ( color * ( 6.2 * color + 1.7 ) + 0.06 ), vec3( 2.2 ) );
}
vec3 RRTAndODTFit( vec3 v ) {
	vec3 a = v * ( v + 0.0245786 ) - 0.000090537;
	vec3 b = v * ( 0.983729 * v + 0.4329510 ) + 0.238081;
	return a / b;
}
vec3 ACESFilmicToneMapping( vec3 color ) {
	const mat3 ACESInputMat = mat3(
		vec3( 0.59719, 0.07600, 0.02840 ),		vec3( 0.35458, 0.90834, 0.13383 ),
		vec3( 0.04823, 0.01566, 0.83777 )
	);
	const mat3 ACESOutputMat = mat3(
		vec3(  1.60475, -0.10208, -0.00327 ),		vec3( -0.53108,  1.10813, -0.07276 ),
		vec3( -0.07367, -0.00605,  1.07602 )
	);
	color *= toneMappingExposure / 0.6;
	color = ACESInputMat * color;
	color = RRTAndODTFit( color );
	color = ACESOutputMat * color;
	return saturate( color );
}
const mat3 LINEAR_REC2020_TO_LINEAR_SRGB = mat3(
	vec3( 1.6605, - 0.1246, - 0.0182 ),
	vec3( - 0.5876, 1.1329, - 0.1006 ),
	vec3( - 0.0728, - 0.0083, 1.1187 )
);
const mat3 LINEAR_SRGB_TO_LINEAR_REC2020 = mat3(
	vec3( 0.6274, 0.0691, 0.0164 ),
	vec3( 0.3293, 0.9195, 0.0880 ),
	vec3( 0.0433, 0.0113, 0.8956 )
);
vec3 agxDefaultContrastApprox( vec3 x ) {
	vec3 x2 = x * x;
	vec3 x4 = x2 * x2;
	return + 15.5 * x4 * x2
		- 40.14 * x4 * x
		+ 31.96 * x4
		- 6.868 * x2 * x
		+ 0.4298 * x2
		+ 0.1191 * x
		- 0.00232;
}
vec3 AgXToneMapping( vec3 color ) {
	const mat3 AgXInsetMatrix = mat3(
		vec3( 0.856627153315983, 0.137318972929847, 0.11189821299995 ),
		vec3( 0.0951212405381588, 0.761241990602591, 0.0767994186031903 ),
		vec3( 0.0482516061458583, 0.101439036467562, 0.811302368396859 )
	);
	const mat3 AgXOutsetMatrix = mat3(
		vec3( 1.1271005818144368, - 0.1413297634984383, - 0.14132976349843826 ),
		vec3( - 0.11060664309660323, 1.157823702216272, - 0.11060664309660294 ),
		vec3( - 0.016493938717834573, - 0.016493938717834257, 1.2519364065950405 )
	);
	const float AgxMinEv = - 12.47393;	const float AgxMaxEv = 4.026069;
	color *= toneMappingExposure;
	color = LINEAR_SRGB_TO_LINEAR_REC2020 * color;
	color = AgXInsetMatrix * color;
	color = max( color, 1e-10 );	color = log2( color );
	color = ( color - AgxMinEv ) / ( AgxMaxEv - AgxMinEv );
	color = clamp( color, 0.0, 1.0 );
	color = agxDefaultContrastApprox( color );
	color = AgXOutsetMatrix * color;
	color = pow( max( vec3( 0.0 ), color ), vec3( 2.2 ) );
	color = LINEAR_REC2020_TO_LINEAR_SRGB * color;
	color = clamp( color, 0.0, 1.0 );
	return color;
}
vec3 NeutralToneMapping( vec3 color ) {
	const float StartCompression = 0.8 - 0.04;
	const float Desaturation = 0.15;
	color *= toneMappingExposure;
	float x = min( color.r, min( color.g, color.b ) );
	float offset = x < 0.08 ? x - 6.25 * x * x : 0.04;
	color -= offset;
	float peak = max( color.r, max( color.g, color.b ) );
	if ( peak < StartCompression ) return color;
	float d = 1. - StartCompression;
	float newPeak = 1. - d * d / ( peak + d - StartCompression );
	color *= newPeak / peak;
	float g = 1. - 1. / ( Desaturation * ( peak - newPeak ) + 1. );
	return mix( color, vec3( newPeak ), g );
}
vec3 CustomToneMapping( vec3 color ) { return color; }`,vm=`#ifdef USE_TRANSMISSION
	material.transmission = transmission;
	material.transmissionAlpha = 1.0;
	material.thickness = thickness;
	material.attenuationDistance = attenuationDistance;
	material.attenuationColor = attenuationColor;
	#ifdef USE_TRANSMISSIONMAP
		material.transmission *= texture2D( transmissionMap, vTransmissionMapUv ).r;
	#endif
	#ifdef USE_THICKNESSMAP
		material.thickness *= texture2D( thicknessMap, vThicknessMapUv ).g;
	#endif
	vec3 pos = vWorldPosition;
	vec3 v = normalize( cameraPosition - pos );
	vec3 n = inverseTransformDirection( normal, viewMatrix );
	vec4 transmitted = getIBLVolumeRefraction(
		n, v, material.roughness, material.diffuseContribution, material.specularColorBlended, material.specularF90,
		pos, modelMatrix, viewMatrix, projectionMatrix, material.dispersion, material.ior, material.thickness,
		material.attenuationColor, material.attenuationDistance );
	material.transmissionAlpha = mix( material.transmissionAlpha, transmitted.a, material.transmission );
	totalDiffuse = mix( totalDiffuse, transmitted.rgb, material.transmission );
#endif`,Mm=`#ifdef USE_TRANSMISSION
	uniform float transmission;
	uniform float thickness;
	uniform float attenuationDistance;
	uniform vec3 attenuationColor;
	#ifdef USE_TRANSMISSIONMAP
		uniform sampler2D transmissionMap;
	#endif
	#ifdef USE_THICKNESSMAP
		uniform sampler2D thicknessMap;
	#endif
	uniform vec2 transmissionSamplerSize;
	uniform sampler2D transmissionSamplerMap;
	uniform mat4 modelMatrix;
	uniform mat4 projectionMatrix;
	varying vec3 vWorldPosition;
	float w0( float a ) {
		return ( 1.0 / 6.0 ) * ( a * ( a * ( - a + 3.0 ) - 3.0 ) + 1.0 );
	}
	float w1( float a ) {
		return ( 1.0 / 6.0 ) * ( a *  a * ( 3.0 * a - 6.0 ) + 4.0 );
	}
	float w2( float a ){
		return ( 1.0 / 6.0 ) * ( a * ( a * ( - 3.0 * a + 3.0 ) + 3.0 ) + 1.0 );
	}
	float w3( float a ) {
		return ( 1.0 / 6.0 ) * ( a * a * a );
	}
	float g0( float a ) {
		return w0( a ) + w1( a );
	}
	float g1( float a ) {
		return w2( a ) + w3( a );
	}
	float h0( float a ) {
		return - 1.0 + w1( a ) / ( w0( a ) + w1( a ) );
	}
	float h1( float a ) {
		return 1.0 + w3( a ) / ( w2( a ) + w3( a ) );
	}
	vec4 bicubic( sampler2D tex, vec2 uv, vec4 texelSize, float lod ) {
		uv = uv * texelSize.zw + 0.5;
		vec2 iuv = floor( uv );
		vec2 fuv = fract( uv );
		float g0x = g0( fuv.x );
		float g1x = g1( fuv.x );
		float h0x = h0( fuv.x );
		float h1x = h1( fuv.x );
		float h0y = h0( fuv.y );
		float h1y = h1( fuv.y );
		vec2 p0 = ( vec2( iuv.x + h0x, iuv.y + h0y ) - 0.5 ) * texelSize.xy;
		vec2 p1 = ( vec2( iuv.x + h1x, iuv.y + h0y ) - 0.5 ) * texelSize.xy;
		vec2 p2 = ( vec2( iuv.x + h0x, iuv.y + h1y ) - 0.5 ) * texelSize.xy;
		vec2 p3 = ( vec2( iuv.x + h1x, iuv.y + h1y ) - 0.5 ) * texelSize.xy;
		return g0( fuv.y ) * ( g0x * textureLod( tex, p0, lod ) + g1x * textureLod( tex, p1, lod ) ) +
			g1( fuv.y ) * ( g0x * textureLod( tex, p2, lod ) + g1x * textureLod( tex, p3, lod ) );
	}
	vec4 textureBicubic( sampler2D sampler, vec2 uv, float lod ) {
		vec2 fLodSize = vec2( textureSize( sampler, int( lod ) ) );
		vec2 cLodSize = vec2( textureSize( sampler, int( lod + 1.0 ) ) );
		vec2 fLodSizeInv = 1.0 / fLodSize;
		vec2 cLodSizeInv = 1.0 / cLodSize;
		vec4 fSample = bicubic( sampler, uv, vec4( fLodSizeInv, fLodSize ), floor( lod ) );
		vec4 cSample = bicubic( sampler, uv, vec4( cLodSizeInv, cLodSize ), ceil( lod ) );
		return mix( fSample, cSample, fract( lod ) );
	}
	vec3 getVolumeTransmissionRay( const in vec3 n, const in vec3 v, const in float thickness, const in float ior, const in mat4 modelMatrix ) {
		vec3 refractionVector = refract( - v, normalize( n ), 1.0 / ior );
		vec3 modelScale;
		modelScale.x = length( vec3( modelMatrix[ 0 ].xyz ) );
		modelScale.y = length( vec3( modelMatrix[ 1 ].xyz ) );
		modelScale.z = length( vec3( modelMatrix[ 2 ].xyz ) );
		return normalize( refractionVector ) * thickness * modelScale;
	}
	float applyIorToRoughness( const in float roughness, const in float ior ) {
		return roughness * clamp( ior * 2.0 - 2.0, 0.0, 1.0 );
	}
	vec4 getTransmissionSample( const in vec2 fragCoord, const in float roughness, const in float ior ) {
		float lod = log2( transmissionSamplerSize.x ) * applyIorToRoughness( roughness, ior );
		return textureBicubic( transmissionSamplerMap, fragCoord.xy, lod );
	}
	vec3 volumeAttenuation( const in float transmissionDistance, const in vec3 attenuationColor, const in float attenuationDistance ) {
		if ( isinf( attenuationDistance ) ) {
			return vec3( 1.0 );
		} else {
			vec3 attenuationCoefficient = -log( attenuationColor ) / attenuationDistance;
			vec3 transmittance = exp( - attenuationCoefficient * transmissionDistance );			return transmittance;
		}
	}
	vec4 getIBLVolumeRefraction( const in vec3 n, const in vec3 v, const in float roughness, const in vec3 diffuseColor,
		const in vec3 specularColor, const in float specularF90, const in vec3 position, const in mat4 modelMatrix,
		const in mat4 viewMatrix, const in mat4 projMatrix, const in float dispersion, const in float ior, const in float thickness,
		const in vec3 attenuationColor, const in float attenuationDistance ) {
		vec4 transmittedLight;
		vec3 transmittance;
		#ifdef USE_DISPERSION
			float halfSpread = ( ior - 1.0 ) * 0.025 * dispersion;
			vec3 iors = vec3( ior - halfSpread, ior, ior + halfSpread );
			for ( int i = 0; i < 3; i ++ ) {
				vec3 transmissionRay = getVolumeTransmissionRay( n, v, thickness, iors[ i ], modelMatrix );
				vec3 refractedRayExit = position + transmissionRay;
				vec4 ndcPos = projMatrix * viewMatrix * vec4( refractedRayExit, 1.0 );
				vec2 refractionCoords = ndcPos.xy / ndcPos.w;
				refractionCoords += 1.0;
				refractionCoords /= 2.0;
				vec4 transmissionSample = getTransmissionSample( refractionCoords, roughness, iors[ i ] );
				transmittedLight[ i ] = transmissionSample[ i ];
				transmittedLight.a += transmissionSample.a;
				transmittance[ i ] = diffuseColor[ i ] * volumeAttenuation( length( transmissionRay ), attenuationColor, attenuationDistance )[ i ];
			}
			transmittedLight.a /= 3.0;
		#else
			vec3 transmissionRay = getVolumeTransmissionRay( n, v, thickness, ior, modelMatrix );
			vec3 refractedRayExit = position + transmissionRay;
			vec4 ndcPos = projMatrix * viewMatrix * vec4( refractedRayExit, 1.0 );
			vec2 refractionCoords = ndcPos.xy / ndcPos.w;
			refractionCoords += 1.0;
			refractionCoords /= 2.0;
			transmittedLight = getTransmissionSample( refractionCoords, roughness, ior );
			transmittance = diffuseColor * volumeAttenuation( length( transmissionRay ), attenuationColor, attenuationDistance );
		#endif
		vec3 attenuatedColor = transmittance * transmittedLight.rgb;
		vec3 F = EnvironmentBRDF( n, v, specularColor, specularF90, roughness );
		float transmittanceFactor = ( transmittance.r + transmittance.g + transmittance.b ) / 3.0;
		return vec4( ( 1.0 - F ) * attenuatedColor, 1.0 - ( 1.0 - transmittedLight.a ) * transmittanceFactor );
	}
#endif`,Sm=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	varying vec2 vUv;
#endif
#ifdef USE_MAP
	varying vec2 vMapUv;
#endif
#ifdef USE_ALPHAMAP
	varying vec2 vAlphaMapUv;
#endif
#ifdef USE_LIGHTMAP
	varying vec2 vLightMapUv;
#endif
#ifdef USE_AOMAP
	varying vec2 vAoMapUv;
#endif
#ifdef USE_BUMPMAP
	varying vec2 vBumpMapUv;
#endif
#ifdef USE_NORMALMAP
	varying vec2 vNormalMapUv;
#endif
#ifdef USE_EMISSIVEMAP
	varying vec2 vEmissiveMapUv;
#endif
#ifdef USE_METALNESSMAP
	varying vec2 vMetalnessMapUv;
#endif
#ifdef USE_ROUGHNESSMAP
	varying vec2 vRoughnessMapUv;
#endif
#ifdef USE_ANISOTROPYMAP
	varying vec2 vAnisotropyMapUv;
#endif
#ifdef USE_CLEARCOATMAP
	varying vec2 vClearcoatMapUv;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	varying vec2 vClearcoatNormalMapUv;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	varying vec2 vClearcoatRoughnessMapUv;
#endif
#ifdef USE_IRIDESCENCEMAP
	varying vec2 vIridescenceMapUv;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	varying vec2 vIridescenceThicknessMapUv;
#endif
#ifdef USE_SHEEN_COLORMAP
	varying vec2 vSheenColorMapUv;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	varying vec2 vSheenRoughnessMapUv;
#endif
#ifdef USE_SPECULARMAP
	varying vec2 vSpecularMapUv;
#endif
#ifdef USE_SPECULAR_COLORMAP
	varying vec2 vSpecularColorMapUv;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	varying vec2 vSpecularIntensityMapUv;
#endif
#ifdef USE_TRANSMISSIONMAP
	uniform mat3 transmissionMapTransform;
	varying vec2 vTransmissionMapUv;
#endif
#ifdef USE_THICKNESSMAP
	uniform mat3 thicknessMapTransform;
	varying vec2 vThicknessMapUv;
#endif`,ym=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	varying vec2 vUv;
#endif
#ifdef USE_MAP
	uniform mat3 mapTransform;
	varying vec2 vMapUv;
#endif
#ifdef USE_ALPHAMAP
	uniform mat3 alphaMapTransform;
	varying vec2 vAlphaMapUv;
#endif
#ifdef USE_LIGHTMAP
	uniform mat3 lightMapTransform;
	varying vec2 vLightMapUv;
#endif
#ifdef USE_AOMAP
	uniform mat3 aoMapTransform;
	varying vec2 vAoMapUv;
#endif
#ifdef USE_BUMPMAP
	uniform mat3 bumpMapTransform;
	varying vec2 vBumpMapUv;
#endif
#ifdef USE_NORMALMAP
	uniform mat3 normalMapTransform;
	varying vec2 vNormalMapUv;
#endif
#ifdef USE_DISPLACEMENTMAP
	uniform mat3 displacementMapTransform;
	varying vec2 vDisplacementMapUv;
#endif
#ifdef USE_EMISSIVEMAP
	uniform mat3 emissiveMapTransform;
	varying vec2 vEmissiveMapUv;
#endif
#ifdef USE_METALNESSMAP
	uniform mat3 metalnessMapTransform;
	varying vec2 vMetalnessMapUv;
#endif
#ifdef USE_ROUGHNESSMAP
	uniform mat3 roughnessMapTransform;
	varying vec2 vRoughnessMapUv;
#endif
#ifdef USE_ANISOTROPYMAP
	uniform mat3 anisotropyMapTransform;
	varying vec2 vAnisotropyMapUv;
#endif
#ifdef USE_CLEARCOATMAP
	uniform mat3 clearcoatMapTransform;
	varying vec2 vClearcoatMapUv;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform mat3 clearcoatNormalMapTransform;
	varying vec2 vClearcoatNormalMapUv;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform mat3 clearcoatRoughnessMapTransform;
	varying vec2 vClearcoatRoughnessMapUv;
#endif
#ifdef USE_SHEEN_COLORMAP
	uniform mat3 sheenColorMapTransform;
	varying vec2 vSheenColorMapUv;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	uniform mat3 sheenRoughnessMapTransform;
	varying vec2 vSheenRoughnessMapUv;
#endif
#ifdef USE_IRIDESCENCEMAP
	uniform mat3 iridescenceMapTransform;
	varying vec2 vIridescenceMapUv;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform mat3 iridescenceThicknessMapTransform;
	varying vec2 vIridescenceThicknessMapUv;
#endif
#ifdef USE_SPECULARMAP
	uniform mat3 specularMapTransform;
	varying vec2 vSpecularMapUv;
#endif
#ifdef USE_SPECULAR_COLORMAP
	uniform mat3 specularColorMapTransform;
	varying vec2 vSpecularColorMapUv;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	uniform mat3 specularIntensityMapTransform;
	varying vec2 vSpecularIntensityMapUv;
#endif
#ifdef USE_TRANSMISSIONMAP
	uniform mat3 transmissionMapTransform;
	varying vec2 vTransmissionMapUv;
#endif
#ifdef USE_THICKNESSMAP
	uniform mat3 thicknessMapTransform;
	varying vec2 vThicknessMapUv;
#endif`,Em=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	vUv = vec3( uv, 1 ).xy;
#endif
#ifdef USE_MAP
	vMapUv = ( mapTransform * vec3( MAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ALPHAMAP
	vAlphaMapUv = ( alphaMapTransform * vec3( ALPHAMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_LIGHTMAP
	vLightMapUv = ( lightMapTransform * vec3( LIGHTMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_AOMAP
	vAoMapUv = ( aoMapTransform * vec3( AOMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_BUMPMAP
	vBumpMapUv = ( bumpMapTransform * vec3( BUMPMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_NORMALMAP
	vNormalMapUv = ( normalMapTransform * vec3( NORMALMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_DISPLACEMENTMAP
	vDisplacementMapUv = ( displacementMapTransform * vec3( DISPLACEMENTMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_EMISSIVEMAP
	vEmissiveMapUv = ( emissiveMapTransform * vec3( EMISSIVEMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_METALNESSMAP
	vMetalnessMapUv = ( metalnessMapTransform * vec3( METALNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ROUGHNESSMAP
	vRoughnessMapUv = ( roughnessMapTransform * vec3( ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ANISOTROPYMAP
	vAnisotropyMapUv = ( anisotropyMapTransform * vec3( ANISOTROPYMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOATMAP
	vClearcoatMapUv = ( clearcoatMapTransform * vec3( CLEARCOATMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	vClearcoatNormalMapUv = ( clearcoatNormalMapTransform * vec3( CLEARCOAT_NORMALMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	vClearcoatRoughnessMapUv = ( clearcoatRoughnessMapTransform * vec3( CLEARCOAT_ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_IRIDESCENCEMAP
	vIridescenceMapUv = ( iridescenceMapTransform * vec3( IRIDESCENCEMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	vIridescenceThicknessMapUv = ( iridescenceThicknessMapTransform * vec3( IRIDESCENCE_THICKNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SHEEN_COLORMAP
	vSheenColorMapUv = ( sheenColorMapTransform * vec3( SHEEN_COLORMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	vSheenRoughnessMapUv = ( sheenRoughnessMapTransform * vec3( SHEEN_ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULARMAP
	vSpecularMapUv = ( specularMapTransform * vec3( SPECULARMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULAR_COLORMAP
	vSpecularColorMapUv = ( specularColorMapTransform * vec3( SPECULAR_COLORMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	vSpecularIntensityMapUv = ( specularIntensityMapTransform * vec3( SPECULAR_INTENSITYMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_TRANSMISSIONMAP
	vTransmissionMapUv = ( transmissionMapTransform * vec3( TRANSMISSIONMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_THICKNESSMAP
	vThicknessMapUv = ( thicknessMapTransform * vec3( THICKNESSMAP_UV, 1 ) ).xy;
#endif`,bm=`#if defined( USE_ENVMAP ) || defined( DISTANCE ) || defined ( USE_SHADOWMAP ) || defined ( USE_TRANSMISSION ) || NUM_SPOT_LIGHT_COORDS > 0
	vec4 worldPosition = vec4( transformed, 1.0 );
	#ifdef USE_BATCHING
		worldPosition = batchingMatrix * worldPosition;
	#endif
	#ifdef USE_INSTANCING
		worldPosition = instanceMatrix * worldPosition;
	#endif
	worldPosition = modelMatrix * worldPosition;
#endif`;const Tm=`varying vec2 vUv;
uniform mat3 uvTransform;
void main() {
	vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	gl_Position = vec4( position.xy, 1.0, 1.0 );
}`,Am=`uniform sampler2D t2D;
uniform float backgroundIntensity;
varying vec2 vUv;
void main() {
	vec4 texColor = texture2D( t2D, vUv );
	#ifdef DECODE_VIDEO_TEXTURE
		texColor = vec4( mix( pow( texColor.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), texColor.rgb * 0.0773993808, vec3( lessThanEqual( texColor.rgb, vec3( 0.04045 ) ) ) ), texColor.w );
	#endif
	texColor.rgb *= backgroundIntensity;
	gl_FragColor = texColor;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,wm=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Rm=`#ifdef ENVMAP_TYPE_CUBE
	uniform samplerCube envMap;
#elif defined( ENVMAP_TYPE_CUBE_UV )
	uniform sampler2D envMap;
#endif
uniform float backgroundBlurriness;
uniform float backgroundIntensity;
uniform mat3 backgroundRotation;
varying vec3 vWorldDirection;
#include <cube_uv_reflection_fragment>
void main() {
	#ifdef ENVMAP_TYPE_CUBE
		vec4 texColor = textureCube( envMap, backgroundRotation * vWorldDirection );
	#elif defined( ENVMAP_TYPE_CUBE_UV )
		vec4 texColor = textureCubeUV( envMap, backgroundRotation * vWorldDirection, backgroundBlurriness );
	#else
		vec4 texColor = vec4( 0.0, 0.0, 0.0, 1.0 );
	#endif
	texColor.rgb *= backgroundIntensity;
	gl_FragColor = texColor;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Cm=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Pm=`uniform samplerCube tCube;
uniform float tFlip;
uniform float opacity;
varying vec3 vWorldDirection;
void main() {
	vec4 texColor = textureCube( tCube, vec3( tFlip * vWorldDirection.x, vWorldDirection.yz ) );
	gl_FragColor = texColor;
	gl_FragColor.a *= opacity;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Lm=`#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
varying vec2 vHighPrecisionZW;
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <skinbase_vertex>
	#include <morphinstance_vertex>
	#ifdef USE_DISPLACEMENTMAP
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vHighPrecisionZW = gl_Position.zw;
}`,Dm=`#if DEPTH_PACKING == 3200
	uniform float opacity;
#endif
#include <common>
#include <packing>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
varying vec2 vHighPrecisionZW;
void main() {
	vec4 diffuseColor = vec4( 1.0 );
	#include <clipping_planes_fragment>
	#if DEPTH_PACKING == 3200
		diffuseColor.a = opacity;
	#endif
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <logdepthbuf_fragment>
	#ifdef USE_REVERSED_DEPTH_BUFFER
		float fragCoordZ = vHighPrecisionZW[ 0 ] / vHighPrecisionZW[ 1 ];
	#else
		float fragCoordZ = 0.5 * vHighPrecisionZW[ 0 ] / vHighPrecisionZW[ 1 ] + 0.5;
	#endif
	#if DEPTH_PACKING == 3200
		gl_FragColor = vec4( vec3( 1.0 - fragCoordZ ), opacity );
	#elif DEPTH_PACKING == 3201
		gl_FragColor = packDepthToRGBA( fragCoordZ );
	#elif DEPTH_PACKING == 3202
		gl_FragColor = vec4( packDepthToRGB( fragCoordZ ), 1.0 );
	#elif DEPTH_PACKING == 3203
		gl_FragColor = vec4( packDepthToRG( fragCoordZ ), 0.0, 1.0 );
	#endif
}`,Im=`#define DISTANCE
varying vec3 vWorldPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <skinbase_vertex>
	#include <morphinstance_vertex>
	#ifdef USE_DISPLACEMENTMAP
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <worldpos_vertex>
	#include <clipping_planes_vertex>
	vWorldPosition = worldPosition.xyz;
}`,Um=`#define DISTANCE
uniform vec3 referencePosition;
uniform float nearDistance;
uniform float farDistance;
varying vec3 vWorldPosition;
#include <common>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <clipping_planes_pars_fragment>
void main () {
	vec4 diffuseColor = vec4( 1.0 );
	#include <clipping_planes_fragment>
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	float dist = length( vWorldPosition - referencePosition );
	dist = ( dist - nearDistance ) / ( farDistance - nearDistance );
	dist = saturate( dist );
	gl_FragColor = vec4( dist, 0.0, 0.0, 1.0 );
}`,Nm=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
}`,Fm=`uniform sampler2D tEquirect;
varying vec3 vWorldDirection;
#include <common>
void main() {
	vec3 direction = normalize( vWorldDirection );
	vec2 sampleUV = equirectUv( direction );
	gl_FragColor = texture2D( tEquirect, sampleUV );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Om=`uniform float scale;
attribute float lineDistance;
varying float vLineDistance;
#include <common>
#include <uv_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	vLineDistance = scale * lineDistance;
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
}`,Bm=`uniform vec3 diffuse;
uniform float opacity;
uniform float dashSize;
uniform float totalSize;
varying float vLineDistance;
#include <common>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	if ( mod( vLineDistance, totalSize ) > dashSize ) {
		discard;
	}
	vec3 outgoingLight = vec3( 0.0 );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,zm=`#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#if defined ( USE_ENVMAP ) || defined ( USE_SKINNING )
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinbase_vertex>
		#include <skinnormal_vertex>
		#include <defaultnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <fog_vertex>
}`,km=`uniform vec3 diffuse;
uniform float opacity;
#ifndef FLAT_SHADED
	varying vec3 vNormal;
#endif
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <fog_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	#ifdef USE_LIGHTMAP
		vec4 lightMapTexel = texture2D( lightMap, vLightMapUv );
		reflectedLight.indirectDiffuse += lightMapTexel.rgb * lightMapIntensity * RECIPROCAL_PI;
	#else
		reflectedLight.indirectDiffuse += vec3( 1.0 );
	#endif
	#include <aomap_fragment>
	reflectedLight.indirectDiffuse *= diffuseColor.rgb;
	vec3 outgoingLight = reflectedLight.indirectDiffuse;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,Hm=`#define LAMBERT
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,Gm=`#define LAMBERT
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float opacity;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <cube_uv_reflection_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <envmap_physical_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_lambert_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_lambert_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + totalEmissiveRadiance;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,Vm=`#define MATCAP
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <color_pars_vertex>
#include <displacementmap_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
	vViewPosition = - mvPosition.xyz;
}`,Wm=`#define MATCAP
uniform vec3 diffuse;
uniform float opacity;
uniform sampler2D matcap;
varying vec3 vViewPosition;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <normal_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	vec3 viewDir = normalize( vViewPosition );
	vec3 x = normalize( vec3( viewDir.z, 0.0, - viewDir.x ) );
	vec3 y = cross( viewDir, x );
	vec2 uv = vec2( dot( x, normal ), dot( y, normal ) ) * 0.495 + 0.5;
	#ifdef USE_MATCAP
		vec4 matcapColor = texture2D( matcap, uv );
	#else
		vec4 matcapColor = vec4( vec3( mix( 0.2, 0.8, uv.y ) ), 1.0 );
	#endif
	vec3 outgoingLight = diffuseColor.rgb * matcapColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,Xm=`#define NORMAL
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	varying vec3 vViewPosition;
#endif
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphinstance_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	vViewPosition = - mvPosition.xyz;
#endif
}`,$m=`#define NORMAL
uniform float opacity;
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	varying vec3 vViewPosition;
#endif
#include <uv_pars_fragment>
#include <normal_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( 0.0, 0.0, 0.0, opacity );
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	gl_FragColor = vec4( normalize( normal ) * 0.5 + 0.5, diffuseColor.a );
	#ifdef OPAQUE
		gl_FragColor.a = 1.0;
	#endif
}`,qm=`#define PHONG
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphinstance_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,Ym=`#define PHONG
uniform vec3 diffuse;
uniform vec3 emissive;
uniform vec3 specular;
uniform float shininess;
uniform float opacity;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <cube_uv_reflection_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <envmap_physical_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_phong_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_phong_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + reflectedLight.directSpecular + reflectedLight.indirectSpecular + totalEmissiveRadiance;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,jm=`#define STANDARD
varying vec3 vViewPosition;
#ifdef USE_TRANSMISSION
	varying vec3 vWorldPosition;
#endif
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
#ifdef USE_TRANSMISSION
	vWorldPosition = worldPosition.xyz;
#endif
}`,Km=`#define STANDARD
#ifdef PHYSICAL
	#define IOR
	#define USE_SPECULAR
#endif
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float roughness;
uniform float metalness;
uniform float opacity;
#ifdef IOR
	uniform float ior;
#endif
#ifdef USE_SPECULAR
	uniform float specularIntensity;
	uniform vec3 specularColor;
	#ifdef USE_SPECULAR_COLORMAP
		uniform sampler2D specularColorMap;
	#endif
	#ifdef USE_SPECULAR_INTENSITYMAP
		uniform sampler2D specularIntensityMap;
	#endif
#endif
#ifdef USE_CLEARCOAT
	uniform float clearcoat;
	uniform float clearcoatRoughness;
#endif
#ifdef USE_DISPERSION
	uniform float dispersion;
#endif
#ifdef USE_IRIDESCENCE
	uniform float iridescence;
	uniform float iridescenceIOR;
	uniform float iridescenceThicknessMinimum;
	uniform float iridescenceThicknessMaximum;
#endif
#ifdef USE_SHEEN
	uniform vec3 sheenColor;
	uniform float sheenRoughness;
	#ifdef USE_SHEEN_COLORMAP
		uniform sampler2D sheenColorMap;
	#endif
	#ifdef USE_SHEEN_ROUGHNESSMAP
		uniform sampler2D sheenRoughnessMap;
	#endif
#endif
#ifdef USE_ANISOTROPY
	uniform vec2 anisotropyVector;
	#ifdef USE_ANISOTROPYMAP
		uniform sampler2D anisotropyMap;
	#endif
#endif
varying vec3 vViewPosition;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <iridescence_fragment>
#include <cube_uv_reflection_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_physical_pars_fragment>
#include <fog_pars_fragment>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_physical_pars_fragment>
#include <transmission_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <clearcoat_pars_fragment>
#include <iridescence_pars_fragment>
#include <roughnessmap_pars_fragment>
#include <metalnessmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <roughnessmap_fragment>
	#include <metalnessmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <clearcoat_normal_fragment_begin>
	#include <clearcoat_normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_physical_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 totalDiffuse = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse;
	vec3 totalSpecular = reflectedLight.directSpecular + reflectedLight.indirectSpecular;
	#include <transmission_fragment>
	vec3 outgoingLight = totalDiffuse + totalSpecular + totalEmissiveRadiance;
	#ifdef USE_SHEEN
 
		outgoingLight = outgoingLight + sheenSpecularDirect + sheenSpecularIndirect;
 
 	#endif
	#ifdef USE_CLEARCOAT
		float dotNVcc = saturate( dot( geometryClearcoatNormal, geometryViewDir ) );
		vec3 Fcc = F_Schlick( material.clearcoatF0, material.clearcoatF90, dotNVcc );
		outgoingLight = outgoingLight * ( 1.0 - material.clearcoat * Fcc ) + ( clearcoatSpecularDirect + clearcoatSpecularIndirect ) * material.clearcoat;
	#endif
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,Zm=`#define TOON
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,Jm=`#define TOON
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float opacity;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <gradientmap_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_toon_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_toon_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + totalEmissiveRadiance;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,Qm=`uniform float size;
uniform float scale;
#include <common>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
#ifdef USE_POINTS_UV
	varying vec2 vUv;
	uniform mat3 uvTransform;
#endif
void main() {
	#ifdef USE_POINTS_UV
		vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	#endif
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <project_vertex>
	gl_PointSize = size;
	#ifdef USE_SIZEATTENUATION
		bool isPerspective = isPerspectiveMatrix( projectionMatrix );
		if ( isPerspective ) gl_PointSize *= ( scale / - mvPosition.z );
	#endif
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <worldpos_vertex>
	#include <fog_vertex>
}`,tg=`uniform vec3 diffuse;
uniform float opacity;
#include <common>
#include <color_pars_fragment>
#include <map_particle_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	vec3 outgoingLight = vec3( 0.0 );
	#include <logdepthbuf_fragment>
	#include <map_particle_fragment>
	#include <color_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,eg=`#include <common>
#include <batching_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <shadowmap_pars_vertex>
void main() {
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphinstance_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,ng=`uniform vec3 color;
uniform float opacity;
#include <common>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <logdepthbuf_pars_fragment>
#include <shadowmap_pars_fragment>
#include <shadowmask_pars_fragment>
void main() {
	#include <logdepthbuf_fragment>
	gl_FragColor = vec4( color, opacity * ( 1.0 - getShadowMask() ) );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,ig=`uniform float rotation;
uniform vec2 center;
#include <common>
#include <uv_pars_vertex>
#include <fog_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	vec4 mvPosition = modelViewMatrix[ 3 ];
	vec2 scale = vec2( length( modelMatrix[ 0 ].xyz ), length( modelMatrix[ 1 ].xyz ) );
	#ifndef USE_SIZEATTENUATION
		bool isPerspective = isPerspectiveMatrix( projectionMatrix );
		if ( isPerspective ) scale *= - mvPosition.z;
	#endif
	vec2 alignedPosition = ( position.xy - ( center - vec2( 0.5 ) ) ) * scale;
	vec2 rotatedPosition;
	rotatedPosition.x = cos( rotation ) * alignedPosition.x - sin( rotation ) * alignedPosition.y;
	rotatedPosition.y = sin( rotation ) * alignedPosition.x + cos( rotation ) * alignedPosition.y;
	mvPosition.xy += rotatedPosition;
	gl_Position = projectionMatrix * mvPosition;
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
}`,sg=`uniform vec3 diffuse;
uniform float opacity;
#include <common>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	vec3 outgoingLight = vec3( 0.0 );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
}`,Xt={alphahash_fragment:Tf,alphahash_pars_fragment:Af,alphamap_fragment:wf,alphamap_pars_fragment:Rf,alphatest_fragment:Cf,alphatest_pars_fragment:Pf,aomap_fragment:Lf,aomap_pars_fragment:Df,batching_pars_vertex:If,batching_vertex:Uf,begin_vertex:Nf,beginnormal_vertex:Ff,bsdfs:Of,iridescence_fragment:Bf,bumpmap_pars_fragment:zf,clipping_planes_fragment:kf,clipping_planes_pars_fragment:Hf,clipping_planes_pars_vertex:Gf,clipping_planes_vertex:Vf,color_fragment:Wf,color_pars_fragment:Xf,color_pars_vertex:$f,color_vertex:qf,common:Yf,cube_uv_reflection_fragment:jf,defaultnormal_vertex:Kf,displacementmap_pars_vertex:Zf,displacementmap_vertex:Jf,emissivemap_fragment:Qf,emissivemap_pars_fragment:tp,colorspace_fragment:ep,colorspace_pars_fragment:np,envmap_fragment:ip,envmap_common_pars_fragment:sp,envmap_pars_fragment:rp,envmap_pars_vertex:ap,envmap_physical_pars_fragment:_p,envmap_vertex:op,fog_vertex:lp,fog_pars_vertex:cp,fog_fragment:hp,fog_pars_fragment:up,gradientmap_pars_fragment:dp,lightmap_pars_fragment:fp,lights_lambert_fragment:pp,lights_lambert_pars_fragment:mp,lights_pars_begin:gp,lights_toon_fragment:xp,lights_toon_pars_fragment:vp,lights_phong_fragment:Mp,lights_phong_pars_fragment:Sp,lights_physical_fragment:yp,lights_physical_pars_fragment:Ep,lights_fragment_begin:bp,lights_fragment_maps:Tp,lights_fragment_end:Ap,lightprobes_pars_fragment:wp,logdepthbuf_fragment:Rp,logdepthbuf_pars_fragment:Cp,logdepthbuf_pars_vertex:Pp,logdepthbuf_vertex:Lp,map_fragment:Dp,map_pars_fragment:Ip,map_particle_fragment:Up,map_particle_pars_fragment:Np,metalnessmap_fragment:Fp,metalnessmap_pars_fragment:Op,morphinstance_vertex:Bp,morphcolor_vertex:zp,morphnormal_vertex:kp,morphtarget_pars_vertex:Hp,morphtarget_vertex:Gp,normal_fragment_begin:Vp,normal_fragment_maps:Wp,normal_pars_fragment:Xp,normal_pars_vertex:$p,normal_vertex:qp,normalmap_pars_fragment:Yp,clearcoat_normal_fragment_begin:jp,clearcoat_normal_fragment_maps:Kp,clearcoat_pars_fragment:Zp,iridescence_pars_fragment:Jp,opaque_fragment:Qp,packing:tm,premultiplied_alpha_fragment:em,project_vertex:nm,dithering_fragment:im,dithering_pars_fragment:sm,roughnessmap_fragment:rm,roughnessmap_pars_fragment:am,shadowmap_pars_fragment:om,shadowmap_pars_vertex:lm,shadowmap_vertex:cm,shadowmask_pars_fragment:hm,skinbase_vertex:um,skinning_pars_vertex:dm,skinning_vertex:fm,skinnormal_vertex:pm,specularmap_fragment:mm,specularmap_pars_fragment:gm,tonemapping_fragment:_m,tonemapping_pars_fragment:xm,transmission_fragment:vm,transmission_pars_fragment:Mm,uv_pars_fragment:Sm,uv_pars_vertex:ym,uv_vertex:Em,worldpos_vertex:bm,background_vert:Tm,background_frag:Am,backgroundCube_vert:wm,backgroundCube_frag:Rm,cube_vert:Cm,cube_frag:Pm,depth_vert:Lm,depth_frag:Dm,distance_vert:Im,distance_frag:Um,equirect_vert:Nm,equirect_frag:Fm,linedashed_vert:Om,linedashed_frag:Bm,meshbasic_vert:zm,meshbasic_frag:km,meshlambert_vert:Hm,meshlambert_frag:Gm,meshmatcap_vert:Vm,meshmatcap_frag:Wm,meshnormal_vert:Xm,meshnormal_frag:$m,meshphong_vert:qm,meshphong_frag:Ym,meshphysical_vert:jm,meshphysical_frag:Km,meshtoon_vert:Zm,meshtoon_frag:Jm,points_vert:Qm,points_frag:tg,shadow_vert:eg,shadow_frag:ng,sprite_vert:ig,sprite_frag:sg},mt={common:{diffuse:{value:new Bt(16777215)},opacity:{value:1},map:{value:null},mapTransform:{value:new zt},alphaMap:{value:null},alphaMapTransform:{value:new zt},alphaTest:{value:0}},specularmap:{specularMap:{value:null},specularMapTransform:{value:new zt}},envmap:{envMap:{value:null},envMapRotation:{value:new zt},reflectivity:{value:1},ior:{value:1.5},refractionRatio:{value:.98},dfgLUT:{value:null}},aomap:{aoMap:{value:null},aoMapIntensity:{value:1},aoMapTransform:{value:new zt}},lightmap:{lightMap:{value:null},lightMapIntensity:{value:1},lightMapTransform:{value:new zt}},bumpmap:{bumpMap:{value:null},bumpMapTransform:{value:new zt},bumpScale:{value:1}},normalmap:{normalMap:{value:null},normalMapTransform:{value:new zt},normalScale:{value:new Nt(1,1)}},displacementmap:{displacementMap:{value:null},displacementMapTransform:{value:new zt},displacementScale:{value:1},displacementBias:{value:0}},emissivemap:{emissiveMap:{value:null},emissiveMapTransform:{value:new zt}},metalnessmap:{metalnessMap:{value:null},metalnessMapTransform:{value:new zt}},roughnessmap:{roughnessMap:{value:null},roughnessMapTransform:{value:new zt}},gradientmap:{gradientMap:{value:null}},fog:{fogDensity:{value:25e-5},fogNear:{value:1},fogFar:{value:2e3},fogColor:{value:new Bt(16777215)}},lights:{ambientLightColor:{value:[]},lightProbe:{value:[]},directionalLights:{value:[],properties:{direction:{},color:{}}},directionalLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},directionalShadowMatrix:{value:[]},spotLights:{value:[],properties:{color:{},position:{},direction:{},distance:{},coneCos:{},penumbraCos:{},decay:{}}},spotLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},spotLightMap:{value:[]},spotLightMatrix:{value:[]},pointLights:{value:[],properties:{color:{},position:{},decay:{},distance:{}}},pointLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{},shadowCameraNear:{},shadowCameraFar:{}}},pointShadowMatrix:{value:[]},hemisphereLights:{value:[],properties:{direction:{},skyColor:{},groundColor:{}}},rectAreaLights:{value:[],properties:{color:{},position:{},width:{},height:{}}},ltc_1:{value:null},ltc_2:{value:null},probesSH:{value:null},probesMin:{value:new F},probesMax:{value:new F},probesResolution:{value:new F}},points:{diffuse:{value:new Bt(16777215)},opacity:{value:1},size:{value:1},scale:{value:1},map:{value:null},alphaMap:{value:null},alphaMapTransform:{value:new zt},alphaTest:{value:0},uvTransform:{value:new zt}},sprite:{diffuse:{value:new Bt(16777215)},opacity:{value:1},center:{value:new Nt(.5,.5)},rotation:{value:0},map:{value:null},mapTransform:{value:new zt},alphaMap:{value:null},alphaMapTransform:{value:new zt},alphaTest:{value:0}}},En={basic:{uniforms:Ge([mt.common,mt.specularmap,mt.envmap,mt.aomap,mt.lightmap,mt.fog]),vertexShader:Xt.meshbasic_vert,fragmentShader:Xt.meshbasic_frag},lambert:{uniforms:Ge([mt.common,mt.specularmap,mt.envmap,mt.aomap,mt.lightmap,mt.emissivemap,mt.bumpmap,mt.normalmap,mt.displacementmap,mt.fog,mt.lights,{emissive:{value:new Bt(0)},envMapIntensity:{value:1}}]),vertexShader:Xt.meshlambert_vert,fragmentShader:Xt.meshlambert_frag},phong:{uniforms:Ge([mt.common,mt.specularmap,mt.envmap,mt.aomap,mt.lightmap,mt.emissivemap,mt.bumpmap,mt.normalmap,mt.displacementmap,mt.fog,mt.lights,{emissive:{value:new Bt(0)},specular:{value:new Bt(1118481)},shininess:{value:30},envMapIntensity:{value:1}}]),vertexShader:Xt.meshphong_vert,fragmentShader:Xt.meshphong_frag},standard:{uniforms:Ge([mt.common,mt.envmap,mt.aomap,mt.lightmap,mt.emissivemap,mt.bumpmap,mt.normalmap,mt.displacementmap,mt.roughnessmap,mt.metalnessmap,mt.fog,mt.lights,{emissive:{value:new Bt(0)},roughness:{value:1},metalness:{value:0},envMapIntensity:{value:1}}]),vertexShader:Xt.meshphysical_vert,fragmentShader:Xt.meshphysical_frag},toon:{uniforms:Ge([mt.common,mt.aomap,mt.lightmap,mt.emissivemap,mt.bumpmap,mt.normalmap,mt.displacementmap,mt.gradientmap,mt.fog,mt.lights,{emissive:{value:new Bt(0)}}]),vertexShader:Xt.meshtoon_vert,fragmentShader:Xt.meshtoon_frag},matcap:{uniforms:Ge([mt.common,mt.bumpmap,mt.normalmap,mt.displacementmap,mt.fog,{matcap:{value:null}}]),vertexShader:Xt.meshmatcap_vert,fragmentShader:Xt.meshmatcap_frag},points:{uniforms:Ge([mt.points,mt.fog]),vertexShader:Xt.points_vert,fragmentShader:Xt.points_frag},dashed:{uniforms:Ge([mt.common,mt.fog,{scale:{value:1},dashSize:{value:1},totalSize:{value:2}}]),vertexShader:Xt.linedashed_vert,fragmentShader:Xt.linedashed_frag},depth:{uniforms:Ge([mt.common,mt.displacementmap]),vertexShader:Xt.depth_vert,fragmentShader:Xt.depth_frag},normal:{uniforms:Ge([mt.common,mt.bumpmap,mt.normalmap,mt.displacementmap,{opacity:{value:1}}]),vertexShader:Xt.meshnormal_vert,fragmentShader:Xt.meshnormal_frag},sprite:{uniforms:Ge([mt.sprite,mt.fog]),vertexShader:Xt.sprite_vert,fragmentShader:Xt.sprite_frag},background:{uniforms:{uvTransform:{value:new zt},t2D:{value:null},backgroundIntensity:{value:1}},vertexShader:Xt.background_vert,fragmentShader:Xt.background_frag},backgroundCube:{uniforms:{envMap:{value:null},backgroundBlurriness:{value:0},backgroundIntensity:{value:1},backgroundRotation:{value:new zt}},vertexShader:Xt.backgroundCube_vert,fragmentShader:Xt.backgroundCube_frag},cube:{uniforms:{tCube:{value:null},tFlip:{value:-1},opacity:{value:1}},vertexShader:Xt.cube_vert,fragmentShader:Xt.cube_frag},equirect:{uniforms:{tEquirect:{value:null}},vertexShader:Xt.equirect_vert,fragmentShader:Xt.equirect_frag},distance:{uniforms:Ge([mt.common,mt.displacementmap,{referencePosition:{value:new F},nearDistance:{value:1},farDistance:{value:1e3}}]),vertexShader:Xt.distance_vert,fragmentShader:Xt.distance_frag},shadow:{uniforms:Ge([mt.lights,mt.fog,{color:{value:new Bt(0)},opacity:{value:1}}]),vertexShader:Xt.shadow_vert,fragmentShader:Xt.shadow_frag}};En.physical={uniforms:Ge([En.standard.uniforms,{clearcoat:{value:0},clearcoatMap:{value:null},clearcoatMapTransform:{value:new zt},clearcoatNormalMap:{value:null},clearcoatNormalMapTransform:{value:new zt},clearcoatNormalScale:{value:new Nt(1,1)},clearcoatRoughness:{value:0},clearcoatRoughnessMap:{value:null},clearcoatRoughnessMapTransform:{value:new zt},dispersion:{value:0},iridescence:{value:0},iridescenceMap:{value:null},iridescenceMapTransform:{value:new zt},iridescenceIOR:{value:1.3},iridescenceThicknessMinimum:{value:100},iridescenceThicknessMaximum:{value:400},iridescenceThicknessMap:{value:null},iridescenceThicknessMapTransform:{value:new zt},sheen:{value:0},sheenColor:{value:new Bt(0)},sheenColorMap:{value:null},sheenColorMapTransform:{value:new zt},sheenRoughness:{value:1},sheenRoughnessMap:{value:null},sheenRoughnessMapTransform:{value:new zt},transmission:{value:0},transmissionMap:{value:null},transmissionMapTransform:{value:new zt},transmissionSamplerSize:{value:new Nt},transmissionSamplerMap:{value:null},thickness:{value:0},thicknessMap:{value:null},thicknessMapTransform:{value:new zt},attenuationDistance:{value:0},attenuationColor:{value:new Bt(0)},specularColor:{value:new Bt(1,1,1)},specularColorMap:{value:null},specularColorMapTransform:{value:new zt},specularIntensity:{value:1},specularIntensityMap:{value:null},specularIntensityMapTransform:{value:new zt},anisotropyVector:{value:new Nt},anisotropyMap:{value:null},anisotropyMapTransform:{value:new zt}}]),vertexShader:Xt.meshphysical_vert,fragmentShader:Xt.meshphysical_frag};const Cr={r:0,b:0,g:0},rg=new me,Qh=new zt;Qh.set(-1,0,0,0,1,0,0,0,1);function ag(n,t,e,i,s,r){const a=new Bt(0);let o=s===!0?0:1,c,l,u=null,g=0,h=null;function m(E){let A=E.isScene===!0?E.background:null;if(A&&A.isTexture){const T=E.backgroundBlurriness>0;A=t.get(A,T)}return A}function _(E){let A=!1;const T=m(E);T===null?f(a,o):T&&T.isColor&&(f(T,1),A=!0);const D=n.xr.getEnvironmentBlendMode();D==="additive"?e.buffers.color.setClear(0,0,0,1,r):D==="alpha-blend"&&e.buffers.color.setClear(0,0,0,0,r),(n.autoClear||A)&&(e.buffers.depth.setTest(!0),e.buffers.depth.setMask(!0),e.buffers.color.setMask(!0),n.clear(n.autoClearColor,n.autoClearDepth,n.autoClearStencil))}function y(E,A){const T=m(A);T&&(T.isCubeTexture||T.mapping===ga)?(l===void 0&&(l=new Xn(new Ks(1,1,1),new gn({name:"BackgroundCubeMaterial",uniforms:_s(En.backgroundCube.uniforms),vertexShader:En.backgroundCube.vertexShader,fragmentShader:En.backgroundCube.fragmentShader,side:Ke,depthTest:!1,depthWrite:!1,fog:!1,allowOverride:!1})),l.geometry.deleteAttribute("normal"),l.geometry.deleteAttribute("uv"),l.onBeforeRender=function(D,b,N){this.matrixWorld.copyPosition(N.matrixWorld)},Object.defineProperty(l.material,"envMap",{get:function(){return this.uniforms.envMap.value}}),i.update(l)),l.material.uniforms.envMap.value=T,l.material.uniforms.backgroundBlurriness.value=A.backgroundBlurriness,l.material.uniforms.backgroundIntensity.value=A.backgroundIntensity,l.material.uniforms.backgroundRotation.value.setFromMatrix4(rg.makeRotationFromEuler(A.backgroundRotation)).transpose(),T.isCubeTexture&&T.isRenderTargetTexture===!1&&l.material.uniforms.backgroundRotation.value.premultiply(Qh),l.material.toneMapped=jt.getTransfer(T.colorSpace)!==ie,(u!==T||g!==T.version||h!==n.toneMapping)&&(l.material.needsUpdate=!0,u=T,g=T.version,h=n.toneMapping),l.layers.enableAll(),E.unshift(l,l.geometry,l.material,0,0,null)):T&&T.isTexture&&(c===void 0&&(c=new Xn(new xa(2,2),new gn({name:"BackgroundMaterial",uniforms:_s(En.background.uniforms),vertexShader:En.background.vertexShader,fragmentShader:En.background.fragmentShader,side:ci,depthTest:!1,depthWrite:!1,fog:!1,allowOverride:!1})),c.geometry.deleteAttribute("normal"),Object.defineProperty(c.material,"map",{get:function(){return this.uniforms.t2D.value}}),i.update(c)),c.material.uniforms.t2D.value=T,c.material.uniforms.backgroundIntensity.value=A.backgroundIntensity,c.material.toneMapped=jt.getTransfer(T.colorSpace)!==ie,T.matrixAutoUpdate===!0&&T.updateMatrix(),c.material.uniforms.uvTransform.value.copy(T.matrix),(u!==T||g!==T.version||h!==n.toneMapping)&&(c.material.needsUpdate=!0,u=T,g=T.version,h=n.toneMapping),c.layers.enableAll(),E.unshift(c,c.geometry,c.material,0,0,null))}function f(E,A){E.getRGB(Cr,Yh(n)),e.buffers.color.setClear(Cr.r,Cr.g,Cr.b,A,r)}function d(){l!==void 0&&(l.geometry.dispose(),l.material.dispose(),l=void 0),c!==void 0&&(c.geometry.dispose(),c.material.dispose(),c=void 0)}return{getClearColor:function(){return a},setClearColor:function(E,A=1){a.set(E),o=A,f(a,o)},getClearAlpha:function(){return o},setClearAlpha:function(E){o=E,f(a,o)},render:_,addToRenderList:y,dispose:d}}function og(n,t){const e=n.getParameter(n.MAX_VERTEX_ATTRIBS),i={},s=h(null);let r=s,a=!1;function o(P,V,J,K,B){let W=!1;const $=g(P,K,J,V);r!==$&&(r=$,l(r.object)),W=m(P,K,J,B),W&&_(P,K,J,B),B!==null&&t.update(B,n.ELEMENT_ARRAY_BUFFER),(W||a)&&(a=!1,T(P,V,J,K),B!==null&&n.bindBuffer(n.ELEMENT_ARRAY_BUFFER,t.get(B).buffer))}function c(){return n.createVertexArray()}function l(P){return n.bindVertexArray(P)}function u(P){return n.deleteVertexArray(P)}function g(P,V,J,K){const B=K.wireframe===!0;let W=i[V.id];W===void 0&&(W={},i[V.id]=W);const $=P.isInstancedMesh===!0?P.id:0;let et=W[$];et===void 0&&(et={},W[$]=et);let at=et[J.id];at===void 0&&(at={},et[J.id]=at);let ct=at[B];return ct===void 0&&(ct=h(c()),at[B]=ct),ct}function h(P){const V=[],J=[],K=[];for(let B=0;B<e;B++)V[B]=0,J[B]=0,K[B]=0;return{geometry:null,program:null,wireframe:!1,newAttributes:V,enabledAttributes:J,attributeDivisors:K,object:P,attributes:{},index:null}}function m(P,V,J,K){const B=r.attributes,W=V.attributes;let $=0;const et=J.getAttributes();for(const at in et)if(et[at].location>=0){const yt=B[at];let wt=W[at];if(wt===void 0&&(at==="instanceMatrix"&&P.instanceMatrix&&(wt=P.instanceMatrix),at==="instanceColor"&&P.instanceColor&&(wt=P.instanceColor)),yt===void 0||yt.attribute!==wt||wt&&yt.data!==wt.data)return!0;$++}return r.attributesNum!==$||r.index!==K}function _(P,V,J,K){const B={},W=V.attributes;let $=0;const et=J.getAttributes();for(const at in et)if(et[at].location>=0){let yt=W[at];yt===void 0&&(at==="instanceMatrix"&&P.instanceMatrix&&(yt=P.instanceMatrix),at==="instanceColor"&&P.instanceColor&&(yt=P.instanceColor));const wt={};wt.attribute=yt,yt&&yt.data&&(wt.data=yt.data),B[at]=wt,$++}r.attributes=B,r.attributesNum=$,r.index=K}function y(){const P=r.newAttributes;for(let V=0,J=P.length;V<J;V++)P[V]=0}function f(P){d(P,0)}function d(P,V){const J=r.newAttributes,K=r.enabledAttributes,B=r.attributeDivisors;J[P]=1,K[P]===0&&(n.enableVertexAttribArray(P),K[P]=1),B[P]!==V&&(n.vertexAttribDivisor(P,V),B[P]=V)}function E(){const P=r.newAttributes,V=r.enabledAttributes;for(let J=0,K=V.length;J<K;J++)V[J]!==P[J]&&(n.disableVertexAttribArray(J),V[J]=0)}function A(P,V,J,K,B,W,$){$===!0?n.vertexAttribIPointer(P,V,J,B,W):n.vertexAttribPointer(P,V,J,K,B,W)}function T(P,V,J,K){y();const B=K.attributes,W=J.getAttributes(),$=V.defaultAttributeValues;for(const et in W){const at=W[et];if(at.location>=0){let ct=B[et];if(ct===void 0&&(et==="instanceMatrix"&&P.instanceMatrix&&(ct=P.instanceMatrix),et==="instanceColor"&&P.instanceColor&&(ct=P.instanceColor)),ct!==void 0){const yt=ct.normalized,wt=ct.itemSize,kt=t.get(ct);if(kt===void 0)continue;const Qt=kt.buffer,C=kt.type,U=kt.bytesPerElement,nt=C===n.INT||C===n.UNSIGNED_INT||ct.gpuType===_l;if(ct.isInterleavedBufferAttribute){const st=ct.data,Mt=st.stride,bt=ct.offset;if(st.isInstancedInterleavedBuffer){for(let Rt=0;Rt<at.locationSize;Rt++)d(at.location+Rt,st.meshPerAttribute);P.isInstancedMesh!==!0&&K._maxInstanceCount===void 0&&(K._maxInstanceCount=st.meshPerAttribute*st.count)}else for(let Rt=0;Rt<at.locationSize;Rt++)f(at.location+Rt);n.bindBuffer(n.ARRAY_BUFFER,Qt);for(let Rt=0;Rt<at.locationSize;Rt++)A(at.location+Rt,wt/at.locationSize,C,yt,Mt*U,(bt+wt/at.locationSize*Rt)*U,nt)}else{if(ct.isInstancedBufferAttribute){for(let st=0;st<at.locationSize;st++)d(at.location+st,ct.meshPerAttribute);P.isInstancedMesh!==!0&&K._maxInstanceCount===void 0&&(K._maxInstanceCount=ct.meshPerAttribute*ct.count)}else for(let st=0;st<at.locationSize;st++)f(at.location+st);n.bindBuffer(n.ARRAY_BUFFER,Qt);for(let st=0;st<at.locationSize;st++)A(at.location+st,wt/at.locationSize,C,yt,wt*U,wt/at.locationSize*st*U,nt)}}else if($!==void 0){const yt=$[et];if(yt!==void 0)switch(yt.length){case 2:n.vertexAttrib2fv(at.location,yt);break;case 3:n.vertexAttrib3fv(at.location,yt);break;case 4:n.vertexAttrib4fv(at.location,yt);break;default:n.vertexAttrib1fv(at.location,yt)}}}}E()}function D(){w();for(const P in i){const V=i[P];for(const J in V){const K=V[J];for(const B in K){const W=K[B];for(const $ in W)u(W[$].object),delete W[$];delete K[B]}}delete i[P]}}function b(P){if(i[P.id]===void 0)return;const V=i[P.id];for(const J in V){const K=V[J];for(const B in K){const W=K[B];for(const $ in W)u(W[$].object),delete W[$];delete K[B]}}delete i[P.id]}function N(P){for(const V in i){const J=i[V];for(const K in J){const B=J[K];if(B[P.id]===void 0)continue;const W=B[P.id];for(const $ in W)u(W[$].object),delete W[$];delete B[P.id]}}}function M(P){for(const V in i){const J=i[V],K=P.isInstancedMesh===!0?P.id:0,B=J[K];if(B!==void 0){for(const W in B){const $=B[W];for(const et in $)u($[et].object),delete $[et];delete B[W]}delete J[K],Object.keys(J).length===0&&delete i[V]}}}function w(){z(),a=!0,r!==s&&(r=s,l(r.object))}function z(){s.geometry=null,s.program=null,s.wireframe=!1}return{setup:o,reset:w,resetDefaultState:z,dispose:D,releaseStatesOfGeometry:b,releaseStatesOfObject:M,releaseStatesOfProgram:N,initAttributes:y,enableAttribute:f,disableUnusedAttributes:E}}function lg(n,t,e){let i;function s(c){i=c}function r(c,l){n.drawArrays(i,c,l),e.update(l,i,1)}function a(c,l,u){u!==0&&(n.drawArraysInstanced(i,c,l,u),e.update(l,i,u))}function o(c,l,u){if(u===0)return;t.get("WEBGL_multi_draw").multiDrawArraysWEBGL(i,c,0,l,0,u);let h=0;for(let m=0;m<u;m++)h+=l[m];e.update(h,i,1)}this.setMode=s,this.render=r,this.renderInstances=a,this.renderMultiDraw=o}function cg(n,t,e,i){let s;function r(){if(s!==void 0)return s;if(t.has("EXT_texture_filter_anisotropic")===!0){const N=t.get("EXT_texture_filter_anisotropic");s=n.getParameter(N.MAX_TEXTURE_MAX_ANISOTROPY_EXT)}else s=0;return s}function a(N){return!(N!==pn&&i.convert(N)!==n.getParameter(n.IMPLEMENTATION_COLOR_READ_FORMAT))}function o(N){const M=N===Vn&&(t.has("EXT_color_buffer_half_float")||t.has("EXT_color_buffer_float"));return!(N!==an&&i.convert(N)!==n.getParameter(n.IMPLEMENTATION_COLOR_READ_TYPE)&&N!==An&&!M)}function c(N){if(N==="highp"){if(n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.HIGH_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.HIGH_FLOAT).precision>0)return"highp";N="mediump"}return N==="mediump"&&n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.MEDIUM_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.MEDIUM_FLOAT).precision>0?"mediump":"lowp"}let l=e.precision!==void 0?e.precision:"highp";const u=c(l);u!==l&&(It("WebGLRenderer:",l,"not supported, using",u,"instead."),l=u);const g=e.logarithmicDepthBuffer===!0,h=e.reversedDepthBuffer===!0&&t.has("EXT_clip_control");e.reversedDepthBuffer===!0&&h===!1&&It("WebGLRenderer: Unable to use reversed depth buffer due to missing EXT_clip_control extension. Fallback to default depth buffer.");const m=n.getParameter(n.MAX_TEXTURE_IMAGE_UNITS),_=n.getParameter(n.MAX_VERTEX_TEXTURE_IMAGE_UNITS),y=n.getParameter(n.MAX_TEXTURE_SIZE),f=n.getParameter(n.MAX_CUBE_MAP_TEXTURE_SIZE),d=n.getParameter(n.MAX_VERTEX_ATTRIBS),E=n.getParameter(n.MAX_VERTEX_UNIFORM_VECTORS),A=n.getParameter(n.MAX_VARYING_VECTORS),T=n.getParameter(n.MAX_FRAGMENT_UNIFORM_VECTORS),D=n.getParameter(n.MAX_SAMPLES),b=n.getParameter(n.SAMPLES);return{isWebGL2:!0,getMaxAnisotropy:r,getMaxPrecision:c,textureFormatReadable:a,textureTypeReadable:o,precision:l,logarithmicDepthBuffer:g,reversedDepthBuffer:h,maxTextures:m,maxVertexTextures:_,maxTextureSize:y,maxCubemapSize:f,maxAttributes:d,maxVertexUniforms:E,maxVaryings:A,maxFragmentUniforms:T,maxSamples:D,samples:b}}function hg(n){const t=this;let e=null,i=0,s=!1,r=!1;const a=new ii,o=new zt,c={value:null,needsUpdate:!1};this.uniform=c,this.numPlanes=0,this.numIntersection=0,this.init=function(g,h){const m=g.length!==0||h||i!==0||s;return s=h,i=g.length,m},this.beginShadows=function(){r=!0,u(null)},this.endShadows=function(){r=!1},this.setGlobalState=function(g,h){e=u(g,h,0)},this.setState=function(g,h,m){const _=g.clippingPlanes,y=g.clipIntersection,f=g.clipShadows,d=n.get(g);if(!s||_===null||_.length===0||r&&!f)r?u(null):l();else{const E=r?0:i,A=E*4;let T=d.clippingState||null;c.value=T,T=u(_,h,A,m);for(let D=0;D!==A;++D)T[D]=e[D];d.clippingState=T,this.numIntersection=y?this.numPlanes:0,this.numPlanes+=E}};function l(){c.value!==e&&(c.value=e,c.needsUpdate=i>0),t.numPlanes=i,t.numIntersection=0}function u(g,h,m,_){const y=g!==null?g.length:0;let f=null;if(y!==0){if(f=c.value,_!==!0||f===null){const d=m+y*4,E=h.matrixWorldInverse;o.getNormalMatrix(E),(f===null||f.length<d)&&(f=new Float32Array(d));for(let A=0,T=m;A!==y;++A,T+=4)a.copy(g[A]).applyMatrix4(E,o),a.normal.toArray(f,T),f[T+3]=a.constant}c.value=f,c.needsUpdate=!0}return t.numPlanes=y,t.numIntersection=0,f}}const ri=4,Nc=[.125,.215,.35,.446,.526,.582],yi=20,ug=256,Rs=new Kh,Fc=new Bt;let Qa=null,to=0,eo=0,no=!1;const dg=new F;class Oc{constructor(t){this._renderer=t,this._pingPongRenderTarget=null,this._lodMax=0,this._cubeSize=0,this._sizeLods=[],this._sigmas=[],this._lodMeshes=[],this._backgroundBox=null,this._cubemapMaterial=null,this._equirectMaterial=null,this._blurMaterial=null,this._ggxMaterial=null}fromScene(t,e=0,i=.1,s=100,r={}){const{size:a=256,position:o=dg}=r;Qa=this._renderer.getRenderTarget(),to=this._renderer.getActiveCubeFace(),eo=this._renderer.getActiveMipmapLevel(),no=this._renderer.xr.enabled,this._renderer.xr.enabled=!1,this._setSize(a);const c=this._allocateTargets();return c.depthBuffer=!0,this._sceneToCubeUV(t,i,s,c,o),e>0&&this._blur(c,0,0,e),this._applyPMREM(c),this._cleanup(c),c}fromEquirectangular(t,e=null){return this._fromTexture(t,e)}fromCubemap(t,e=null){return this._fromTexture(t,e)}compileCubemapShader(){this._cubemapMaterial===null&&(this._cubemapMaterial=kc(),this._compileMaterial(this._cubemapMaterial))}compileEquirectangularShader(){this._equirectMaterial===null&&(this._equirectMaterial=zc(),this._compileMaterial(this._equirectMaterial))}dispose(){this._dispose(),this._cubemapMaterial!==null&&this._cubemapMaterial.dispose(),this._equirectMaterial!==null&&this._equirectMaterial.dispose(),this._backgroundBox!==null&&(this._backgroundBox.geometry.dispose(),this._backgroundBox.material.dispose())}_setSize(t){this._lodMax=Math.floor(Math.log2(t)),this._cubeSize=Math.pow(2,this._lodMax)}_dispose(){this._blurMaterial!==null&&this._blurMaterial.dispose(),this._ggxMaterial!==null&&this._ggxMaterial.dispose(),this._pingPongRenderTarget!==null&&this._pingPongRenderTarget.dispose();for(let t=0;t<this._lodMeshes.length;t++)this._lodMeshes[t].geometry.dispose()}_cleanup(t){this._renderer.setRenderTarget(Qa,to,eo),this._renderer.xr.enabled=no,t.scissorTest=!1,is(t,0,0,t.width,t.height)}_fromTexture(t,e){t.mapping===Pi||t.mapping===ms?this._setSize(t.image.length===0?16:t.image[0].width||t.image[0].image.width):this._setSize(t.image.width/4),Qa=this._renderer.getRenderTarget(),to=this._renderer.getActiveCubeFace(),eo=this._renderer.getActiveMipmapLevel(),no=this._renderer.xr.enabled,this._renderer.xr.enabled=!1;const i=e||this._allocateTargets();return this._textureToCubeUV(t,i),this._applyPMREM(i),this._cleanup(i),i}_allocateTargets(){const t=3*Math.max(this._cubeSize,112),e=4*this._cubeSize,i={magFilter:Be,minFilter:Be,generateMipmaps:!1,type:Vn,format:pn,colorSpace:ia,depthBuffer:!1},s=Bc(t,e,i);if(this._pingPongRenderTarget===null||this._pingPongRenderTarget.width!==t||this._pingPongRenderTarget.height!==e){this._pingPongRenderTarget!==null&&this._dispose(),this._pingPongRenderTarget=Bc(t,e,i);const{_lodMax:r}=this;({lodMeshes:this._lodMeshes,sizeLods:this._sizeLods,sigmas:this._sigmas}=fg(r)),this._blurMaterial=mg(r,t,e),this._ggxMaterial=pg(r,t,e)}return s}_compileMaterial(t){const e=new Xn(new Ue,t);this._renderer.compile(e,Rs)}_sceneToCubeUV(t,e,i,s,r){const c=new rn(90,1,e,i),l=[1,-1,1,1,1,1],u=[1,1,1,-1,-1,-1],g=this._renderer,h=g.autoClear,m=g.toneMapping;g.getClearColor(Fc),g.toneMapping=Cn,g.autoClear=!1,g.state.buffers.depth.getReversed()&&(g.setRenderTarget(s),g.clearDepth(),g.setRenderTarget(null)),this._backgroundBox===null&&(this._backgroundBox=new Xn(new Ks,new Gh({name:"PMREM.Background",side:Ke,depthWrite:!1,depthTest:!1})));const y=this._backgroundBox,f=y.material;let d=!1;const E=t.background;E?E.isColor&&(f.color.copy(E),t.background=null,d=!0):(f.color.copy(Fc),d=!0);for(let A=0;A<6;A++){const T=A%3;T===0?(c.up.set(0,l[A],0),c.position.set(r.x,r.y,r.z),c.lookAt(r.x+u[A],r.y,r.z)):T===1?(c.up.set(0,0,l[A]),c.position.set(r.x,r.y,r.z),c.lookAt(r.x,r.y+u[A],r.z)):(c.up.set(0,l[A],0),c.position.set(r.x,r.y,r.z),c.lookAt(r.x,r.y,r.z+u[A]));const D=this._cubeSize;is(s,T*D,A>2?D:0,D,D),g.setRenderTarget(s),d&&g.render(y,c),g.render(t,c)}g.toneMapping=m,g.autoClear=h,t.background=E}_textureToCubeUV(t,e){const i=this._renderer,s=t.mapping===Pi||t.mapping===ms;s?(this._cubemapMaterial===null&&(this._cubemapMaterial=kc()),this._cubemapMaterial.uniforms.flipEnvMap.value=t.isRenderTargetTexture===!1?-1:1):this._equirectMaterial===null&&(this._equirectMaterial=zc());const r=s?this._cubemapMaterial:this._equirectMaterial,a=this._lodMeshes[0];a.material=r;const o=r.uniforms;o.envMap.value=t;const c=this._cubeSize;is(e,0,0,3*c,2*c),i.setRenderTarget(e),i.render(a,Rs)}_applyPMREM(t){const e=this._renderer,i=e.autoClear;e.autoClear=!1;const s=this._lodMeshes.length;for(let r=1;r<s;r++)this._applyGGXFilter(t,r-1,r);e.autoClear=i}_applyGGXFilter(t,e,i){const s=this._renderer,r=this._pingPongRenderTarget,a=this._ggxMaterial,o=this._lodMeshes[i];o.material=a;const c=a.uniforms,l=i/(this._lodMeshes.length-1),u=e/(this._lodMeshes.length-1),g=Math.sqrt(l*l-u*u),h=0+l*1.25,m=g*h,{_lodMax:_}=this,y=this._sizeLods[i],f=3*y*(i>_-ri?i-_+ri:0),d=4*(this._cubeSize-y);c.envMap.value=t.texture,c.roughness.value=m,c.mipInt.value=_-e,is(r,f,d,3*y,2*y),s.setRenderTarget(r),s.render(o,Rs),c.envMap.value=r.texture,c.roughness.value=0,c.mipInt.value=_-i,is(t,f,d,3*y,2*y),s.setRenderTarget(t),s.render(o,Rs)}_blur(t,e,i,s,r){const a=this._pingPongRenderTarget;this._halfBlur(t,a,e,i,s,"latitudinal",r),this._halfBlur(a,t,i,i,s,"longitudinal",r)}_halfBlur(t,e,i,s,r,a,o){const c=this._renderer,l=this._blurMaterial;a!=="latitudinal"&&a!=="longitudinal"&&Kt("blur direction must be either latitudinal or longitudinal!");const u=3,g=this._lodMeshes[s];g.material=l;const h=l.uniforms,m=this._sizeLods[i]-1,_=isFinite(r)?Math.PI/(2*m):2*Math.PI/(2*yi-1),y=r/_,f=isFinite(r)?1+Math.floor(u*y):yi;f>yi&&It(`sigmaRadians, ${r}, is too large and will clip, as it requested ${f} samples when the maximum is set to ${yi}`);const d=[];let E=0;for(let N=0;N<yi;++N){const M=N/y,w=Math.exp(-M*M/2);d.push(w),N===0?E+=w:N<f&&(E+=2*w)}for(let N=0;N<d.length;N++)d[N]=d[N]/E;h.envMap.value=t.texture,h.samples.value=f,h.weights.value=d,h.latitudinal.value=a==="latitudinal",o&&(h.poleAxis.value=o);const{_lodMax:A}=this;h.dTheta.value=_,h.mipInt.value=A-i;const T=this._sizeLods[s],D=3*T*(s>A-ri?s-A+ri:0),b=4*(this._cubeSize-T);is(e,D,b,3*T,2*T),c.setRenderTarget(e),c.render(g,Rs)}}function fg(n){const t=[],e=[],i=[];let s=n;const r=n-ri+1+Nc.length;for(let a=0;a<r;a++){const o=Math.pow(2,s);t.push(o);let c=1/o;a>n-ri?c=Nc[a-n+ri-1]:a===0&&(c=0),e.push(c);const l=1/(o-2),u=-l,g=1+l,h=[u,u,g,u,g,g,u,u,g,g,u,g],m=6,_=6,y=3,f=2,d=1,E=new Float32Array(y*_*m),A=new Float32Array(f*_*m),T=new Float32Array(d*_*m);for(let b=0;b<m;b++){const N=b%3*2/3-1,M=b>2?0:-1,w=[N,M,0,N+2/3,M,0,N+2/3,M+1,0,N,M,0,N+2/3,M+1,0,N,M+1,0];E.set(w,y*_*b),A.set(h,f*_*b);const z=[b,b,b,b,b,b];T.set(z,d*_*b)}const D=new Ue;D.setAttribute("position",new Ae(E,y)),D.setAttribute("uv",new Ae(A,f)),D.setAttribute("faceIndex",new Ae(T,d)),i.push(new Xn(D,null)),s>ri&&s--}return{lodMeshes:i,sizeLods:t,sigmas:e}}function Bc(n,t,e){const i=new Pn(n,t,e);return i.texture.mapping=ga,i.texture.name="PMREM.cubeUv",i.scissorTest=!0,i}function is(n,t,e,i,s){n.viewport.set(t,e,i,s),n.scissor.set(t,e,i,s)}function pg(n,t,e){return new gn({name:"PMREMGGXConvolution",defines:{GGX_SAMPLES:ug,CUBEUV_TEXEL_WIDTH:1/t,CUBEUV_TEXEL_HEIGHT:1/e,CUBEUV_MAX_MIP:`${n}.0`},uniforms:{envMap:{value:null},roughness:{value:0},mipInt:{value:0}},vertexShader:va(),fragmentShader:`

			precision highp float;
			precision highp int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;
			uniform float roughness;
			uniform float mipInt;

			#define ENVMAP_TYPE_CUBE_UV
			#include <cube_uv_reflection_fragment>

			#define PI 3.14159265359

			// Van der Corput radical inverse
			float radicalInverse_VdC(uint bits) {
				bits = (bits << 16u) | (bits >> 16u);
				bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
				bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
				bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
				bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
				return float(bits) * 2.3283064365386963e-10; // / 0x100000000
			}

			// Hammersley sequence
			vec2 hammersley(uint i, uint N) {
				return vec2(float(i) / float(N), radicalInverse_VdC(i));
			}

			// GGX VNDF importance sampling (Eric Heitz 2018)
			// "Sampling the GGX Distribution of Visible Normals"
			// https://jcgt.org/published/0007/04/01/
			vec3 importanceSampleGGX_VNDF(vec2 Xi, vec3 V, float roughness) {
				float alpha = roughness * roughness;

				// Section 4.1: Orthonormal basis
				vec3 T1 = vec3(1.0, 0.0, 0.0);
				vec3 T2 = cross(V, T1);

				// Section 4.2: Parameterization of projected area
				float r = sqrt(Xi.x);
				float phi = 2.0 * PI * Xi.y;
				float t1 = r * cos(phi);
				float t2 = r * sin(phi);
				float s = 0.5 * (1.0 + V.z);
				t2 = (1.0 - s) * sqrt(1.0 - t1 * t1) + s * t2;

				// Section 4.3: Reprojection onto hemisphere
				vec3 Nh = t1 * T1 + t2 * T2 + sqrt(max(0.0, 1.0 - t1 * t1 - t2 * t2)) * V;

				// Section 3.4: Transform back to ellipsoid configuration
				return normalize(vec3(alpha * Nh.x, alpha * Nh.y, max(0.0, Nh.z)));
			}

			void main() {
				vec3 N = normalize(vOutputDirection);
				vec3 V = N; // Assume view direction equals normal for pre-filtering

				vec3 prefilteredColor = vec3(0.0);
				float totalWeight = 0.0;

				// For very low roughness, just sample the environment directly
				if (roughness < 0.001) {
					gl_FragColor = vec4(bilinearCubeUV(envMap, N, mipInt), 1.0);
					return;
				}

				// Tangent space basis for VNDF sampling
				vec3 up = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);
				vec3 tangent = normalize(cross(up, N));
				vec3 bitangent = cross(N, tangent);

				for(uint i = 0u; i < uint(GGX_SAMPLES); i++) {
					vec2 Xi = hammersley(i, uint(GGX_SAMPLES));

					// For PMREM, V = N, so in tangent space V is always (0, 0, 1)
					vec3 H_tangent = importanceSampleGGX_VNDF(Xi, vec3(0.0, 0.0, 1.0), roughness);

					// Transform H back to world space
					vec3 H = normalize(tangent * H_tangent.x + bitangent * H_tangent.y + N * H_tangent.z);
					vec3 L = normalize(2.0 * dot(V, H) * H - V);

					float NdotL = max(dot(N, L), 0.0);

					if(NdotL > 0.0) {
						// Sample environment at fixed mip level
						// VNDF importance sampling handles the distribution filtering
						vec3 sampleColor = bilinearCubeUV(envMap, L, mipInt);

						// Weight by NdotL for the split-sum approximation
						// VNDF PDF naturally accounts for the visible microfacet distribution
						prefilteredColor += sampleColor * NdotL;
						totalWeight += NdotL;
					}
				}

				if (totalWeight > 0.0) {
					prefilteredColor = prefilteredColor / totalWeight;
				}

				gl_FragColor = vec4(prefilteredColor, 1.0);
			}
		`,blending:Hn,depthTest:!1,depthWrite:!1})}function mg(n,t,e){const i=new Float32Array(yi),s=new F(0,1,0);return new gn({name:"SphericalGaussianBlur",defines:{n:yi,CUBEUV_TEXEL_WIDTH:1/t,CUBEUV_TEXEL_HEIGHT:1/e,CUBEUV_MAX_MIP:`${n}.0`},uniforms:{envMap:{value:null},samples:{value:1},weights:{value:i},latitudinal:{value:!1},dTheta:{value:0},mipInt:{value:0},poleAxis:{value:s}},vertexShader:va(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;
			uniform int samples;
			uniform float weights[ n ];
			uniform bool latitudinal;
			uniform float dTheta;
			uniform float mipInt;
			uniform vec3 poleAxis;

			#define ENVMAP_TYPE_CUBE_UV
			#include <cube_uv_reflection_fragment>

			vec3 getSample( float theta, vec3 axis ) {

				float cosTheta = cos( theta );
				// Rodrigues' axis-angle rotation
				vec3 sampleDirection = vOutputDirection * cosTheta
					+ cross( axis, vOutputDirection ) * sin( theta )
					+ axis * dot( axis, vOutputDirection ) * ( 1.0 - cosTheta );

				return bilinearCubeUV( envMap, sampleDirection, mipInt );

			}

			void main() {

				vec3 axis = latitudinal ? poleAxis : cross( poleAxis, vOutputDirection );

				if ( all( equal( axis, vec3( 0.0 ) ) ) ) {

					axis = vec3( vOutputDirection.z, 0.0, - vOutputDirection.x );

				}

				axis = normalize( axis );

				gl_FragColor = vec4( 0.0, 0.0, 0.0, 1.0 );
				gl_FragColor.rgb += weights[ 0 ] * getSample( 0.0, axis );

				for ( int i = 1; i < n; i++ ) {

					if ( i >= samples ) {

						break;

					}

					float theta = dTheta * float( i );
					gl_FragColor.rgb += weights[ i ] * getSample( -1.0 * theta, axis );
					gl_FragColor.rgb += weights[ i ] * getSample( theta, axis );

				}

			}
		`,blending:Hn,depthTest:!1,depthWrite:!1})}function zc(){return new gn({name:"EquirectangularToCubeUV",uniforms:{envMap:{value:null}},vertexShader:va(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;

			#include <common>

			void main() {

				vec3 outputDirection = normalize( vOutputDirection );
				vec2 uv = equirectUv( outputDirection );

				gl_FragColor = vec4( texture2D ( envMap, uv ).rgb, 1.0 );

			}
		`,blending:Hn,depthTest:!1,depthWrite:!1})}function kc(){return new gn({name:"CubemapToCubeUV",uniforms:{envMap:{value:null},flipEnvMap:{value:-1}},vertexShader:va(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			uniform float flipEnvMap;

			varying vec3 vOutputDirection;

			uniform samplerCube envMap;

			void main() {

				gl_FragColor = textureCube( envMap, vec3( flipEnvMap * vOutputDirection.x, vOutputDirection.yz ) );

			}
		`,blending:Hn,depthTest:!1,depthWrite:!1})}function va(){return`

		precision mediump float;
		precision mediump int;

		attribute float faceIndex;

		varying vec3 vOutputDirection;

		// RH coordinate system; PMREM face-indexing convention
		vec3 getDirection( vec2 uv, float face ) {

			uv = 2.0 * uv - 1.0;

			vec3 direction = vec3( uv, 1.0 );

			if ( face == 0.0 ) {

				direction = direction.zyx; // ( 1, v, u ) pos x

			} else if ( face == 1.0 ) {

				direction = direction.xzy;
				direction.xz *= -1.0; // ( -u, 1, -v ) pos y

			} else if ( face == 2.0 ) {

				direction.x *= -1.0; // ( -u, v, 1 ) pos z

			} else if ( face == 3.0 ) {

				direction = direction.zyx;
				direction.xz *= -1.0; // ( -1, v, -u ) neg x

			} else if ( face == 4.0 ) {

				direction = direction.xzy;
				direction.xy *= -1.0; // ( -u, -1, v ) neg y

			} else if ( face == 5.0 ) {

				direction.z *= -1.0; // ( u, v, -1 ) neg z

			}

			return direction;

		}

		void main() {

			vOutputDirection = getDirection( uv, faceIndex );
			gl_Position = vec4( position, 1.0 );

		}
	`}class tu extends Pn{constructor(t=1,e={}){super(t,t,e),this.isWebGLCubeRenderTarget=!0;const i={width:t,height:t,depth:1},s=[i,i,i,i,i,i];this.texture=new Xh(s),this._setTextureOptions(e),this.texture.isRenderTargetTexture=!0}fromEquirectangularTexture(t,e){this.texture.type=e.type,this.texture.colorSpace=e.colorSpace,this.texture.generateMipmaps=e.generateMipmaps,this.texture.minFilter=e.minFilter,this.texture.magFilter=e.magFilter;const i={uniforms:{tEquirect:{value:null}},vertexShader:`

				varying vec3 vWorldDirection;

				vec3 transformDirection( in vec3 dir, in mat4 matrix ) {

					return normalize( ( matrix * vec4( dir, 0.0 ) ).xyz );

				}

				void main() {

					vWorldDirection = transformDirection( position, modelMatrix );

					#include <begin_vertex>
					#include <project_vertex>

				}
			`,fragmentShader:`

				uniform sampler2D tEquirect;

				varying vec3 vWorldDirection;

				#include <common>

				void main() {

					vec3 direction = normalize( vWorldDirection );

					vec2 sampleUV = equirectUv( direction );

					gl_FragColor = texture2D( tEquirect, sampleUV );

				}
			`},s=new Ks(5,5,5),r=new gn({name:"CubemapFromEquirect",uniforms:_s(i.uniforms),vertexShader:i.vertexShader,fragmentShader:i.fragmentShader,side:Ke,blending:Hn});r.uniforms.tEquirect.value=e;const a=new Xn(s,r),o=e.minFilter;return e.minFilter===Ti&&(e.minFilter=Be),new Mf(1,10,this).update(t,a),e.minFilter=o,a.geometry.dispose(),a.material.dispose(),this}clear(t,e=!0,i=!0,s=!0){const r=t.getRenderTarget();for(let a=0;a<6;a++)t.setRenderTarget(this,a),t.clear(e,i,s);t.setRenderTarget(r)}}function gg(n){let t=new WeakMap,e=new WeakMap,i=null;function s(h,m=!1){return h==null?null:m?a(h):r(h)}function r(h){if(h&&h.isTexture){const m=h.mapping;if(m===Ta||m===Aa)if(t.has(h)){const _=t.get(h).texture;return o(_,h.mapping)}else{const _=h.image;if(_&&_.height>0){const y=new tu(_.height);return y.fromEquirectangularTexture(n,h),t.set(h,y),h.addEventListener("dispose",l),o(y.texture,h.mapping)}else return null}}return h}function a(h){if(h&&h.isTexture){const m=h.mapping,_=m===Ta||m===Aa,y=m===Pi||m===ms;if(_||y){let f=e.get(h);const d=f!==void 0?f.texture.pmremVersion:0;if(h.isRenderTargetTexture&&h.pmremVersion!==d)return i===null&&(i=new Oc(n)),f=_?i.fromEquirectangular(h,f):i.fromCubemap(h,f),f.texture.pmremVersion=h.pmremVersion,e.set(h,f),f.texture;if(f!==void 0)return f.texture;{const E=h.image;return _&&E&&E.height>0||y&&E&&c(E)?(i===null&&(i=new Oc(n)),f=_?i.fromEquirectangular(h):i.fromCubemap(h),f.texture.pmremVersion=h.pmremVersion,e.set(h,f),h.addEventListener("dispose",u),f.texture):null}}}return h}function o(h,m){return m===Ta?h.mapping=Pi:m===Aa&&(h.mapping=ms),h}function c(h){let m=0;const _=6;for(let y=0;y<_;y++)h[y]!==void 0&&m++;return m===_}function l(h){const m=h.target;m.removeEventListener("dispose",l);const _=t.get(m);_!==void 0&&(t.delete(m),_.dispose())}function u(h){const m=h.target;m.removeEventListener("dispose",u);const _=e.get(m);_!==void 0&&(e.delete(m),_.dispose())}function g(){t=new WeakMap,e=new WeakMap,i!==null&&(i.dispose(),i=null)}return{get:s,dispose:g}}function _g(n){const t={};function e(i){if(t[i]!==void 0)return t[i];const s=n.getExtension(i);return t[i]=s,s}return{has:function(i){return e(i)!==null},init:function(){e("EXT_color_buffer_float"),e("WEBGL_clip_cull_distance"),e("OES_texture_float_linear"),e("EXT_color_buffer_half_float"),e("WEBGL_multisampled_render_to_texture"),e("WEBGL_render_shared_exponent")},get:function(i){const s=e(i);return s===null&&Jo("WebGLRenderer: "+i+" extension not supported."),s}}}function xg(n,t,e,i){const s={},r=new WeakMap;function a(g){const h=g.target;h.index!==null&&t.remove(h.index);for(const _ in h.attributes)t.remove(h.attributes[_]);h.removeEventListener("dispose",a),delete s[h.id];const m=r.get(h);m&&(t.remove(m),r.delete(h)),i.releaseStatesOfGeometry(h),h.isInstancedBufferGeometry===!0&&delete h._maxInstanceCount,e.memory.geometries--}function o(g,h){return s[h.id]===!0||(h.addEventListener("dispose",a),s[h.id]=!0,e.memory.geometries++),h}function c(g){const h=g.attributes;for(const m in h)t.update(h[m],n.ARRAY_BUFFER)}function l(g){const h=[],m=g.index,_=g.attributes.position;let y=0;if(_===void 0)return;if(m!==null){const E=m.array;y=m.version;for(let A=0,T=E.length;A<T;A+=3){const D=E[A+0],b=E[A+1],N=E[A+2];h.push(D,b,b,N,N,D)}}else{const E=_.array;y=_.version;for(let A=0,T=E.length/3-1;A<T;A+=3){const D=A+0,b=A+1,N=A+2;h.push(D,b,b,N,N,D)}}const f=new(_.count>=65535?zh:Bh)(h,1);f.version=y;const d=r.get(g);d&&t.remove(d),r.set(g,f)}function u(g){const h=r.get(g);if(h){const m=g.index;m!==null&&h.version<m.version&&l(g)}else l(g);return r.get(g)}return{get:o,update:c,getWireframeAttribute:u}}function vg(n,t,e){let i;function s(g){i=g}let r,a;function o(g){r=g.type,a=g.bytesPerElement}function c(g,h){n.drawElements(i,h,r,g*a),e.update(h,i,1)}function l(g,h,m){m!==0&&(n.drawElementsInstanced(i,h,r,g*a,m),e.update(h,i,m))}function u(g,h,m){if(m===0)return;t.get("WEBGL_multi_draw").multiDrawElementsWEBGL(i,h,0,r,g,0,m);let y=0;for(let f=0;f<m;f++)y+=h[f];e.update(y,i,1)}this.setMode=s,this.setIndex=o,this.render=c,this.renderInstances=l,this.renderMultiDraw=u}function Mg(n){const t={geometries:0,textures:0},e={frame:0,calls:0,triangles:0,points:0,lines:0};function i(r,a,o){switch(e.calls++,a){case n.TRIANGLES:e.triangles+=o*(r/3);break;case n.LINES:e.lines+=o*(r/2);break;case n.LINE_STRIP:e.lines+=o*(r-1);break;case n.LINE_LOOP:e.lines+=o*r;break;case n.POINTS:e.points+=o*r;break;default:Kt("WebGLInfo: Unknown draw mode:",a);break}}function s(){e.calls=0,e.triangles=0,e.points=0,e.lines=0}return{memory:t,render:e,programs:null,autoReset:!0,reset:s,update:i}}function Sg(n,t,e){const i=new WeakMap,s=new ye;function r(a,o,c){const l=a.morphTargetInfluences,u=o.morphAttributes.position||o.morphAttributes.normal||o.morphAttributes.color,g=u!==void 0?u.length:0;let h=i.get(o);if(h===void 0||h.count!==g){let w=function(){N.dispose(),i.delete(o),o.removeEventListener("dispose",w)};h!==void 0&&h.texture.dispose();const m=o.morphAttributes.position!==void 0,_=o.morphAttributes.normal!==void 0,y=o.morphAttributes.color!==void 0,f=o.morphAttributes.position||[],d=o.morphAttributes.normal||[],E=o.morphAttributes.color||[];let A=0;m===!0&&(A=1),_===!0&&(A=2),y===!0&&(A=3);let T=o.attributes.position.count*A,D=1;T>t.maxTextureSize&&(D=Math.ceil(T/t.maxTextureSize),T=t.maxTextureSize);const b=new Float32Array(T*D*4*g),N=new Nh(b,T,D,g);N.type=An,N.needsUpdate=!0;const M=A*4;for(let z=0;z<g;z++){const P=f[z],V=d[z],J=E[z],K=T*D*4*z;for(let B=0;B<P.count;B++){const W=B*M;m===!0&&(s.fromBufferAttribute(P,B),b[K+W+0]=s.x,b[K+W+1]=s.y,b[K+W+2]=s.z,b[K+W+3]=0),_===!0&&(s.fromBufferAttribute(V,B),b[K+W+4]=s.x,b[K+W+5]=s.y,b[K+W+6]=s.z,b[K+W+7]=0),y===!0&&(s.fromBufferAttribute(J,B),b[K+W+8]=s.x,b[K+W+9]=s.y,b[K+W+10]=s.z,b[K+W+11]=J.itemSize===4?s.w:1)}}h={count:g,texture:N,size:new Nt(T,D)},i.set(o,h),o.addEventListener("dispose",w)}if(a.isInstancedMesh===!0&&a.morphTexture!==null)c.getUniforms().setValue(n,"morphTexture",a.morphTexture,e);else{let m=0;for(let y=0;y<l.length;y++)m+=l[y];const _=o.morphTargetsRelative?1:1-m;c.getUniforms().setValue(n,"morphTargetBaseInfluence",_),c.getUniforms().setValue(n,"morphTargetInfluences",l)}c.getUniforms().setValue(n,"morphTargetsTexture",h.texture,e),c.getUniforms().setValue(n,"morphTargetsTextureSize",h.size)}return{update:r}}function yg(n,t,e,i,s){let r=new WeakMap;function a(l){const u=s.render.frame,g=l.geometry,h=t.get(l,g);if(r.get(h)!==u&&(t.update(h),r.set(h,u)),l.isInstancedMesh&&(l.hasEventListener("dispose",c)===!1&&l.addEventListener("dispose",c),r.get(l)!==u&&(e.update(l.instanceMatrix,n.ARRAY_BUFFER),l.instanceColor!==null&&e.update(l.instanceColor,n.ARRAY_BUFFER),r.set(l,u))),l.isSkinnedMesh){const m=l.skeleton;r.get(m)!==u&&(m.update(),r.set(m,u))}return h}function o(){r=new WeakMap}function c(l){const u=l.target;u.removeEventListener("dispose",c),i.releaseStatesOfObject(u),e.remove(u.instanceMatrix),u.instanceColor!==null&&e.remove(u.instanceColor)}return{update:a,dispose:o}}const Eg={[vh]:"LINEAR_TONE_MAPPING",[Mh]:"REINHARD_TONE_MAPPING",[Sh]:"CINEON_TONE_MAPPING",[yh]:"ACES_FILMIC_TONE_MAPPING",[bh]:"AGX_TONE_MAPPING",[Th]:"NEUTRAL_TONE_MAPPING",[Eh]:"CUSTOM_TONE_MAPPING"};function bg(n,t,e,i,s){const r=new Pn(t,e,{type:n,depthBuffer:i,stencilBuffer:s,depthTexture:i?new gs(t,e):void 0}),a=new Pn(t,e,{type:Vn,depthBuffer:!1,stencilBuffer:!1}),o=new Ue;o.setAttribute("position",new mn([-1,3,0,-1,-1,0,3,-1,0],3)),o.setAttribute("uv",new mn([0,2,0,0,2,0],2));const c=new _f({uniforms:{tDiffuse:{value:null}},vertexShader:`
			precision highp float;

			uniform mat4 modelViewMatrix;
			uniform mat4 projectionMatrix;

			attribute vec3 position;
			attribute vec2 uv;

			varying vec2 vUv;

			void main() {
				vUv = uv;
				gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
			}`,fragmentShader:`
			precision highp float;

			uniform sampler2D tDiffuse;

			varying vec2 vUv;

			#include <tonemapping_pars_fragment>
			#include <colorspace_pars_fragment>

			void main() {
				gl_FragColor = texture2D( tDiffuse, vUv );

				#ifdef LINEAR_TONE_MAPPING
					gl_FragColor.rgb = LinearToneMapping( gl_FragColor.rgb );
				#elif defined( REINHARD_TONE_MAPPING )
					gl_FragColor.rgb = ReinhardToneMapping( gl_FragColor.rgb );
				#elif defined( CINEON_TONE_MAPPING )
					gl_FragColor.rgb = CineonToneMapping( gl_FragColor.rgb );
				#elif defined( ACES_FILMIC_TONE_MAPPING )
					gl_FragColor.rgb = ACESFilmicToneMapping( gl_FragColor.rgb );
				#elif defined( AGX_TONE_MAPPING )
					gl_FragColor.rgb = AgXToneMapping( gl_FragColor.rgb );
				#elif defined( NEUTRAL_TONE_MAPPING )
					gl_FragColor.rgb = NeutralToneMapping( gl_FragColor.rgb );
				#elif defined( CUSTOM_TONE_MAPPING )
					gl_FragColor.rgb = CustomToneMapping( gl_FragColor.rgb );
				#endif

				#ifdef SRGB_TRANSFER
					gl_FragColor = sRGBTransferOETF( gl_FragColor );
				#endif
			}`,depthTest:!1,depthWrite:!1}),l=new Xn(o,c),u=new Kh(-1,1,1,-1,0,1);let g=null,h=null,m=!1,_,y=null,f=[],d=!1;this.setSize=function(E,A){r.setSize(E,A),a.setSize(E,A);for(let T=0;T<f.length;T++){const D=f[T];D.setSize&&D.setSize(E,A)}},this.setEffects=function(E){f=E,d=f.length>0&&f[0].isRenderPass===!0;const A=r.width,T=r.height;for(let D=0;D<f.length;D++){const b=f[D];b.setSize&&b.setSize(A,T)}},this.begin=function(E,A){if(m||E.toneMapping===Cn&&f.length===0)return!1;if(y=A,A!==null){const T=A.width,D=A.height;(r.width!==T||r.height!==D)&&this.setSize(T,D)}return d===!1&&E.setRenderTarget(r),_=E.toneMapping,E.toneMapping=Cn,!0},this.hasRenderPass=function(){return d},this.end=function(E,A){E.toneMapping=_,m=!0;let T=r,D=a;for(let b=0;b<f.length;b++){const N=f[b];if(N.enabled!==!1&&(N.render(E,D,T,A),N.needsSwap!==!1)){const M=T;T=D,D=M}}if(g!==E.outputColorSpace||h!==E.toneMapping){g=E.outputColorSpace,h=E.toneMapping,c.defines={},jt.getTransfer(g)===ie&&(c.defines.SRGB_TRANSFER="");const b=Eg[h];b&&(c.defines[b]=""),c.needsUpdate=!0}c.uniforms.tDiffuse.value=T.texture,E.setRenderTarget(y),E.render(l,u),y=null,m=!1},this.isCompositing=function(){return m},this.dispose=function(){r.depthTexture&&r.depthTexture.dispose(),r.dispose(),a.dispose(),o.dispose(),c.dispose()}}const eu=new ze,el=new gs(1,1),nu=new Nh,iu=new Wd,su=new Xh,Hc=[],Gc=[],Vc=new Float32Array(16),Wc=new Float32Array(9),Xc=new Float32Array(4);function xs(n,t,e){const i=n[0];if(i<=0||i>0)return n;const s=t*e;let r=Hc[s];if(r===void 0&&(r=new Float32Array(s),Hc[s]=r),t!==0){i.toArray(r,0);for(let a=1,o=0;a!==t;++a)o+=e,n[a].toArray(r,o)}return r}function Re(n,t){if(n.length!==t.length)return!1;for(let e=0,i=n.length;e<i;e++)if(n[e]!==t[e])return!1;return!0}function Ce(n,t){for(let e=0,i=t.length;e<i;e++)n[e]=t[e]}function Ma(n,t){let e=Gc[t];e===void 0&&(e=new Int32Array(t),Gc[t]=e);for(let i=0;i!==t;++i)e[i]=n.allocateTextureUnit();return e}function Tg(n,t){const e=this.cache;e[0]!==t&&(n.uniform1f(this.addr,t),e[0]=t)}function Ag(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2f(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(Re(e,t))return;n.uniform2fv(this.addr,t),Ce(e,t)}}function wg(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3f(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else if(t.r!==void 0)(e[0]!==t.r||e[1]!==t.g||e[2]!==t.b)&&(n.uniform3f(this.addr,t.r,t.g,t.b),e[0]=t.r,e[1]=t.g,e[2]=t.b);else{if(Re(e,t))return;n.uniform3fv(this.addr,t),Ce(e,t)}}function Rg(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4f(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(Re(e,t))return;n.uniform4fv(this.addr,t),Ce(e,t)}}function Cg(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(Re(e,t))return;n.uniformMatrix2fv(this.addr,!1,t),Ce(e,t)}else{if(Re(e,i))return;Xc.set(i),n.uniformMatrix2fv(this.addr,!1,Xc),Ce(e,i)}}function Pg(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(Re(e,t))return;n.uniformMatrix3fv(this.addr,!1,t),Ce(e,t)}else{if(Re(e,i))return;Wc.set(i),n.uniformMatrix3fv(this.addr,!1,Wc),Ce(e,i)}}function Lg(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(Re(e,t))return;n.uniformMatrix4fv(this.addr,!1,t),Ce(e,t)}else{if(Re(e,i))return;Vc.set(i),n.uniformMatrix4fv(this.addr,!1,Vc),Ce(e,i)}}function Dg(n,t){const e=this.cache;e[0]!==t&&(n.uniform1i(this.addr,t),e[0]=t)}function Ig(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2i(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(Re(e,t))return;n.uniform2iv(this.addr,t),Ce(e,t)}}function Ug(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3i(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else{if(Re(e,t))return;n.uniform3iv(this.addr,t),Ce(e,t)}}function Ng(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4i(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(Re(e,t))return;n.uniform4iv(this.addr,t),Ce(e,t)}}function Fg(n,t){const e=this.cache;e[0]!==t&&(n.uniform1ui(this.addr,t),e[0]=t)}function Og(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2ui(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(Re(e,t))return;n.uniform2uiv(this.addr,t),Ce(e,t)}}function Bg(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3ui(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else{if(Re(e,t))return;n.uniform3uiv(this.addr,t),Ce(e,t)}}function zg(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4ui(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(Re(e,t))return;n.uniform4uiv(this.addr,t),Ce(e,t)}}function kg(n,t,e){const i=this.cache,s=e.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s);let r;this.type===n.SAMPLER_2D_SHADOW?(el.compareFunction=e.isReversedDepthBuffer()?bl:El,r=el):r=eu,e.setTexture2D(t||r,s)}function Hg(n,t,e){const i=this.cache,s=e.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s),e.setTexture3D(t||iu,s)}function Gg(n,t,e){const i=this.cache,s=e.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s),e.setTextureCube(t||su,s)}function Vg(n,t,e){const i=this.cache,s=e.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s),e.setTexture2DArray(t||nu,s)}function Wg(n){switch(n){case 5126:return Tg;case 35664:return Ag;case 35665:return wg;case 35666:return Rg;case 35674:return Cg;case 35675:return Pg;case 35676:return Lg;case 5124:case 35670:return Dg;case 35667:case 35671:return Ig;case 35668:case 35672:return Ug;case 35669:case 35673:return Ng;case 5125:return Fg;case 36294:return Og;case 36295:return Bg;case 36296:return zg;case 35678:case 36198:case 36298:case 36306:case 35682:return kg;case 35679:case 36299:case 36307:return Hg;case 35680:case 36300:case 36308:case 36293:return Gg;case 36289:case 36303:case 36311:case 36292:return Vg}}function Xg(n,t){n.uniform1fv(this.addr,t)}function $g(n,t){const e=xs(t,this.size,2);n.uniform2fv(this.addr,e)}function qg(n,t){const e=xs(t,this.size,3);n.uniform3fv(this.addr,e)}function Yg(n,t){const e=xs(t,this.size,4);n.uniform4fv(this.addr,e)}function jg(n,t){const e=xs(t,this.size,4);n.uniformMatrix2fv(this.addr,!1,e)}function Kg(n,t){const e=xs(t,this.size,9);n.uniformMatrix3fv(this.addr,!1,e)}function Zg(n,t){const e=xs(t,this.size,16);n.uniformMatrix4fv(this.addr,!1,e)}function Jg(n,t){n.uniform1iv(this.addr,t)}function Qg(n,t){n.uniform2iv(this.addr,t)}function t_(n,t){n.uniform3iv(this.addr,t)}function e_(n,t){n.uniform4iv(this.addr,t)}function n_(n,t){n.uniform1uiv(this.addr,t)}function i_(n,t){n.uniform2uiv(this.addr,t)}function s_(n,t){n.uniform3uiv(this.addr,t)}function r_(n,t){n.uniform4uiv(this.addr,t)}function a_(n,t,e){const i=this.cache,s=t.length,r=Ma(e,s);Re(i,r)||(n.uniform1iv(this.addr,r),Ce(i,r));let a;this.type===n.SAMPLER_2D_SHADOW?a=el:a=eu;for(let o=0;o!==s;++o)e.setTexture2D(t[o]||a,r[o])}function o_(n,t,e){const i=this.cache,s=t.length,r=Ma(e,s);Re(i,r)||(n.uniform1iv(this.addr,r),Ce(i,r));for(let a=0;a!==s;++a)e.setTexture3D(t[a]||iu,r[a])}function l_(n,t,e){const i=this.cache,s=t.length,r=Ma(e,s);Re(i,r)||(n.uniform1iv(this.addr,r),Ce(i,r));for(let a=0;a!==s;++a)e.setTextureCube(t[a]||su,r[a])}function c_(n,t,e){const i=this.cache,s=t.length,r=Ma(e,s);Re(i,r)||(n.uniform1iv(this.addr,r),Ce(i,r));for(let a=0;a!==s;++a)e.setTexture2DArray(t[a]||nu,r[a])}function h_(n){switch(n){case 5126:return Xg;case 35664:return $g;case 35665:return qg;case 35666:return Yg;case 35674:return jg;case 35675:return Kg;case 35676:return Zg;case 5124:case 35670:return Jg;case 35667:case 35671:return Qg;case 35668:case 35672:return t_;case 35669:case 35673:return e_;case 5125:return n_;case 36294:return i_;case 36295:return s_;case 36296:return r_;case 35678:case 36198:case 36298:case 36306:case 35682:return a_;case 35679:case 36299:case 36307:return o_;case 35680:case 36300:case 36308:case 36293:return l_;case 36289:case 36303:case 36311:case 36292:return c_}}class u_{constructor(t,e,i){this.id=t,this.addr=i,this.cache=[],this.type=e.type,this.setValue=Wg(e.type)}}class d_{constructor(t,e,i){this.id=t,this.addr=i,this.cache=[],this.type=e.type,this.size=e.size,this.setValue=h_(e.type)}}class f_{constructor(t){this.id=t,this.seq=[],this.map={}}setValue(t,e,i){const s=this.seq;for(let r=0,a=s.length;r!==a;++r){const o=s[r];o.setValue(t,e[o.id],i)}}}const io=/(\w+)(\])?(\[|\.)?/g;function $c(n,t){n.seq.push(t),n.map[t.id]=t}function p_(n,t,e){const i=n.name,s=i.length;for(io.lastIndex=0;;){const r=io.exec(i),a=io.lastIndex;let o=r[1];const c=r[2]==="]",l=r[3];if(c&&(o=o|0),l===void 0||l==="["&&a+2===s){$c(e,l===void 0?new u_(o,n,t):new d_(o,n,t));break}else{let g=e.map[o];g===void 0&&(g=new f_(o),$c(e,g)),e=g}}}class Xr{constructor(t,e){this.seq=[],this.map={};const i=t.getProgramParameter(e,t.ACTIVE_UNIFORMS);for(let a=0;a<i;++a){const o=t.getActiveUniform(e,a),c=t.getUniformLocation(e,o.name);p_(o,c,this)}const s=[],r=[];for(const a of this.seq)a.type===t.SAMPLER_2D_SHADOW||a.type===t.SAMPLER_CUBE_SHADOW||a.type===t.SAMPLER_2D_ARRAY_SHADOW?s.push(a):r.push(a);s.length>0&&(this.seq=s.concat(r))}setValue(t,e,i,s){const r=this.map[e];r!==void 0&&r.setValue(t,i,s)}setOptional(t,e,i){const s=e[i];s!==void 0&&this.setValue(t,i,s)}static upload(t,e,i,s){for(let r=0,a=e.length;r!==a;++r){const o=e[r],c=i[o.id];c.needsUpdate!==!1&&o.setValue(t,c.value,s)}}static seqWithValue(t,e){const i=[];for(let s=0,r=t.length;s!==r;++s){const a=t[s];a.id in e&&i.push(a)}return i}}function qc(n,t,e){const i=n.createShader(t);return n.shaderSource(i,e),n.compileShader(i),i}const m_=37297;let g_=0;function __(n,t){const e=n.split(`
`),i=[],s=Math.max(t-6,0),r=Math.min(t+6,e.length);for(let a=s;a<r;a++){const o=a+1;i.push(`${o===t?">":" "} ${o}: ${e[a]}`)}return i.join(`
`)}const Yc=new zt;function x_(n){jt._getMatrix(Yc,jt.workingColorSpace,n);const t=`mat3( ${Yc.elements.map(e=>e.toFixed(4))} )`;switch(jt.getTransfer(n)){case sa:return[t,"LinearTransferOETF"];case ie:return[t,"sRGBTransferOETF"];default:return It("WebGLProgram: Unsupported color space: ",n),[t,"LinearTransferOETF"]}}function jc(n,t,e){const i=n.getShaderParameter(t,n.COMPILE_STATUS),r=(n.getShaderInfoLog(t)||"").trim();if(i&&r==="")return"";const a=/ERROR: 0:(\d+)/.exec(r);if(a){const o=parseInt(a[1]);return e.toUpperCase()+`

`+r+`

`+__(n.getShaderSource(t),o)}else return r}function v_(n,t){const e=x_(t);return[`vec4 ${n}( vec4 value ) {`,`	return ${e[1]}( vec4( value.rgb * ${e[0]}, value.a ) );`,"}"].join(`
`)}const M_={[vh]:"Linear",[Mh]:"Reinhard",[Sh]:"Cineon",[yh]:"ACESFilmic",[bh]:"AgX",[Th]:"Neutral",[Eh]:"Custom"};function S_(n,t){const e=M_[t];return e===void 0?(It("WebGLProgram: Unsupported toneMapping:",t),"vec3 "+n+"( vec3 color ) { return LinearToneMapping( color ); }"):"vec3 "+n+"( vec3 color ) { return "+e+"ToneMapping( color ); }"}const Pr=new F;function y_(){jt.getLuminanceCoefficients(Pr);const n=Pr.x.toFixed(4),t=Pr.y.toFixed(4),e=Pr.z.toFixed(4);return["float luminance( const in vec3 rgb ) {",`	const vec3 weights = vec3( ${n}, ${t}, ${e} );`,"	return dot( weights, rgb );","}"].join(`
`)}function E_(n){return[n.extensionClipCullDistance?"#extension GL_ANGLE_clip_cull_distance : require":"",n.extensionMultiDraw?"#extension GL_ANGLE_multi_draw : require":""].filter(Ls).join(`
`)}function b_(n){const t=[];for(const e in n){const i=n[e];i!==!1&&t.push("#define "+e+" "+i)}return t.join(`
`)}function T_(n,t){const e={},i=n.getProgramParameter(t,n.ACTIVE_ATTRIBUTES);for(let s=0;s<i;s++){const r=n.getActiveAttrib(t,s),a=r.name;let o=1;r.type===n.FLOAT_MAT2&&(o=2),r.type===n.FLOAT_MAT3&&(o=3),r.type===n.FLOAT_MAT4&&(o=4),e[a]={type:r.type,location:n.getAttribLocation(t,a),locationSize:o}}return e}function Ls(n){return n!==""}function Kc(n,t){const e=t.numSpotLightShadows+t.numSpotLightMaps-t.numSpotLightShadowsWithMaps;return n.replace(/NUM_DIR_LIGHTS/g,t.numDirLights).replace(/NUM_SPOT_LIGHTS/g,t.numSpotLights).replace(/NUM_SPOT_LIGHT_MAPS/g,t.numSpotLightMaps).replace(/NUM_SPOT_LIGHT_COORDS/g,e).replace(/NUM_RECT_AREA_LIGHTS/g,t.numRectAreaLights).replace(/NUM_POINT_LIGHTS/g,t.numPointLights).replace(/NUM_HEMI_LIGHTS/g,t.numHemiLights).replace(/NUM_DIR_LIGHT_SHADOWS/g,t.numDirLightShadows).replace(/NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS/g,t.numSpotLightShadowsWithMaps).replace(/NUM_SPOT_LIGHT_SHADOWS/g,t.numSpotLightShadows).replace(/NUM_POINT_LIGHT_SHADOWS/g,t.numPointLightShadows)}function Zc(n,t){return n.replace(/NUM_CLIPPING_PLANES/g,t.numClippingPlanes).replace(/UNION_CLIPPING_PLANES/g,t.numClippingPlanes-t.numClipIntersection)}const A_=/^[ \t]*#include +<([\w\d./]+)>/gm;function nl(n){return n.replace(A_,R_)}const w_=new Map;function R_(n,t){let e=Xt[t];if(e===void 0){const i=w_.get(t);if(i!==void 0)e=Xt[i],It('WebGLRenderer: Shader chunk "%s" has been deprecated. Use "%s" instead.',t,i);else throw new Error("Can not resolve #include <"+t+">")}return nl(e)}const C_=/#pragma unroll_loop_start\s+for\s*\(\s*int\s+i\s*=\s*(\d+)\s*;\s*i\s*<\s*(\d+)\s*;\s*i\s*\+\+\s*\)\s*{([\s\S]+?)}\s+#pragma unroll_loop_end/g;function Jc(n){return n.replace(C_,P_)}function P_(n,t,e,i){let s="";for(let r=parseInt(t);r<parseInt(e);r++)s+=i.replace(/\[\s*i\s*\]/g,"[ "+r+" ]").replace(/UNROLLED_LOOP_INDEX/g,r);return s}function Qc(n){let t=`precision ${n.precision} float;
	precision ${n.precision} int;
	precision ${n.precision} sampler2D;
	precision ${n.precision} samplerCube;
	precision ${n.precision} sampler3D;
	precision ${n.precision} sampler2DArray;
	precision ${n.precision} sampler2DShadow;
	precision ${n.precision} samplerCubeShadow;
	precision ${n.precision} sampler2DArrayShadow;
	precision ${n.precision} isampler2D;
	precision ${n.precision} isampler3D;
	precision ${n.precision} isamplerCube;
	precision ${n.precision} isampler2DArray;
	precision ${n.precision} usampler2D;
	precision ${n.precision} usampler3D;
	precision ${n.precision} usamplerCube;
	precision ${n.precision} usampler2DArray;
	`;return n.precision==="highp"?t+=`
#define HIGH_PRECISION`:n.precision==="mediump"?t+=`
#define MEDIUM_PRECISION`:n.precision==="lowp"&&(t+=`
#define LOW_PRECISION`),t}const L_={[zr]:"SHADOWMAP_TYPE_PCF",[Ps]:"SHADOWMAP_TYPE_VSM"};function D_(n){return L_[n.shadowMapType]||"SHADOWMAP_TYPE_BASIC"}const I_={[Pi]:"ENVMAP_TYPE_CUBE",[ms]:"ENVMAP_TYPE_CUBE",[ga]:"ENVMAP_TYPE_CUBE_UV"};function U_(n){return n.envMap===!1?"ENVMAP_TYPE_CUBE":I_[n.envMapMode]||"ENVMAP_TYPE_CUBE"}const N_={[ms]:"ENVMAP_MODE_REFRACTION"};function F_(n){return n.envMap===!1?"ENVMAP_MODE_REFLECTION":N_[n.envMapMode]||"ENVMAP_MODE_REFLECTION"}const O_={[xh]:"ENVMAP_BLENDING_MULTIPLY",[yd]:"ENVMAP_BLENDING_MIX",[Ed]:"ENVMAP_BLENDING_ADD"};function B_(n){return n.envMap===!1?"ENVMAP_BLENDING_NONE":O_[n.combine]||"ENVMAP_BLENDING_NONE"}function z_(n){const t=n.envMapCubeUVHeight;if(t===null)return null;const e=Math.log2(t)-2,i=1/t;return{texelWidth:1/(3*Math.max(Math.pow(2,e),7*16)),texelHeight:i,maxMip:e}}function k_(n,t,e,i){const s=n.getContext(),r=e.defines;let a=e.vertexShader,o=e.fragmentShader;const c=D_(e),l=U_(e),u=F_(e),g=B_(e),h=z_(e),m=E_(e),_=b_(r),y=s.createProgram();let f,d,E=e.glslVersion?"#version "+e.glslVersion+`
`:"";e.isRawShaderMaterial?(f=["#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,_].filter(Ls).join(`
`),f.length>0&&(f+=`
`),d=["#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,_].filter(Ls).join(`
`),d.length>0&&(d+=`
`)):(f=[Qc(e),"#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,_,e.extensionClipCullDistance?"#define USE_CLIP_DISTANCE":"",e.batching?"#define USE_BATCHING":"",e.batchingColor?"#define USE_BATCHING_COLOR":"",e.instancing?"#define USE_INSTANCING":"",e.instancingColor?"#define USE_INSTANCING_COLOR":"",e.instancingMorph?"#define USE_INSTANCING_MORPH":"",e.useFog&&e.fog?"#define USE_FOG":"",e.useFog&&e.fogExp2?"#define FOG_EXP2":"",e.map?"#define USE_MAP":"",e.envMap?"#define USE_ENVMAP":"",e.envMap?"#define "+u:"",e.lightMap?"#define USE_LIGHTMAP":"",e.aoMap?"#define USE_AOMAP":"",e.bumpMap?"#define USE_BUMPMAP":"",e.normalMap?"#define USE_NORMALMAP":"",e.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",e.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",e.displacementMap?"#define USE_DISPLACEMENTMAP":"",e.emissiveMap?"#define USE_EMISSIVEMAP":"",e.anisotropy?"#define USE_ANISOTROPY":"",e.anisotropyMap?"#define USE_ANISOTROPYMAP":"",e.clearcoatMap?"#define USE_CLEARCOATMAP":"",e.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",e.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",e.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",e.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",e.specularMap?"#define USE_SPECULARMAP":"",e.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",e.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",e.roughnessMap?"#define USE_ROUGHNESSMAP":"",e.metalnessMap?"#define USE_METALNESSMAP":"",e.alphaMap?"#define USE_ALPHAMAP":"",e.alphaHash?"#define USE_ALPHAHASH":"",e.transmission?"#define USE_TRANSMISSION":"",e.transmissionMap?"#define USE_TRANSMISSIONMAP":"",e.thicknessMap?"#define USE_THICKNESSMAP":"",e.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",e.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",e.mapUv?"#define MAP_UV "+e.mapUv:"",e.alphaMapUv?"#define ALPHAMAP_UV "+e.alphaMapUv:"",e.lightMapUv?"#define LIGHTMAP_UV "+e.lightMapUv:"",e.aoMapUv?"#define AOMAP_UV "+e.aoMapUv:"",e.emissiveMapUv?"#define EMISSIVEMAP_UV "+e.emissiveMapUv:"",e.bumpMapUv?"#define BUMPMAP_UV "+e.bumpMapUv:"",e.normalMapUv?"#define NORMALMAP_UV "+e.normalMapUv:"",e.displacementMapUv?"#define DISPLACEMENTMAP_UV "+e.displacementMapUv:"",e.metalnessMapUv?"#define METALNESSMAP_UV "+e.metalnessMapUv:"",e.roughnessMapUv?"#define ROUGHNESSMAP_UV "+e.roughnessMapUv:"",e.anisotropyMapUv?"#define ANISOTROPYMAP_UV "+e.anisotropyMapUv:"",e.clearcoatMapUv?"#define CLEARCOATMAP_UV "+e.clearcoatMapUv:"",e.clearcoatNormalMapUv?"#define CLEARCOAT_NORMALMAP_UV "+e.clearcoatNormalMapUv:"",e.clearcoatRoughnessMapUv?"#define CLEARCOAT_ROUGHNESSMAP_UV "+e.clearcoatRoughnessMapUv:"",e.iridescenceMapUv?"#define IRIDESCENCEMAP_UV "+e.iridescenceMapUv:"",e.iridescenceThicknessMapUv?"#define IRIDESCENCE_THICKNESSMAP_UV "+e.iridescenceThicknessMapUv:"",e.sheenColorMapUv?"#define SHEEN_COLORMAP_UV "+e.sheenColorMapUv:"",e.sheenRoughnessMapUv?"#define SHEEN_ROUGHNESSMAP_UV "+e.sheenRoughnessMapUv:"",e.specularMapUv?"#define SPECULARMAP_UV "+e.specularMapUv:"",e.specularColorMapUv?"#define SPECULAR_COLORMAP_UV "+e.specularColorMapUv:"",e.specularIntensityMapUv?"#define SPECULAR_INTENSITYMAP_UV "+e.specularIntensityMapUv:"",e.transmissionMapUv?"#define TRANSMISSIONMAP_UV "+e.transmissionMapUv:"",e.thicknessMapUv?"#define THICKNESSMAP_UV "+e.thicknessMapUv:"",e.vertexTangents&&e.flatShading===!1?"#define USE_TANGENT":"",e.vertexNormals?"#define HAS_NORMAL":"",e.vertexColors?"#define USE_COLOR":"",e.vertexAlphas?"#define USE_COLOR_ALPHA":"",e.vertexUv1s?"#define USE_UV1":"",e.vertexUv2s?"#define USE_UV2":"",e.vertexUv3s?"#define USE_UV3":"",e.pointsUvs?"#define USE_POINTS_UV":"",e.flatShading?"#define FLAT_SHADED":"",e.skinning?"#define USE_SKINNING":"",e.morphTargets?"#define USE_MORPHTARGETS":"",e.morphNormals&&e.flatShading===!1?"#define USE_MORPHNORMALS":"",e.morphColors?"#define USE_MORPHCOLORS":"",e.morphTargetsCount>0?"#define MORPHTARGETS_TEXTURE_STRIDE "+e.morphTextureStride:"",e.morphTargetsCount>0?"#define MORPHTARGETS_COUNT "+e.morphTargetsCount:"",e.doubleSided?"#define DOUBLE_SIDED":"",e.flipSided?"#define FLIP_SIDED":"",e.shadowMapEnabled?"#define USE_SHADOWMAP":"",e.shadowMapEnabled?"#define "+c:"",e.sizeAttenuation?"#define USE_SIZEATTENUATION":"",e.numLightProbes>0?"#define USE_LIGHT_PROBES":"",e.logarithmicDepthBuffer?"#define USE_LOGARITHMIC_DEPTH_BUFFER":"",e.reversedDepthBuffer?"#define USE_REVERSED_DEPTH_BUFFER":"","uniform mat4 modelMatrix;","uniform mat4 modelViewMatrix;","uniform mat4 projectionMatrix;","uniform mat4 viewMatrix;","uniform mat3 normalMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;","#ifdef USE_INSTANCING","	attribute mat4 instanceMatrix;","#endif","#ifdef USE_INSTANCING_COLOR","	attribute vec3 instanceColor;","#endif","#ifdef USE_INSTANCING_MORPH","	uniform sampler2D morphTexture;","#endif","attribute vec3 position;","attribute vec3 normal;","attribute vec2 uv;","#ifdef USE_UV1","	attribute vec2 uv1;","#endif","#ifdef USE_UV2","	attribute vec2 uv2;","#endif","#ifdef USE_UV3","	attribute vec2 uv3;","#endif","#ifdef USE_TANGENT","	attribute vec4 tangent;","#endif","#if defined( USE_COLOR_ALPHA )","	attribute vec4 color;","#elif defined( USE_COLOR )","	attribute vec3 color;","#endif","#ifdef USE_SKINNING","	attribute vec4 skinIndex;","	attribute vec4 skinWeight;","#endif",`
`].filter(Ls).join(`
`),d=[Qc(e),"#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,_,e.useFog&&e.fog?"#define USE_FOG":"",e.useFog&&e.fogExp2?"#define FOG_EXP2":"",e.alphaToCoverage?"#define ALPHA_TO_COVERAGE":"",e.map?"#define USE_MAP":"",e.matcap?"#define USE_MATCAP":"",e.envMap?"#define USE_ENVMAP":"",e.envMap?"#define "+l:"",e.envMap?"#define "+u:"",e.envMap?"#define "+g:"",h?"#define CUBEUV_TEXEL_WIDTH "+h.texelWidth:"",h?"#define CUBEUV_TEXEL_HEIGHT "+h.texelHeight:"",h?"#define CUBEUV_MAX_MIP "+h.maxMip+".0":"",e.lightMap?"#define USE_LIGHTMAP":"",e.aoMap?"#define USE_AOMAP":"",e.bumpMap?"#define USE_BUMPMAP":"",e.normalMap?"#define USE_NORMALMAP":"",e.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",e.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",e.packedNormalMap?"#define USE_PACKED_NORMALMAP":"",e.emissiveMap?"#define USE_EMISSIVEMAP":"",e.anisotropy?"#define USE_ANISOTROPY":"",e.anisotropyMap?"#define USE_ANISOTROPYMAP":"",e.clearcoat?"#define USE_CLEARCOAT":"",e.clearcoatMap?"#define USE_CLEARCOATMAP":"",e.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",e.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",e.dispersion?"#define USE_DISPERSION":"",e.iridescence?"#define USE_IRIDESCENCE":"",e.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",e.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",e.specularMap?"#define USE_SPECULARMAP":"",e.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",e.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",e.roughnessMap?"#define USE_ROUGHNESSMAP":"",e.metalnessMap?"#define USE_METALNESSMAP":"",e.alphaMap?"#define USE_ALPHAMAP":"",e.alphaTest?"#define USE_ALPHATEST":"",e.alphaHash?"#define USE_ALPHAHASH":"",e.sheen?"#define USE_SHEEN":"",e.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",e.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",e.transmission?"#define USE_TRANSMISSION":"",e.transmissionMap?"#define USE_TRANSMISSIONMAP":"",e.thicknessMap?"#define USE_THICKNESSMAP":"",e.vertexTangents&&e.flatShading===!1?"#define USE_TANGENT":"",e.vertexColors||e.instancingColor?"#define USE_COLOR":"",e.vertexAlphas||e.batchingColor?"#define USE_COLOR_ALPHA":"",e.vertexUv1s?"#define USE_UV1":"",e.vertexUv2s?"#define USE_UV2":"",e.vertexUv3s?"#define USE_UV3":"",e.pointsUvs?"#define USE_POINTS_UV":"",e.gradientMap?"#define USE_GRADIENTMAP":"",e.flatShading?"#define FLAT_SHADED":"",e.doubleSided?"#define DOUBLE_SIDED":"",e.flipSided?"#define FLIP_SIDED":"",e.shadowMapEnabled?"#define USE_SHADOWMAP":"",e.shadowMapEnabled?"#define "+c:"",e.premultipliedAlpha?"#define PREMULTIPLIED_ALPHA":"",e.numLightProbes>0?"#define USE_LIGHT_PROBES":"",e.numLightProbeGrids>0?"#define USE_LIGHT_PROBES_GRID":"",e.decodeVideoTexture?"#define DECODE_VIDEO_TEXTURE":"",e.decodeVideoTextureEmissive?"#define DECODE_VIDEO_TEXTURE_EMISSIVE":"",e.logarithmicDepthBuffer?"#define USE_LOGARITHMIC_DEPTH_BUFFER":"",e.reversedDepthBuffer?"#define USE_REVERSED_DEPTH_BUFFER":"","uniform mat4 viewMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;",e.toneMapping!==Cn?"#define TONE_MAPPING":"",e.toneMapping!==Cn?Xt.tonemapping_pars_fragment:"",e.toneMapping!==Cn?S_("toneMapping",e.toneMapping):"",e.dithering?"#define DITHERING":"",e.opaque?"#define OPAQUE":"",Xt.colorspace_pars_fragment,v_("linearToOutputTexel",e.outputColorSpace),y_(),e.useDepthPacking?"#define DEPTH_PACKING "+e.depthPacking:"",`
`].filter(Ls).join(`
`)),a=nl(a),a=Kc(a,e),a=Zc(a,e),o=nl(o),o=Kc(o,e),o=Zc(o,e),a=Jc(a),o=Jc(o),e.isRawShaderMaterial!==!0&&(E=`#version 300 es
`,f=[m,"#define attribute in","#define varying out","#define texture2D texture"].join(`
`)+`
`+f,d=["#define varying in",e.glslVersion===ac?"":"layout(location = 0) out highp vec4 pc_fragColor;",e.glslVersion===ac?"":"#define gl_FragColor pc_fragColor","#define gl_FragDepthEXT gl_FragDepth","#define texture2D texture","#define textureCube texture","#define texture2DProj textureProj","#define texture2DLodEXT textureLod","#define texture2DProjLodEXT textureProjLod","#define textureCubeLodEXT textureLod","#define texture2DGradEXT textureGrad","#define texture2DProjGradEXT textureProjGrad","#define textureCubeGradEXT textureGrad"].join(`
`)+`
`+d);const A=E+f+a,T=E+d+o,D=qc(s,s.VERTEX_SHADER,A),b=qc(s,s.FRAGMENT_SHADER,T);s.attachShader(y,D),s.attachShader(y,b),e.index0AttributeName!==void 0?s.bindAttribLocation(y,0,e.index0AttributeName):e.morphTargets===!0&&s.bindAttribLocation(y,0,"position"),s.linkProgram(y);function N(P){if(n.debug.checkShaderErrors){const V=s.getProgramInfoLog(y)||"",J=s.getShaderInfoLog(D)||"",K=s.getShaderInfoLog(b)||"",B=V.trim(),W=J.trim(),$=K.trim();let et=!0,at=!0;if(s.getProgramParameter(y,s.LINK_STATUS)===!1)if(et=!1,typeof n.debug.onShaderError=="function")n.debug.onShaderError(s,y,D,b);else{const ct=jc(s,D,"vertex"),yt=jc(s,b,"fragment");Kt("THREE.WebGLProgram: Shader Error "+s.getError()+" - VALIDATE_STATUS "+s.getProgramParameter(y,s.VALIDATE_STATUS)+`

Material Name: `+P.name+`
Material Type: `+P.type+`

Program Info Log: `+B+`
`+ct+`
`+yt)}else B!==""?It("WebGLProgram: Program Info Log:",B):(W===""||$==="")&&(at=!1);at&&(P.diagnostics={runnable:et,programLog:B,vertexShader:{log:W,prefix:f},fragmentShader:{log:$,prefix:d}})}s.deleteShader(D),s.deleteShader(b),M=new Xr(s,y),w=T_(s,y)}let M;this.getUniforms=function(){return M===void 0&&N(this),M};let w;this.getAttributes=function(){return w===void 0&&N(this),w};let z=e.rendererExtensionParallelShaderCompile===!1;return this.isReady=function(){return z===!1&&(z=s.getProgramParameter(y,m_)),z},this.destroy=function(){i.releaseStatesOfProgram(this),s.deleteProgram(y),this.program=void 0},this.type=e.shaderType,this.name=e.shaderName,this.id=g_++,this.cacheKey=t,this.usedTimes=1,this.program=y,this.vertexShader=D,this.fragmentShader=b,this}let H_=0;class G_{constructor(){this.shaderCache=new Map,this.materialCache=new Map}update(t){const e=t.vertexShader,i=t.fragmentShader,s=this._getShaderStage(e),r=this._getShaderStage(i),a=this._getShaderCacheForMaterial(t);return a.has(s)===!1&&(a.add(s),s.usedTimes++),a.has(r)===!1&&(a.add(r),r.usedTimes++),this}remove(t){const e=this.materialCache.get(t);for(const i of e)i.usedTimes--,i.usedTimes===0&&this.shaderCache.delete(i.code);return this.materialCache.delete(t),this}getVertexShaderID(t){return this._getShaderStage(t.vertexShader).id}getFragmentShaderID(t){return this._getShaderStage(t.fragmentShader).id}dispose(){this.shaderCache.clear(),this.materialCache.clear()}_getShaderCacheForMaterial(t){const e=this.materialCache;let i=e.get(t);return i===void 0&&(i=new Set,e.set(t,i)),i}_getShaderStage(t){const e=this.shaderCache;let i=e.get(t);return i===void 0&&(i=new V_(t),e.set(t,i)),i}}class V_{constructor(t){this.id=H_++,this.code=t,this.usedTimes=0}}function W_(n){return n===Li||n===ea||n===na}function X_(n,t,e,i,s,r){const a=new Fh,o=new G_,c=new Set,l=[],u=new Map,g=i.logarithmicDepthBuffer;let h=i.precision;const m={MeshDepthMaterial:"depth",MeshDistanceMaterial:"distance",MeshNormalMaterial:"normal",MeshBasicMaterial:"basic",MeshLambertMaterial:"lambert",MeshPhongMaterial:"phong",MeshToonMaterial:"toon",MeshStandardMaterial:"physical",MeshPhysicalMaterial:"physical",MeshMatcapMaterial:"matcap",LineBasicMaterial:"basic",LineDashedMaterial:"dashed",PointsMaterial:"points",ShadowMaterial:"shadow",SpriteMaterial:"sprite"};function _(M){return c.add(M),M===0?"uv":`uv${M}`}function y(M,w,z,P,V,J){const K=P.fog,B=V.geometry,W=M.isMeshStandardMaterial||M.isMeshLambertMaterial||M.isMeshPhongMaterial?P.environment:null,$=M.isMeshStandardMaterial||M.isMeshLambertMaterial&&!M.envMap||M.isMeshPhongMaterial&&!M.envMap,et=t.get(M.envMap||W,$),at=et&&et.mapping===ga?et.image.height:null,ct=m[M.type];M.precision!==null&&(h=i.getMaxPrecision(M.precision),h!==M.precision&&It("WebGLProgram.getParameters:",M.precision,"not supported, using",h,"instead."));const yt=B.morphAttributes.position||B.morphAttributes.normal||B.morphAttributes.color,wt=yt!==void 0?yt.length:0;let kt=0;B.morphAttributes.position!==void 0&&(kt=1),B.morphAttributes.normal!==void 0&&(kt=2),B.morphAttributes.color!==void 0&&(kt=3);let Qt,C,U,nt;if(ct){const Gt=En[ct];Qt=Gt.vertexShader,C=Gt.fragmentShader}else Qt=M.vertexShader,C=M.fragmentShader,o.update(M),U=o.getVertexShaderID(M),nt=o.getFragmentShaderID(M);const st=n.getRenderTarget(),Mt=n.state.buffers.depth.getReversed(),bt=V.isInstancedMesh===!0,Rt=V.isBatchedMesh===!0,qt=!!M.map,Ut=!!M.matcap,ee=!!et,ce=!!M.aoMap,Ht=!!M.lightMap,pe=!!M.bumpMap,se=!!M.normalMap,be=!!M.displacementMap,S=!!M.emissiveMap,j=!!M.metalnessMap,Q=!!M.roughnessMap,ot=M.anisotropy>0,Y=M.clearcoat>0,vt=M.dispersion>0,x=M.iridescence>0,p=M.sheen>0,R=M.transmission>0,k=ot&&!!M.anisotropyMap,Z=Y&&!!M.clearcoatMap,it=Y&&!!M.clearcoatNormalMap,lt=Y&&!!M.clearcoatRoughnessMap,I=x&&!!M.iridescenceMap,X=x&&!!M.iridescenceThicknessMap,dt=p&&!!M.sheenColorMap,_t=p&&!!M.sheenRoughnessMap,ft=!!M.specularMap,ht=!!M.specularColorMap,Ft=!!M.specularIntensityMap,Wt=R&&!!M.transmissionMap,te=R&&!!M.thicknessMap,L=!!M.gradientMap,ut=!!M.alphaMap,tt=M.alphaTest>0,St=!!M.alphaHash,pt=!!M.extensions;let rt=Cn;M.toneMapped&&(st===null||st.isXRRenderTarget===!0)&&(rt=n.toneMapping);const Pt={shaderID:ct,shaderType:M.type,shaderName:M.name,vertexShader:Qt,fragmentShader:C,defines:M.defines,customVertexShaderID:U,customFragmentShaderID:nt,isRawShaderMaterial:M.isRawShaderMaterial===!0,glslVersion:M.glslVersion,precision:h,batching:Rt,batchingColor:Rt&&V._colorsTexture!==null,instancing:bt,instancingColor:bt&&V.instanceColor!==null,instancingMorph:bt&&V.morphTexture!==null,outputColorSpace:st===null?n.outputColorSpace:st.isXRRenderTarget===!0?st.texture.colorSpace:jt.workingColorSpace,alphaToCoverage:!!M.alphaToCoverage,map:qt,matcap:Ut,envMap:ee,envMapMode:ee&&et.mapping,envMapCubeUVHeight:at,aoMap:ce,lightMap:Ht,bumpMap:pe,normalMap:se,displacementMap:be,emissiveMap:S,normalMapObjectSpace:se&&M.normalMapType===Ad,normalMapTangentSpace:se&&M.normalMapType===sc,packedNormalMap:se&&M.normalMapType===sc&&W_(M.normalMap.format),metalnessMap:j,roughnessMap:Q,anisotropy:ot,anisotropyMap:k,clearcoat:Y,clearcoatMap:Z,clearcoatNormalMap:it,clearcoatRoughnessMap:lt,dispersion:vt,iridescence:x,iridescenceMap:I,iridescenceThicknessMap:X,sheen:p,sheenColorMap:dt,sheenRoughnessMap:_t,specularMap:ft,specularColorMap:ht,specularIntensityMap:Ft,transmission:R,transmissionMap:Wt,thicknessMap:te,gradientMap:L,opaque:M.transparent===!1&&M.blending===us&&M.alphaToCoverage===!1,alphaMap:ut,alphaTest:tt,alphaHash:St,combine:M.combine,mapUv:qt&&_(M.map.channel),aoMapUv:ce&&_(M.aoMap.channel),lightMapUv:Ht&&_(M.lightMap.channel),bumpMapUv:pe&&_(M.bumpMap.channel),normalMapUv:se&&_(M.normalMap.channel),displacementMapUv:be&&_(M.displacementMap.channel),emissiveMapUv:S&&_(M.emissiveMap.channel),metalnessMapUv:j&&_(M.metalnessMap.channel),roughnessMapUv:Q&&_(M.roughnessMap.channel),anisotropyMapUv:k&&_(M.anisotropyMap.channel),clearcoatMapUv:Z&&_(M.clearcoatMap.channel),clearcoatNormalMapUv:it&&_(M.clearcoatNormalMap.channel),clearcoatRoughnessMapUv:lt&&_(M.clearcoatRoughnessMap.channel),iridescenceMapUv:I&&_(M.iridescenceMap.channel),iridescenceThicknessMapUv:X&&_(M.iridescenceThicknessMap.channel),sheenColorMapUv:dt&&_(M.sheenColorMap.channel),sheenRoughnessMapUv:_t&&_(M.sheenRoughnessMap.channel),specularMapUv:ft&&_(M.specularMap.channel),specularColorMapUv:ht&&_(M.specularColorMap.channel),specularIntensityMapUv:Ft&&_(M.specularIntensityMap.channel),transmissionMapUv:Wt&&_(M.transmissionMap.channel),thicknessMapUv:te&&_(M.thicknessMap.channel),alphaMapUv:ut&&_(M.alphaMap.channel),vertexTangents:!!B.attributes.tangent&&(se||ot),vertexNormals:!!B.attributes.normal,vertexColors:M.vertexColors,vertexAlphas:M.vertexColors===!0&&!!B.attributes.color&&B.attributes.color.itemSize===4,pointsUvs:V.isPoints===!0&&!!B.attributes.uv&&(qt||ut),fog:!!K,useFog:M.fog===!0,fogExp2:!!K&&K.isFogExp2,flatShading:M.wireframe===!1&&(M.flatShading===!0||B.attributes.normal===void 0&&se===!1&&(M.isMeshLambertMaterial||M.isMeshPhongMaterial||M.isMeshStandardMaterial||M.isMeshPhysicalMaterial)),sizeAttenuation:M.sizeAttenuation===!0,logarithmicDepthBuffer:g,reversedDepthBuffer:Mt,skinning:V.isSkinnedMesh===!0,morphTargets:B.morphAttributes.position!==void 0,morphNormals:B.morphAttributes.normal!==void 0,morphColors:B.morphAttributes.color!==void 0,morphTargetsCount:wt,morphTextureStride:kt,numDirLights:w.directional.length,numPointLights:w.point.length,numSpotLights:w.spot.length,numSpotLightMaps:w.spotLightMap.length,numRectAreaLights:w.rectArea.length,numHemiLights:w.hemi.length,numDirLightShadows:w.directionalShadowMap.length,numPointLightShadows:w.pointShadowMap.length,numSpotLightShadows:w.spotShadowMap.length,numSpotLightShadowsWithMaps:w.numSpotLightShadowsWithMaps,numLightProbes:w.numLightProbes,numLightProbeGrids:J.length,numClippingPlanes:r.numPlanes,numClipIntersection:r.numIntersection,dithering:M.dithering,shadowMapEnabled:n.shadowMap.enabled&&z.length>0,shadowMapType:n.shadowMap.type,toneMapping:rt,decodeVideoTexture:qt&&M.map.isVideoTexture===!0&&jt.getTransfer(M.map.colorSpace)===ie,decodeVideoTextureEmissive:S&&M.emissiveMap.isVideoTexture===!0&&jt.getTransfer(M.emissiveMap.colorSpace)===ie,premultipliedAlpha:M.premultipliedAlpha,doubleSided:M.side===Bn,flipSided:M.side===Ke,useDepthPacking:M.depthPacking>=0,depthPacking:M.depthPacking||0,index0AttributeName:M.index0AttributeName,extensionClipCullDistance:pt&&M.extensions.clipCullDistance===!0&&e.has("WEBGL_clip_cull_distance"),extensionMultiDraw:(pt&&M.extensions.multiDraw===!0||Rt)&&e.has("WEBGL_multi_draw"),rendererExtensionParallelShaderCompile:e.has("KHR_parallel_shader_compile"),customProgramCacheKey:M.customProgramCacheKey()};return Pt.vertexUv1s=c.has(1),Pt.vertexUv2s=c.has(2),Pt.vertexUv3s=c.has(3),c.clear(),Pt}function f(M){const w=[];if(M.shaderID?w.push(M.shaderID):(w.push(M.customVertexShaderID),w.push(M.customFragmentShaderID)),M.defines!==void 0)for(const z in M.defines)w.push(z),w.push(M.defines[z]);return M.isRawShaderMaterial===!1&&(d(w,M),E(w,M),w.push(n.outputColorSpace)),w.push(M.customProgramCacheKey),w.join()}function d(M,w){M.push(w.precision),M.push(w.outputColorSpace),M.push(w.envMapMode),M.push(w.envMapCubeUVHeight),M.push(w.mapUv),M.push(w.alphaMapUv),M.push(w.lightMapUv),M.push(w.aoMapUv),M.push(w.bumpMapUv),M.push(w.normalMapUv),M.push(w.displacementMapUv),M.push(w.emissiveMapUv),M.push(w.metalnessMapUv),M.push(w.roughnessMapUv),M.push(w.anisotropyMapUv),M.push(w.clearcoatMapUv),M.push(w.clearcoatNormalMapUv),M.push(w.clearcoatRoughnessMapUv),M.push(w.iridescenceMapUv),M.push(w.iridescenceThicknessMapUv),M.push(w.sheenColorMapUv),M.push(w.sheenRoughnessMapUv),M.push(w.specularMapUv),M.push(w.specularColorMapUv),M.push(w.specularIntensityMapUv),M.push(w.transmissionMapUv),M.push(w.thicknessMapUv),M.push(w.combine),M.push(w.fogExp2),M.push(w.sizeAttenuation),M.push(w.morphTargetsCount),M.push(w.morphAttributeCount),M.push(w.numDirLights),M.push(w.numPointLights),M.push(w.numSpotLights),M.push(w.numSpotLightMaps),M.push(w.numHemiLights),M.push(w.numRectAreaLights),M.push(w.numDirLightShadows),M.push(w.numPointLightShadows),M.push(w.numSpotLightShadows),M.push(w.numSpotLightShadowsWithMaps),M.push(w.numLightProbes),M.push(w.shadowMapType),M.push(w.toneMapping),M.push(w.numClippingPlanes),M.push(w.numClipIntersection),M.push(w.depthPacking)}function E(M,w){a.disableAll(),w.instancing&&a.enable(0),w.instancingColor&&a.enable(1),w.instancingMorph&&a.enable(2),w.matcap&&a.enable(3),w.envMap&&a.enable(4),w.normalMapObjectSpace&&a.enable(5),w.normalMapTangentSpace&&a.enable(6),w.clearcoat&&a.enable(7),w.iridescence&&a.enable(8),w.alphaTest&&a.enable(9),w.vertexColors&&a.enable(10),w.vertexAlphas&&a.enable(11),w.vertexUv1s&&a.enable(12),w.vertexUv2s&&a.enable(13),w.vertexUv3s&&a.enable(14),w.vertexTangents&&a.enable(15),w.anisotropy&&a.enable(16),w.alphaHash&&a.enable(17),w.batching&&a.enable(18),w.dispersion&&a.enable(19),w.batchingColor&&a.enable(20),w.gradientMap&&a.enable(21),w.packedNormalMap&&a.enable(22),w.vertexNormals&&a.enable(23),M.push(a.mask),a.disableAll(),w.fog&&a.enable(0),w.useFog&&a.enable(1),w.flatShading&&a.enable(2),w.logarithmicDepthBuffer&&a.enable(3),w.reversedDepthBuffer&&a.enable(4),w.skinning&&a.enable(5),w.morphTargets&&a.enable(6),w.morphNormals&&a.enable(7),w.morphColors&&a.enable(8),w.premultipliedAlpha&&a.enable(9),w.shadowMapEnabled&&a.enable(10),w.doubleSided&&a.enable(11),w.flipSided&&a.enable(12),w.useDepthPacking&&a.enable(13),w.dithering&&a.enable(14),w.transmission&&a.enable(15),w.sheen&&a.enable(16),w.opaque&&a.enable(17),w.pointsUvs&&a.enable(18),w.decodeVideoTexture&&a.enable(19),w.decodeVideoTextureEmissive&&a.enable(20),w.alphaToCoverage&&a.enable(21),w.numLightProbeGrids>0&&a.enable(22),M.push(a.mask)}function A(M){const w=m[M.type];let z;if(w){const P=En[w];z=pf.clone(P.uniforms)}else z=M.uniforms;return z}function T(M,w){let z=u.get(w);return z!==void 0?++z.usedTimes:(z=new k_(n,w,M,s),l.push(z),u.set(w,z)),z}function D(M){if(--M.usedTimes===0){const w=l.indexOf(M);l[w]=l[l.length-1],l.pop(),u.delete(M.cacheKey),M.destroy()}}function b(M){o.remove(M)}function N(){o.dispose()}return{getParameters:y,getProgramCacheKey:f,getUniforms:A,acquireProgram:T,releaseProgram:D,releaseShaderCache:b,programs:l,dispose:N}}function $_(){let n=new WeakMap;function t(a){return n.has(a)}function e(a){let o=n.get(a);return o===void 0&&(o={},n.set(a,o)),o}function i(a){n.delete(a)}function s(a,o,c){n.get(a)[o]=c}function r(){n=new WeakMap}return{has:t,get:e,remove:i,update:s,dispose:r}}function q_(n,t){return n.groupOrder!==t.groupOrder?n.groupOrder-t.groupOrder:n.renderOrder!==t.renderOrder?n.renderOrder-t.renderOrder:n.material.id!==t.material.id?n.material.id-t.material.id:n.materialVariant!==t.materialVariant?n.materialVariant-t.materialVariant:n.z!==t.z?n.z-t.z:n.id-t.id}function th(n,t){return n.groupOrder!==t.groupOrder?n.groupOrder-t.groupOrder:n.renderOrder!==t.renderOrder?n.renderOrder-t.renderOrder:n.z!==t.z?t.z-n.z:n.id-t.id}function eh(){const n=[];let t=0;const e=[],i=[],s=[];function r(){t=0,e.length=0,i.length=0,s.length=0}function a(h){let m=0;return h.isInstancedMesh&&(m+=2),h.isSkinnedMesh&&(m+=1),m}function o(h,m,_,y,f,d){let E=n[t];return E===void 0?(E={id:h.id,object:h,geometry:m,material:_,materialVariant:a(h),groupOrder:y,renderOrder:h.renderOrder,z:f,group:d},n[t]=E):(E.id=h.id,E.object=h,E.geometry=m,E.material=_,E.materialVariant=a(h),E.groupOrder=y,E.renderOrder=h.renderOrder,E.z=f,E.group=d),t++,E}function c(h,m,_,y,f,d){const E=o(h,m,_,y,f,d);_.transmission>0?i.push(E):_.transparent===!0?s.push(E):e.push(E)}function l(h,m,_,y,f,d){const E=o(h,m,_,y,f,d);_.transmission>0?i.unshift(E):_.transparent===!0?s.unshift(E):e.unshift(E)}function u(h,m){e.length>1&&e.sort(h||q_),i.length>1&&i.sort(m||th),s.length>1&&s.sort(m||th)}function g(){for(let h=t,m=n.length;h<m;h++){const _=n[h];if(_.id===null)break;_.id=null,_.object=null,_.geometry=null,_.material=null,_.group=null}}return{opaque:e,transmissive:i,transparent:s,init:r,push:c,unshift:l,finish:g,sort:u}}function Y_(){let n=new WeakMap;function t(i,s){const r=n.get(i);let a;return r===void 0?(a=new eh,n.set(i,[a])):s>=r.length?(a=new eh,r.push(a)):a=r[s],a}function e(){n=new WeakMap}return{get:t,dispose:e}}function j_(){const n={};return{get:function(t){if(n[t.id]!==void 0)return n[t.id];let e;switch(t.type){case"DirectionalLight":e={direction:new F,color:new Bt};break;case"SpotLight":e={position:new F,direction:new F,color:new Bt,distance:0,coneCos:0,penumbraCos:0,decay:0};break;case"PointLight":e={position:new F,color:new Bt,distance:0,decay:0};break;case"HemisphereLight":e={direction:new F,skyColor:new Bt,groundColor:new Bt};break;case"RectAreaLight":e={color:new Bt,position:new F,halfWidth:new F,halfHeight:new F};break}return n[t.id]=e,e}}}function K_(){const n={};return{get:function(t){if(n[t.id]!==void 0)return n[t.id];let e;switch(t.type){case"DirectionalLight":e={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Nt};break;case"SpotLight":e={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Nt};break;case"PointLight":e={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Nt,shadowCameraNear:1,shadowCameraFar:1e3};break}return n[t.id]=e,e}}}let Z_=0;function J_(n,t){return(t.castShadow?2:0)-(n.castShadow?2:0)+(t.map?1:0)-(n.map?1:0)}function Q_(n){const t=new j_,e=K_(),i={version:0,hash:{directionalLength:-1,pointLength:-1,spotLength:-1,rectAreaLength:-1,hemiLength:-1,numDirectionalShadows:-1,numPointShadows:-1,numSpotShadows:-1,numSpotMaps:-1,numLightProbes:-1},ambient:[0,0,0],probe:[],directional:[],directionalShadow:[],directionalShadowMap:[],directionalShadowMatrix:[],spot:[],spotLightMap:[],spotShadow:[],spotShadowMap:[],spotLightMatrix:[],rectArea:[],rectAreaLTC1:null,rectAreaLTC2:null,point:[],pointShadow:[],pointShadowMap:[],pointShadowMatrix:[],hemi:[],numSpotLightShadowsWithMaps:0,numLightProbes:0};for(let l=0;l<9;l++)i.probe.push(new F);const s=new F,r=new me,a=new me;function o(l){let u=0,g=0,h=0;for(let w=0;w<9;w++)i.probe[w].set(0,0,0);let m=0,_=0,y=0,f=0,d=0,E=0,A=0,T=0,D=0,b=0,N=0;l.sort(J_);for(let w=0,z=l.length;w<z;w++){const P=l[w],V=P.color,J=P.intensity,K=P.distance;let B=null;if(P.shadow&&P.shadow.map&&(P.shadow.map.texture.format===Li?B=P.shadow.map.texture:B=P.shadow.map.depthTexture||P.shadow.map.texture),P.isAmbientLight)u+=V.r*J,g+=V.g*J,h+=V.b*J;else if(P.isLightProbe){for(let W=0;W<9;W++)i.probe[W].addScaledVector(P.sh.coefficients[W],J);N++}else if(P.isDirectionalLight){const W=t.get(P);if(W.color.copy(P.color).multiplyScalar(P.intensity),P.castShadow){const $=P.shadow,et=e.get(P);et.shadowIntensity=$.intensity,et.shadowBias=$.bias,et.shadowNormalBias=$.normalBias,et.shadowRadius=$.radius,et.shadowMapSize=$.mapSize,i.directionalShadow[m]=et,i.directionalShadowMap[m]=B,i.directionalShadowMatrix[m]=P.shadow.matrix,E++}i.directional[m]=W,m++}else if(P.isSpotLight){const W=t.get(P);W.position.setFromMatrixPosition(P.matrixWorld),W.color.copy(V).multiplyScalar(J),W.distance=K,W.coneCos=Math.cos(P.angle),W.penumbraCos=Math.cos(P.angle*(1-P.penumbra)),W.decay=P.decay,i.spot[y]=W;const $=P.shadow;if(P.map&&(i.spotLightMap[D]=P.map,D++,$.updateMatrices(P),P.castShadow&&b++),i.spotLightMatrix[y]=$.matrix,P.castShadow){const et=e.get(P);et.shadowIntensity=$.intensity,et.shadowBias=$.bias,et.shadowNormalBias=$.normalBias,et.shadowRadius=$.radius,et.shadowMapSize=$.mapSize,i.spotShadow[y]=et,i.spotShadowMap[y]=B,T++}y++}else if(P.isRectAreaLight){const W=t.get(P);W.color.copy(V).multiplyScalar(J),W.halfWidth.set(P.width*.5,0,0),W.halfHeight.set(0,P.height*.5,0),i.rectArea[f]=W,f++}else if(P.isPointLight){const W=t.get(P);if(W.color.copy(P.color).multiplyScalar(P.intensity),W.distance=P.distance,W.decay=P.decay,P.castShadow){const $=P.shadow,et=e.get(P);et.shadowIntensity=$.intensity,et.shadowBias=$.bias,et.shadowNormalBias=$.normalBias,et.shadowRadius=$.radius,et.shadowMapSize=$.mapSize,et.shadowCameraNear=$.camera.near,et.shadowCameraFar=$.camera.far,i.pointShadow[_]=et,i.pointShadowMap[_]=B,i.pointShadowMatrix[_]=P.shadow.matrix,A++}i.point[_]=W,_++}else if(P.isHemisphereLight){const W=t.get(P);W.skyColor.copy(P.color).multiplyScalar(J),W.groundColor.copy(P.groundColor).multiplyScalar(J),i.hemi[d]=W,d++}}f>0&&(n.has("OES_texture_float_linear")===!0?(i.rectAreaLTC1=mt.LTC_FLOAT_1,i.rectAreaLTC2=mt.LTC_FLOAT_2):(i.rectAreaLTC1=mt.LTC_HALF_1,i.rectAreaLTC2=mt.LTC_HALF_2)),i.ambient[0]=u,i.ambient[1]=g,i.ambient[2]=h;const M=i.hash;(M.directionalLength!==m||M.pointLength!==_||M.spotLength!==y||M.rectAreaLength!==f||M.hemiLength!==d||M.numDirectionalShadows!==E||M.numPointShadows!==A||M.numSpotShadows!==T||M.numSpotMaps!==D||M.numLightProbes!==N)&&(i.directional.length=m,i.spot.length=y,i.rectArea.length=f,i.point.length=_,i.hemi.length=d,i.directionalShadow.length=E,i.directionalShadowMap.length=E,i.pointShadow.length=A,i.pointShadowMap.length=A,i.spotShadow.length=T,i.spotShadowMap.length=T,i.directionalShadowMatrix.length=E,i.pointShadowMatrix.length=A,i.spotLightMatrix.length=T+D-b,i.spotLightMap.length=D,i.numSpotLightShadowsWithMaps=b,i.numLightProbes=N,M.directionalLength=m,M.pointLength=_,M.spotLength=y,M.rectAreaLength=f,M.hemiLength=d,M.numDirectionalShadows=E,M.numPointShadows=A,M.numSpotShadows=T,M.numSpotMaps=D,M.numLightProbes=N,i.version=Z_++)}function c(l,u){let g=0,h=0,m=0,_=0,y=0;const f=u.matrixWorldInverse;for(let d=0,E=l.length;d<E;d++){const A=l[d];if(A.isDirectionalLight){const T=i.directional[g];T.direction.setFromMatrixPosition(A.matrixWorld),s.setFromMatrixPosition(A.target.matrixWorld),T.direction.sub(s),T.direction.transformDirection(f),g++}else if(A.isSpotLight){const T=i.spot[m];T.position.setFromMatrixPosition(A.matrixWorld),T.position.applyMatrix4(f),T.direction.setFromMatrixPosition(A.matrixWorld),s.setFromMatrixPosition(A.target.matrixWorld),T.direction.sub(s),T.direction.transformDirection(f),m++}else if(A.isRectAreaLight){const T=i.rectArea[_];T.position.setFromMatrixPosition(A.matrixWorld),T.position.applyMatrix4(f),a.identity(),r.copy(A.matrixWorld),r.premultiply(f),a.extractRotation(r),T.halfWidth.set(A.width*.5,0,0),T.halfHeight.set(0,A.height*.5,0),T.halfWidth.applyMatrix4(a),T.halfHeight.applyMatrix4(a),_++}else if(A.isPointLight){const T=i.point[h];T.position.setFromMatrixPosition(A.matrixWorld),T.position.applyMatrix4(f),h++}else if(A.isHemisphereLight){const T=i.hemi[y];T.direction.setFromMatrixPosition(A.matrixWorld),T.direction.transformDirection(f),y++}}}return{setup:o,setupView:c,state:i}}function nh(n){const t=new Q_(n),e=[],i=[],s=[];function r(h){g.camera=h,e.length=0,i.length=0,s.length=0}function a(h){e.push(h)}function o(h){i.push(h)}function c(h){s.push(h)}function l(){t.setup(e)}function u(h){t.setupView(e,h)}const g={lightsArray:e,shadowsArray:i,lightProbeGridArray:s,camera:null,lights:t,transmissionRenderTarget:{},textureUnits:0};return{init:r,state:g,setupLights:l,setupLightsView:u,pushLight:a,pushShadow:o,pushLightProbeGrid:c}}function t0(n){let t=new WeakMap;function e(s,r=0){const a=t.get(s);let o;return a===void 0?(o=new nh(n),t.set(s,[o])):r>=a.length?(o=new nh(n),a.push(o)):o=a[r],o}function i(){t=new WeakMap}return{get:e,dispose:i}}const e0=`void main() {
	gl_Position = vec4( position, 1.0 );
}`,n0=`uniform sampler2D shadow_pass;
uniform vec2 resolution;
uniform float radius;
void main() {
	const float samples = float( VSM_SAMPLES );
	float mean = 0.0;
	float squared_mean = 0.0;
	float uvStride = samples <= 1.0 ? 0.0 : 2.0 / ( samples - 1.0 );
	float uvStart = samples <= 1.0 ? 0.0 : - 1.0;
	for ( float i = 0.0; i < samples; i ++ ) {
		float uvOffset = uvStart + i * uvStride;
		#ifdef HORIZONTAL_PASS
			vec2 distribution = texture2D( shadow_pass, ( gl_FragCoord.xy + vec2( uvOffset, 0.0 ) * radius ) / resolution ).rg;
			mean += distribution.x;
			squared_mean += distribution.y * distribution.y + distribution.x * distribution.x;
		#else
			float depth = texture2D( shadow_pass, ( gl_FragCoord.xy + vec2( 0.0, uvOffset ) * radius ) / resolution ).r;
			mean += depth;
			squared_mean += depth * depth;
		#endif
	}
	mean = mean / samples;
	squared_mean = squared_mean / samples;
	float std_dev = sqrt( max( 0.0, squared_mean - mean * mean ) );
	gl_FragColor = vec4( mean, std_dev, 0.0, 1.0 );
}`,i0=[new F(1,0,0),new F(-1,0,0),new F(0,1,0),new F(0,-1,0),new F(0,0,1),new F(0,0,-1)],s0=[new F(0,-1,0),new F(0,-1,0),new F(0,0,1),new F(0,0,-1),new F(0,-1,0),new F(0,-1,0)],ih=new me,Cs=new F,so=new F;function r0(n,t,e){let i=new Vh;const s=new Nt,r=new Nt,a=new ye,o=new xf,c=new vf,l={},u=e.maxTextureSize,g={[ci]:Ke,[Ke]:ci,[Bn]:Bn},h=new gn({defines:{VSM_SAMPLES:8},uniforms:{shadow_pass:{value:null},resolution:{value:new Nt},radius:{value:4}},vertexShader:e0,fragmentShader:n0}),m=h.clone();m.defines.HORIZONTAL_PASS=1;const _=new Ue;_.setAttribute("position",new Ae(new Float32Array([-1,-1,.5,3,-1,.5,-1,3,.5]),3));const y=new Xn(_,h),f=this;this.enabled=!1,this.autoUpdate=!0,this.needsUpdate=!1,this.type=zr;let d=this.type;this.render=function(b,N,M){if(f.enabled===!1||f.autoUpdate===!1&&f.needsUpdate===!1||b.length===0)return;this.type===id&&(It("WebGLShadowMap: PCFSoftShadowMap has been deprecated. Using PCFShadowMap instead."),this.type=zr);const w=n.getRenderTarget(),z=n.getActiveCubeFace(),P=n.getActiveMipmapLevel(),V=n.state;V.setBlending(Hn),V.buffers.depth.getReversed()===!0?V.buffers.color.setClear(0,0,0,0):V.buffers.color.setClear(1,1,1,1),V.buffers.depth.setTest(!0),V.setScissorTest(!1);const J=d!==this.type;J&&N.traverse(function(K){K.material&&(Array.isArray(K.material)?K.material.forEach(B=>B.needsUpdate=!0):K.material.needsUpdate=!0)});for(let K=0,B=b.length;K<B;K++){const W=b[K],$=W.shadow;if($===void 0){It("WebGLShadowMap:",W,"has no shadow.");continue}if($.autoUpdate===!1&&$.needsUpdate===!1)continue;s.copy($.mapSize);const et=$.getFrameExtents();s.multiply(et),r.copy($.mapSize),(s.x>u||s.y>u)&&(s.x>u&&(r.x=Math.floor(u/et.x),s.x=r.x*et.x,$.mapSize.x=r.x),s.y>u&&(r.y=Math.floor(u/et.y),s.y=r.y*et.y,$.mapSize.y=r.y));const at=n.state.buffers.depth.getReversed();if($.camera._reversedDepth=at,$.map===null||J===!0){if($.map!==null&&($.map.depthTexture!==null&&($.map.depthTexture.dispose(),$.map.depthTexture=null),$.map.dispose()),this.type===Ps){if(W.isPointLight){It("WebGLShadowMap: VSM shadow maps are not supported for PointLights. Use PCF or BasicShadowMap instead.");continue}$.map=new Pn(s.x,s.y,{format:Li,type:Vn,minFilter:Be,magFilter:Be,generateMipmaps:!1}),$.map.texture.name=W.name+".shadowMap",$.map.depthTexture=new gs(s.x,s.y,An),$.map.depthTexture.name=W.name+".shadowMapDepth",$.map.depthTexture.format=Wn,$.map.depthTexture.compareFunction=null,$.map.depthTexture.minFilter=Le,$.map.depthTexture.magFilter=Le}else W.isPointLight?($.map=new tu(s.x),$.map.depthTexture=new df(s.x,Ln)):($.map=new Pn(s.x,s.y),$.map.depthTexture=new gs(s.x,s.y,Ln)),$.map.depthTexture.name=W.name+".shadowMap",$.map.depthTexture.format=Wn,this.type===zr?($.map.depthTexture.compareFunction=at?bl:El,$.map.depthTexture.minFilter=Be,$.map.depthTexture.magFilter=Be):($.map.depthTexture.compareFunction=null,$.map.depthTexture.minFilter=Le,$.map.depthTexture.magFilter=Le);$.camera.updateProjectionMatrix()}const ct=$.map.isWebGLCubeRenderTarget?6:1;for(let yt=0;yt<ct;yt++){if($.map.isWebGLCubeRenderTarget)n.setRenderTarget($.map,yt),n.clear();else{yt===0&&(n.setRenderTarget($.map),n.clear());const wt=$.getViewport(yt);a.set(r.x*wt.x,r.y*wt.y,r.x*wt.z,r.y*wt.w),V.viewport(a)}if(W.isPointLight){const wt=$.camera,kt=$.matrix,Qt=W.distance||wt.far;Qt!==wt.far&&(wt.far=Qt,wt.updateProjectionMatrix()),Cs.setFromMatrixPosition(W.matrixWorld),wt.position.copy(Cs),so.copy(wt.position),so.add(i0[yt]),wt.up.copy(s0[yt]),wt.lookAt(so),wt.updateMatrixWorld(),kt.makeTranslation(-Cs.x,-Cs.y,-Cs.z),ih.multiplyMatrices(wt.projectionMatrix,wt.matrixWorldInverse),$._frustum.setFromProjectionMatrix(ih,wt.coordinateSystem,wt.reversedDepth)}else $.updateMatrices(W);i=$.getFrustum(),T(N,M,$.camera,W,this.type)}$.isPointLightShadow!==!0&&this.type===Ps&&E($,M),$.needsUpdate=!1}d=this.type,f.needsUpdate=!1,n.setRenderTarget(w,z,P)};function E(b,N){const M=t.update(y);h.defines.VSM_SAMPLES!==b.blurSamples&&(h.defines.VSM_SAMPLES=b.blurSamples,m.defines.VSM_SAMPLES=b.blurSamples,h.needsUpdate=!0,m.needsUpdate=!0),b.mapPass===null&&(b.mapPass=new Pn(s.x,s.y,{format:Li,type:Vn})),h.uniforms.shadow_pass.value=b.map.depthTexture,h.uniforms.resolution.value=b.mapSize,h.uniforms.radius.value=b.radius,n.setRenderTarget(b.mapPass),n.clear(),n.renderBufferDirect(N,null,M,h,y,null),m.uniforms.shadow_pass.value=b.mapPass.texture,m.uniforms.resolution.value=b.mapSize,m.uniforms.radius.value=b.radius,n.setRenderTarget(b.map),n.clear(),n.renderBufferDirect(N,null,M,m,y,null)}function A(b,N,M,w){let z=null;const P=M.isPointLight===!0?b.customDistanceMaterial:b.customDepthMaterial;if(P!==void 0)z=P;else if(z=M.isPointLight===!0?c:o,n.localClippingEnabled&&N.clipShadows===!0&&Array.isArray(N.clippingPlanes)&&N.clippingPlanes.length!==0||N.displacementMap&&N.displacementScale!==0||N.alphaMap&&N.alphaTest>0||N.map&&N.alphaTest>0||N.alphaToCoverage===!0){const V=z.uuid,J=N.uuid;let K=l[V];K===void 0&&(K={},l[V]=K);let B=K[J];B===void 0&&(B=z.clone(),K[J]=B,N.addEventListener("dispose",D)),z=B}if(z.visible=N.visible,z.wireframe=N.wireframe,w===Ps?z.side=N.shadowSide!==null?N.shadowSide:N.side:z.side=N.shadowSide!==null?N.shadowSide:g[N.side],z.alphaMap=N.alphaMap,z.alphaTest=N.alphaToCoverage===!0?.5:N.alphaTest,z.map=N.map,z.clipShadows=N.clipShadows,z.clippingPlanes=N.clippingPlanes,z.clipIntersection=N.clipIntersection,z.displacementMap=N.displacementMap,z.displacementScale=N.displacementScale,z.displacementBias=N.displacementBias,z.wireframeLinewidth=N.wireframeLinewidth,z.linewidth=N.linewidth,M.isPointLight===!0&&z.isMeshDistanceMaterial===!0){const V=n.properties.get(z);V.light=M}return z}function T(b,N,M,w,z){if(b.visible===!1)return;if(b.layers.test(N.layers)&&(b.isMesh||b.isLine||b.isPoints)&&(b.castShadow||b.receiveShadow&&z===Ps)&&(!b.frustumCulled||i.intersectsObject(b))){b.modelViewMatrix.multiplyMatrices(M.matrixWorldInverse,b.matrixWorld);const J=t.update(b),K=b.material;if(Array.isArray(K)){const B=J.groups;for(let W=0,$=B.length;W<$;W++){const et=B[W],at=K[et.materialIndex];if(at&&at.visible){const ct=A(b,at,w,z);b.onBeforeShadow(n,b,N,M,J,ct,et),n.renderBufferDirect(M,null,J,ct,b,et),b.onAfterShadow(n,b,N,M,J,ct,et)}}}else if(K.visible){const B=A(b,K,w,z);b.onBeforeShadow(n,b,N,M,J,B,null),n.renderBufferDirect(M,null,J,B,b,null),b.onAfterShadow(n,b,N,M,J,B,null)}}const V=b.children;for(let J=0,K=V.length;J<K;J++)T(V[J],N,M,w,z)}function D(b){b.target.removeEventListener("dispose",D);for(const M in l){const w=l[M],z=b.target.uuid;z in w&&(w[z].dispose(),delete w[z])}}}function a0(n,t){function e(){let L=!1;const ut=new ye;let tt=null;const St=new ye(0,0,0,0);return{setMask:function(pt){tt!==pt&&!L&&(n.colorMask(pt,pt,pt,pt),tt=pt)},setLocked:function(pt){L=pt},setClear:function(pt,rt,Pt,Gt,xe){xe===!0&&(pt*=Gt,rt*=Gt,Pt*=Gt),ut.set(pt,rt,Pt,Gt),St.equals(ut)===!1&&(n.clearColor(pt,rt,Pt,Gt),St.copy(ut))},reset:function(){L=!1,tt=null,St.set(-1,0,0,0)}}}function i(){let L=!1,ut=!1,tt=null,St=null,pt=null;return{setReversed:function(rt){if(ut!==rt){const Pt=t.get("EXT_clip_control");rt?Pt.clipControlEXT(Pt.LOWER_LEFT_EXT,Pt.ZERO_TO_ONE_EXT):Pt.clipControlEXT(Pt.LOWER_LEFT_EXT,Pt.NEGATIVE_ONE_TO_ONE_EXT),ut=rt;const Gt=pt;pt=null,this.setClear(Gt)}},getReversed:function(){return ut},setTest:function(rt){rt?st(n.DEPTH_TEST):Mt(n.DEPTH_TEST)},setMask:function(rt){tt!==rt&&!L&&(n.depthMask(rt),tt=rt)},setFunc:function(rt){if(ut&&(rt=Fd[rt]),St!==rt){switch(rt){case uo:n.depthFunc(n.NEVER);break;case fo:n.depthFunc(n.ALWAYS);break;case po:n.depthFunc(n.LESS);break;case ps:n.depthFunc(n.LEQUAL);break;case mo:n.depthFunc(n.EQUAL);break;case go:n.depthFunc(n.GEQUAL);break;case _o:n.depthFunc(n.GREATER);break;case xo:n.depthFunc(n.NOTEQUAL);break;default:n.depthFunc(n.LEQUAL)}St=rt}},setLocked:function(rt){L=rt},setClear:function(rt){pt!==rt&&(pt=rt,ut&&(rt=1-rt),n.clearDepth(rt))},reset:function(){L=!1,tt=null,St=null,pt=null,ut=!1}}}function s(){let L=!1,ut=null,tt=null,St=null,pt=null,rt=null,Pt=null,Gt=null,xe=null;return{setTest:function(re){L||(re?st(n.STENCIL_TEST):Mt(n.STENCIL_TEST))},setMask:function(re){ut!==re&&!L&&(n.stencilMask(re),ut=re)},setFunc:function(re,Dn,vn){(tt!==re||St!==Dn||pt!==vn)&&(n.stencilFunc(re,Dn,vn),tt=re,St=Dn,pt=vn)},setOp:function(re,Dn,vn){(rt!==re||Pt!==Dn||Gt!==vn)&&(n.stencilOp(re,Dn,vn),rt=re,Pt=Dn,Gt=vn)},setLocked:function(re){L=re},setClear:function(re){xe!==re&&(n.clearStencil(re),xe=re)},reset:function(){L=!1,ut=null,tt=null,St=null,pt=null,rt=null,Pt=null,Gt=null,xe=null}}}const r=new e,a=new i,o=new s,c=new WeakMap,l=new WeakMap;let u={},g={},h={},m=new WeakMap,_=[],y=null,f=!1,d=null,E=null,A=null,T=null,D=null,b=null,N=null,M=new Bt(0,0,0),w=0,z=!1,P=null,V=null,J=null,K=null,B=null;const W=n.getParameter(n.MAX_COMBINED_TEXTURE_IMAGE_UNITS);let $=!1,et=0;const at=n.getParameter(n.VERSION);at.indexOf("WebGL")!==-1?(et=parseFloat(/^WebGL (\d)/.exec(at)[1]),$=et>=1):at.indexOf("OpenGL ES")!==-1&&(et=parseFloat(/^OpenGL ES (\d)/.exec(at)[1]),$=et>=2);let ct=null,yt={};const wt=n.getParameter(n.SCISSOR_BOX),kt=n.getParameter(n.VIEWPORT),Qt=new ye().fromArray(wt),C=new ye().fromArray(kt);function U(L,ut,tt,St){const pt=new Uint8Array(4),rt=n.createTexture();n.bindTexture(L,rt),n.texParameteri(L,n.TEXTURE_MIN_FILTER,n.NEAREST),n.texParameteri(L,n.TEXTURE_MAG_FILTER,n.NEAREST);for(let Pt=0;Pt<tt;Pt++)L===n.TEXTURE_3D||L===n.TEXTURE_2D_ARRAY?n.texImage3D(ut,0,n.RGBA,1,1,St,0,n.RGBA,n.UNSIGNED_BYTE,pt):n.texImage2D(ut+Pt,0,n.RGBA,1,1,0,n.RGBA,n.UNSIGNED_BYTE,pt);return rt}const nt={};nt[n.TEXTURE_2D]=U(n.TEXTURE_2D,n.TEXTURE_2D,1),nt[n.TEXTURE_CUBE_MAP]=U(n.TEXTURE_CUBE_MAP,n.TEXTURE_CUBE_MAP_POSITIVE_X,6),nt[n.TEXTURE_2D_ARRAY]=U(n.TEXTURE_2D_ARRAY,n.TEXTURE_2D_ARRAY,1,1),nt[n.TEXTURE_3D]=U(n.TEXTURE_3D,n.TEXTURE_3D,1,1),r.setClear(0,0,0,1),a.setClear(1),o.setClear(0),st(n.DEPTH_TEST),a.setFunc(ps),pe(!1),se(ec),st(n.CULL_FACE),ce(Hn);function st(L){u[L]!==!0&&(n.enable(L),u[L]=!0)}function Mt(L){u[L]!==!1&&(n.disable(L),u[L]=!1)}function bt(L,ut){return h[L]!==ut?(n.bindFramebuffer(L,ut),h[L]=ut,L===n.DRAW_FRAMEBUFFER&&(h[n.FRAMEBUFFER]=ut),L===n.FRAMEBUFFER&&(h[n.DRAW_FRAMEBUFFER]=ut),!0):!1}function Rt(L,ut){let tt=_,St=!1;if(L){tt=m.get(ut),tt===void 0&&(tt=[],m.set(ut,tt));const pt=L.textures;if(tt.length!==pt.length||tt[0]!==n.COLOR_ATTACHMENT0){for(let rt=0,Pt=pt.length;rt<Pt;rt++)tt[rt]=n.COLOR_ATTACHMENT0+rt;tt.length=pt.length,St=!0}}else tt[0]!==n.BACK&&(tt[0]=n.BACK,St=!0);St&&n.drawBuffers(tt)}function qt(L){return y!==L?(n.useProgram(L),y=L,!0):!1}const Ut={[Si]:n.FUNC_ADD,[rd]:n.FUNC_SUBTRACT,[ad]:n.FUNC_REVERSE_SUBTRACT};Ut[od]=n.MIN,Ut[ld]=n.MAX;const ee={[cd]:n.ZERO,[hd]:n.ONE,[ud]:n.SRC_COLOR,[co]:n.SRC_ALPHA,[_d]:n.SRC_ALPHA_SATURATE,[md]:n.DST_COLOR,[fd]:n.DST_ALPHA,[dd]:n.ONE_MINUS_SRC_COLOR,[ho]:n.ONE_MINUS_SRC_ALPHA,[gd]:n.ONE_MINUS_DST_COLOR,[pd]:n.ONE_MINUS_DST_ALPHA,[xd]:n.CONSTANT_COLOR,[vd]:n.ONE_MINUS_CONSTANT_COLOR,[Md]:n.CONSTANT_ALPHA,[Sd]:n.ONE_MINUS_CONSTANT_ALPHA};function ce(L,ut,tt,St,pt,rt,Pt,Gt,xe,re){if(L===Hn){f===!0&&(Mt(n.BLEND),f=!1);return}if(f===!1&&(st(n.BLEND),f=!0),L!==sd){if(L!==d||re!==z){if((E!==Si||D!==Si)&&(n.blendEquation(n.FUNC_ADD),E=Si,D=Si),re)switch(L){case us:n.blendFuncSeparate(n.ONE,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case ta:n.blendFunc(n.ONE,n.ONE);break;case nc:n.blendFuncSeparate(n.ZERO,n.ONE_MINUS_SRC_COLOR,n.ZERO,n.ONE);break;case ic:n.blendFuncSeparate(n.DST_COLOR,n.ONE_MINUS_SRC_ALPHA,n.ZERO,n.ONE);break;default:Kt("WebGLState: Invalid blending: ",L);break}else switch(L){case us:n.blendFuncSeparate(n.SRC_ALPHA,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case ta:n.blendFuncSeparate(n.SRC_ALPHA,n.ONE,n.ONE,n.ONE);break;case nc:Kt("WebGLState: SubtractiveBlending requires material.premultipliedAlpha = true");break;case ic:Kt("WebGLState: MultiplyBlending requires material.premultipliedAlpha = true");break;default:Kt("WebGLState: Invalid blending: ",L);break}A=null,T=null,b=null,N=null,M.set(0,0,0),w=0,d=L,z=re}return}pt=pt||ut,rt=rt||tt,Pt=Pt||St,(ut!==E||pt!==D)&&(n.blendEquationSeparate(Ut[ut],Ut[pt]),E=ut,D=pt),(tt!==A||St!==T||rt!==b||Pt!==N)&&(n.blendFuncSeparate(ee[tt],ee[St],ee[rt],ee[Pt]),A=tt,T=St,b=rt,N=Pt),(Gt.equals(M)===!1||xe!==w)&&(n.blendColor(Gt.r,Gt.g,Gt.b,xe),M.copy(Gt),w=xe),d=L,z=!1}function Ht(L,ut){L.side===Bn?Mt(n.CULL_FACE):st(n.CULL_FACE);let tt=L.side===Ke;ut&&(tt=!tt),pe(tt),L.blending===us&&L.transparent===!1?ce(Hn):ce(L.blending,L.blendEquation,L.blendSrc,L.blendDst,L.blendEquationAlpha,L.blendSrcAlpha,L.blendDstAlpha,L.blendColor,L.blendAlpha,L.premultipliedAlpha),a.setFunc(L.depthFunc),a.setTest(L.depthTest),a.setMask(L.depthWrite),r.setMask(L.colorWrite);const St=L.stencilWrite;o.setTest(St),St&&(o.setMask(L.stencilWriteMask),o.setFunc(L.stencilFunc,L.stencilRef,L.stencilFuncMask),o.setOp(L.stencilFail,L.stencilZFail,L.stencilZPass)),S(L.polygonOffset,L.polygonOffsetFactor,L.polygonOffsetUnits),L.alphaToCoverage===!0?st(n.SAMPLE_ALPHA_TO_COVERAGE):Mt(n.SAMPLE_ALPHA_TO_COVERAGE)}function pe(L){P!==L&&(L?n.frontFace(n.CW):n.frontFace(n.CCW),P=L)}function se(L){L!==ed?(st(n.CULL_FACE),L!==V&&(L===ec?n.cullFace(n.BACK):L===nd?n.cullFace(n.FRONT):n.cullFace(n.FRONT_AND_BACK))):Mt(n.CULL_FACE),V=L}function be(L){L!==J&&($&&n.lineWidth(L),J=L)}function S(L,ut,tt){L?(st(n.POLYGON_OFFSET_FILL),(K!==ut||B!==tt)&&(K=ut,B=tt,a.getReversed()&&(ut=-ut),n.polygonOffset(ut,tt))):Mt(n.POLYGON_OFFSET_FILL)}function j(L){L?st(n.SCISSOR_TEST):Mt(n.SCISSOR_TEST)}function Q(L){L===void 0&&(L=n.TEXTURE0+W-1),ct!==L&&(n.activeTexture(L),ct=L)}function ot(L,ut,tt){tt===void 0&&(ct===null?tt=n.TEXTURE0+W-1:tt=ct);let St=yt[tt];St===void 0&&(St={type:void 0,texture:void 0},yt[tt]=St),(St.type!==L||St.texture!==ut)&&(ct!==tt&&(n.activeTexture(tt),ct=tt),n.bindTexture(L,ut||nt[L]),St.type=L,St.texture=ut)}function Y(){const L=yt[ct];L!==void 0&&L.type!==void 0&&(n.bindTexture(L.type,null),L.type=void 0,L.texture=void 0)}function vt(){try{n.compressedTexImage2D(...arguments)}catch(L){Kt("WebGLState:",L)}}function x(){try{n.compressedTexImage3D(...arguments)}catch(L){Kt("WebGLState:",L)}}function p(){try{n.texSubImage2D(...arguments)}catch(L){Kt("WebGLState:",L)}}function R(){try{n.texSubImage3D(...arguments)}catch(L){Kt("WebGLState:",L)}}function k(){try{n.compressedTexSubImage2D(...arguments)}catch(L){Kt("WebGLState:",L)}}function Z(){try{n.compressedTexSubImage3D(...arguments)}catch(L){Kt("WebGLState:",L)}}function it(){try{n.texStorage2D(...arguments)}catch(L){Kt("WebGLState:",L)}}function lt(){try{n.texStorage3D(...arguments)}catch(L){Kt("WebGLState:",L)}}function I(){try{n.texImage2D(...arguments)}catch(L){Kt("WebGLState:",L)}}function X(){try{n.texImage3D(...arguments)}catch(L){Kt("WebGLState:",L)}}function dt(L){return g[L]!==void 0?g[L]:n.getParameter(L)}function _t(L,ut){g[L]!==ut&&(n.pixelStorei(L,ut),g[L]=ut)}function ft(L){Qt.equals(L)===!1&&(n.scissor(L.x,L.y,L.z,L.w),Qt.copy(L))}function ht(L){C.equals(L)===!1&&(n.viewport(L.x,L.y,L.z,L.w),C.copy(L))}function Ft(L,ut){let tt=l.get(ut);tt===void 0&&(tt=new WeakMap,l.set(ut,tt));let St=tt.get(L);St===void 0&&(St=n.getUniformBlockIndex(ut,L.name),tt.set(L,St))}function Wt(L,ut){const St=l.get(ut).get(L);c.get(ut)!==St&&(n.uniformBlockBinding(ut,St,L.__bindingPointIndex),c.set(ut,St))}function te(){n.disable(n.BLEND),n.disable(n.CULL_FACE),n.disable(n.DEPTH_TEST),n.disable(n.POLYGON_OFFSET_FILL),n.disable(n.SCISSOR_TEST),n.disable(n.STENCIL_TEST),n.disable(n.SAMPLE_ALPHA_TO_COVERAGE),n.blendEquation(n.FUNC_ADD),n.blendFunc(n.ONE,n.ZERO),n.blendFuncSeparate(n.ONE,n.ZERO,n.ONE,n.ZERO),n.blendColor(0,0,0,0),n.colorMask(!0,!0,!0,!0),n.clearColor(0,0,0,0),n.depthMask(!0),n.depthFunc(n.LESS),a.setReversed(!1),n.clearDepth(1),n.stencilMask(4294967295),n.stencilFunc(n.ALWAYS,0,4294967295),n.stencilOp(n.KEEP,n.KEEP,n.KEEP),n.clearStencil(0),n.cullFace(n.BACK),n.frontFace(n.CCW),n.polygonOffset(0,0),n.activeTexture(n.TEXTURE0),n.bindFramebuffer(n.FRAMEBUFFER,null),n.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),n.bindFramebuffer(n.READ_FRAMEBUFFER,null),n.useProgram(null),n.lineWidth(1),n.scissor(0,0,n.canvas.width,n.canvas.height),n.viewport(0,0,n.canvas.width,n.canvas.height),n.pixelStorei(n.PACK_ALIGNMENT,4),n.pixelStorei(n.UNPACK_ALIGNMENT,4),n.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,!1),n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!1),n.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,n.BROWSER_DEFAULT_WEBGL),n.pixelStorei(n.PACK_ROW_LENGTH,0),n.pixelStorei(n.PACK_SKIP_PIXELS,0),n.pixelStorei(n.PACK_SKIP_ROWS,0),n.pixelStorei(n.UNPACK_ROW_LENGTH,0),n.pixelStorei(n.UNPACK_IMAGE_HEIGHT,0),n.pixelStorei(n.UNPACK_SKIP_PIXELS,0),n.pixelStorei(n.UNPACK_SKIP_ROWS,0),n.pixelStorei(n.UNPACK_SKIP_IMAGES,0),u={},g={},ct=null,yt={},h={},m=new WeakMap,_=[],y=null,f=!1,d=null,E=null,A=null,T=null,D=null,b=null,N=null,M=new Bt(0,0,0),w=0,z=!1,P=null,V=null,J=null,K=null,B=null,Qt.set(0,0,n.canvas.width,n.canvas.height),C.set(0,0,n.canvas.width,n.canvas.height),r.reset(),a.reset(),o.reset()}return{buffers:{color:r,depth:a,stencil:o},enable:st,disable:Mt,bindFramebuffer:bt,drawBuffers:Rt,useProgram:qt,setBlending:ce,setMaterial:Ht,setFlipSided:pe,setCullFace:se,setLineWidth:be,setPolygonOffset:S,setScissorTest:j,activeTexture:Q,bindTexture:ot,unbindTexture:Y,compressedTexImage2D:vt,compressedTexImage3D:x,texImage2D:I,texImage3D:X,pixelStorei:_t,getParameter:dt,updateUBOMapping:Ft,uniformBlockBinding:Wt,texStorage2D:it,texStorage3D:lt,texSubImage2D:p,texSubImage3D:R,compressedTexSubImage2D:k,compressedTexSubImage3D:Z,scissor:ft,viewport:ht,reset:te}}function o0(n,t,e,i,s,r,a){const o=t.has("WEBGL_multisampled_render_to_texture")?t.get("WEBGL_multisampled_render_to_texture"):null,c=typeof navigator>"u"?!1:/OculusBrowser/g.test(navigator.userAgent),l=new Nt,u=new WeakMap,g=new Set;let h;const m=new WeakMap;let _=!1;try{_=typeof OffscreenCanvas<"u"&&new OffscreenCanvas(1,1).getContext("2d")!==null}catch{}function y(x,p){return _?new OffscreenCanvas(x,p):aa("canvas")}function f(x,p,R){let k=1;const Z=vt(x);if((Z.width>R||Z.height>R)&&(k=R/Math.max(Z.width,Z.height)),k<1)if(typeof HTMLImageElement<"u"&&x instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&x instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&x instanceof ImageBitmap||typeof VideoFrame<"u"&&x instanceof VideoFrame){const it=Math.floor(k*Z.width),lt=Math.floor(k*Z.height);h===void 0&&(h=y(it,lt));const I=p?y(it,lt):h;return I.width=it,I.height=lt,I.getContext("2d").drawImage(x,0,0,it,lt),It("WebGLRenderer: Texture has been resized from ("+Z.width+"x"+Z.height+") to ("+it+"x"+lt+")."),I}else return"data"in x&&It("WebGLRenderer: Image in DataTexture is too big ("+Z.width+"x"+Z.height+")."),x;return x}function d(x){return x.generateMipmaps}function E(x){n.generateMipmap(x)}function A(x){return x.isWebGLCubeRenderTarget?n.TEXTURE_CUBE_MAP:x.isWebGL3DRenderTarget?n.TEXTURE_3D:x.isWebGLArrayRenderTarget||x.isCompressedArrayTexture?n.TEXTURE_2D_ARRAY:n.TEXTURE_2D}function T(x,p,R,k,Z,it=!1){if(x!==null){if(n[x]!==void 0)return n[x];It("WebGLRenderer: Attempt to use non-existing WebGL internal format '"+x+"'")}let lt;k&&(lt=t.get("EXT_texture_norm16"),lt||It("WebGLRenderer: Unable to use normalized textures without EXT_texture_norm16 extension"));let I=p;if(p===n.RED&&(R===n.FLOAT&&(I=n.R32F),R===n.HALF_FLOAT&&(I=n.R16F),R===n.UNSIGNED_BYTE&&(I=n.R8),R===n.UNSIGNED_SHORT&&lt&&(I=lt.R16_EXT),R===n.SHORT&&lt&&(I=lt.R16_SNORM_EXT)),p===n.RED_INTEGER&&(R===n.UNSIGNED_BYTE&&(I=n.R8UI),R===n.UNSIGNED_SHORT&&(I=n.R16UI),R===n.UNSIGNED_INT&&(I=n.R32UI),R===n.BYTE&&(I=n.R8I),R===n.SHORT&&(I=n.R16I),R===n.INT&&(I=n.R32I)),p===n.RG&&(R===n.FLOAT&&(I=n.RG32F),R===n.HALF_FLOAT&&(I=n.RG16F),R===n.UNSIGNED_BYTE&&(I=n.RG8),R===n.UNSIGNED_SHORT&&lt&&(I=lt.RG16_EXT),R===n.SHORT&&lt&&(I=lt.RG16_SNORM_EXT)),p===n.RG_INTEGER&&(R===n.UNSIGNED_BYTE&&(I=n.RG8UI),R===n.UNSIGNED_SHORT&&(I=n.RG16UI),R===n.UNSIGNED_INT&&(I=n.RG32UI),R===n.BYTE&&(I=n.RG8I),R===n.SHORT&&(I=n.RG16I),R===n.INT&&(I=n.RG32I)),p===n.RGB_INTEGER&&(R===n.UNSIGNED_BYTE&&(I=n.RGB8UI),R===n.UNSIGNED_SHORT&&(I=n.RGB16UI),R===n.UNSIGNED_INT&&(I=n.RGB32UI),R===n.BYTE&&(I=n.RGB8I),R===n.SHORT&&(I=n.RGB16I),R===n.INT&&(I=n.RGB32I)),p===n.RGBA_INTEGER&&(R===n.UNSIGNED_BYTE&&(I=n.RGBA8UI),R===n.UNSIGNED_SHORT&&(I=n.RGBA16UI),R===n.UNSIGNED_INT&&(I=n.RGBA32UI),R===n.BYTE&&(I=n.RGBA8I),R===n.SHORT&&(I=n.RGBA16I),R===n.INT&&(I=n.RGBA32I)),p===n.RGB&&(R===n.UNSIGNED_SHORT&&lt&&(I=lt.RGB16_EXT),R===n.SHORT&&lt&&(I=lt.RGB16_SNORM_EXT),R===n.UNSIGNED_INT_5_9_9_9_REV&&(I=n.RGB9_E5),R===n.UNSIGNED_INT_10F_11F_11F_REV&&(I=n.R11F_G11F_B10F)),p===n.RGBA){const X=it?sa:jt.getTransfer(Z);R===n.FLOAT&&(I=n.RGBA32F),R===n.HALF_FLOAT&&(I=n.RGBA16F),R===n.UNSIGNED_BYTE&&(I=X===ie?n.SRGB8_ALPHA8:n.RGBA8),R===n.UNSIGNED_SHORT&&lt&&(I=lt.RGBA16_EXT),R===n.SHORT&&lt&&(I=lt.RGBA16_SNORM_EXT),R===n.UNSIGNED_SHORT_4_4_4_4&&(I=n.RGBA4),R===n.UNSIGNED_SHORT_5_5_5_1&&(I=n.RGB5_A1)}return(I===n.R16F||I===n.R32F||I===n.RG16F||I===n.RG32F||I===n.RGBA16F||I===n.RGBA32F)&&t.get("EXT_color_buffer_float"),I}function D(x,p){let R;return x?p===null||p===Ln||p===Hs?R=n.DEPTH24_STENCIL8:p===An?R=n.DEPTH32F_STENCIL8:p===ks&&(R=n.DEPTH24_STENCIL8,It("DepthTexture: 16 bit depth attachment is not supported with stencil. Using 24-bit attachment.")):p===null||p===Ln||p===Hs?R=n.DEPTH_COMPONENT24:p===An?R=n.DEPTH_COMPONENT32F:p===ks&&(R=n.DEPTH_COMPONENT16),R}function b(x,p){return d(x)===!0||x.isFramebufferTexture&&x.minFilter!==Le&&x.minFilter!==Be?Math.log2(Math.max(p.width,p.height))+1:x.mipmaps!==void 0&&x.mipmaps.length>0?x.mipmaps.length:x.isCompressedTexture&&Array.isArray(x.image)?p.mipmaps.length:1}function N(x){const p=x.target;p.removeEventListener("dispose",N),w(p),p.isVideoTexture&&u.delete(p),p.isHTMLTexture&&g.delete(p)}function M(x){const p=x.target;p.removeEventListener("dispose",M),P(p)}function w(x){const p=i.get(x);if(p.__webglInit===void 0)return;const R=x.source,k=m.get(R);if(k){const Z=k[p.__cacheKey];Z.usedTimes--,Z.usedTimes===0&&z(x),Object.keys(k).length===0&&m.delete(R)}i.remove(x)}function z(x){const p=i.get(x);n.deleteTexture(p.__webglTexture);const R=x.source,k=m.get(R);delete k[p.__cacheKey],a.memory.textures--}function P(x){const p=i.get(x);if(x.depthTexture&&(x.depthTexture.dispose(),i.remove(x.depthTexture)),x.isWebGLCubeRenderTarget)for(let k=0;k<6;k++){if(Array.isArray(p.__webglFramebuffer[k]))for(let Z=0;Z<p.__webglFramebuffer[k].length;Z++)n.deleteFramebuffer(p.__webglFramebuffer[k][Z]);else n.deleteFramebuffer(p.__webglFramebuffer[k]);p.__webglDepthbuffer&&n.deleteRenderbuffer(p.__webglDepthbuffer[k])}else{if(Array.isArray(p.__webglFramebuffer))for(let k=0;k<p.__webglFramebuffer.length;k++)n.deleteFramebuffer(p.__webglFramebuffer[k]);else n.deleteFramebuffer(p.__webglFramebuffer);if(p.__webglDepthbuffer&&n.deleteRenderbuffer(p.__webglDepthbuffer),p.__webglMultisampledFramebuffer&&n.deleteFramebuffer(p.__webglMultisampledFramebuffer),p.__webglColorRenderbuffer)for(let k=0;k<p.__webglColorRenderbuffer.length;k++)p.__webglColorRenderbuffer[k]&&n.deleteRenderbuffer(p.__webglColorRenderbuffer[k]);p.__webglDepthRenderbuffer&&n.deleteRenderbuffer(p.__webglDepthRenderbuffer)}const R=x.textures;for(let k=0,Z=R.length;k<Z;k++){const it=i.get(R[k]);it.__webglTexture&&(n.deleteTexture(it.__webglTexture),a.memory.textures--),i.remove(R[k])}i.remove(x)}let V=0;function J(){V=0}function K(){return V}function B(x){V=x}function W(){const x=V;return x>=s.maxTextures&&It("WebGLTextures: Trying to use "+x+" texture units while this GPU supports only "+s.maxTextures),V+=1,x}function $(x){const p=[];return p.push(x.wrapS),p.push(x.wrapT),p.push(x.wrapR||0),p.push(x.magFilter),p.push(x.minFilter),p.push(x.anisotropy),p.push(x.internalFormat),p.push(x.format),p.push(x.type),p.push(x.generateMipmaps),p.push(x.premultiplyAlpha),p.push(x.flipY),p.push(x.unpackAlignment),p.push(x.colorSpace),p.join()}function et(x,p){const R=i.get(x);if(x.isVideoTexture&&ot(x),x.isRenderTargetTexture===!1&&x.isExternalTexture!==!0&&x.version>0&&R.__version!==x.version){const k=x.image;if(k===null)It("WebGLRenderer: Texture marked for update but no image data found.");else if(k.complete===!1)It("WebGLRenderer: Texture marked for update but image is incomplete");else{Mt(R,x,p);return}}else x.isExternalTexture&&(R.__webglTexture=x.sourceTexture?x.sourceTexture:null);e.bindTexture(n.TEXTURE_2D,R.__webglTexture,n.TEXTURE0+p)}function at(x,p){const R=i.get(x);if(x.isRenderTargetTexture===!1&&x.version>0&&R.__version!==x.version){Mt(R,x,p);return}else x.isExternalTexture&&(R.__webglTexture=x.sourceTexture?x.sourceTexture:null);e.bindTexture(n.TEXTURE_2D_ARRAY,R.__webglTexture,n.TEXTURE0+p)}function ct(x,p){const R=i.get(x);if(x.isRenderTargetTexture===!1&&x.version>0&&R.__version!==x.version){Mt(R,x,p);return}e.bindTexture(n.TEXTURE_3D,R.__webglTexture,n.TEXTURE0+p)}function yt(x,p){const R=i.get(x);if(x.isCubeDepthTexture!==!0&&x.version>0&&R.__version!==x.version){bt(R,x,p);return}e.bindTexture(n.TEXTURE_CUBE_MAP,R.__webglTexture,n.TEXTURE0+p)}const wt={[vo]:n.REPEAT,[kn]:n.CLAMP_TO_EDGE,[Mo]:n.MIRRORED_REPEAT},kt={[Le]:n.NEAREST,[bd]:n.NEAREST_MIPMAP_NEAREST,[er]:n.NEAREST_MIPMAP_LINEAR,[Be]:n.LINEAR,[wa]:n.LINEAR_MIPMAP_NEAREST,[Ti]:n.LINEAR_MIPMAP_LINEAR},Qt={[wd]:n.NEVER,[Dd]:n.ALWAYS,[Rd]:n.LESS,[El]:n.LEQUAL,[Cd]:n.EQUAL,[bl]:n.GEQUAL,[Pd]:n.GREATER,[Ld]:n.NOTEQUAL};function C(x,p){if(p.type===An&&t.has("OES_texture_float_linear")===!1&&(p.magFilter===Be||p.magFilter===wa||p.magFilter===er||p.magFilter===Ti||p.minFilter===Be||p.minFilter===wa||p.minFilter===er||p.minFilter===Ti)&&It("WebGLRenderer: Unable to use linear filtering with floating point textures. OES_texture_float_linear not supported on this device."),n.texParameteri(x,n.TEXTURE_WRAP_S,wt[p.wrapS]),n.texParameteri(x,n.TEXTURE_WRAP_T,wt[p.wrapT]),(x===n.TEXTURE_3D||x===n.TEXTURE_2D_ARRAY)&&n.texParameteri(x,n.TEXTURE_WRAP_R,wt[p.wrapR]),n.texParameteri(x,n.TEXTURE_MAG_FILTER,kt[p.magFilter]),n.texParameteri(x,n.TEXTURE_MIN_FILTER,kt[p.minFilter]),p.compareFunction&&(n.texParameteri(x,n.TEXTURE_COMPARE_MODE,n.COMPARE_REF_TO_TEXTURE),n.texParameteri(x,n.TEXTURE_COMPARE_FUNC,Qt[p.compareFunction])),t.has("EXT_texture_filter_anisotropic")===!0){if(p.magFilter===Le||p.minFilter!==er&&p.minFilter!==Ti||p.type===An&&t.has("OES_texture_float_linear")===!1)return;if(p.anisotropy>1||i.get(p).__currentAnisotropy){const R=t.get("EXT_texture_filter_anisotropic");n.texParameterf(x,R.TEXTURE_MAX_ANISOTROPY_EXT,Math.min(p.anisotropy,s.getMaxAnisotropy())),i.get(p).__currentAnisotropy=p.anisotropy}}}function U(x,p){let R=!1;x.__webglInit===void 0&&(x.__webglInit=!0,p.addEventListener("dispose",N));const k=p.source;let Z=m.get(k);Z===void 0&&(Z={},m.set(k,Z));const it=$(p);if(it!==x.__cacheKey){Z[it]===void 0&&(Z[it]={texture:n.createTexture(),usedTimes:0},a.memory.textures++,R=!0),Z[it].usedTimes++;const lt=Z[x.__cacheKey];lt!==void 0&&(Z[x.__cacheKey].usedTimes--,lt.usedTimes===0&&z(p)),x.__cacheKey=it,x.__webglTexture=Z[it].texture}return R}function nt(x,p,R){return Math.floor(Math.floor(x/R)/p)}function st(x,p,R,k){const it=x.updateRanges;if(it.length===0)e.texSubImage2D(n.TEXTURE_2D,0,0,0,p.width,p.height,R,k,p.data);else{it.sort((_t,ft)=>_t.start-ft.start);let lt=0;for(let _t=1;_t<it.length;_t++){const ft=it[lt],ht=it[_t],Ft=ft.start+ft.count,Wt=nt(ht.start,p.width,4),te=nt(ft.start,p.width,4);ht.start<=Ft+1&&Wt===te&&nt(ht.start+ht.count-1,p.width,4)===Wt?ft.count=Math.max(ft.count,ht.start+ht.count-ft.start):(++lt,it[lt]=ht)}it.length=lt+1;const I=e.getParameter(n.UNPACK_ROW_LENGTH),X=e.getParameter(n.UNPACK_SKIP_PIXELS),dt=e.getParameter(n.UNPACK_SKIP_ROWS);e.pixelStorei(n.UNPACK_ROW_LENGTH,p.width);for(let _t=0,ft=it.length;_t<ft;_t++){const ht=it[_t],Ft=Math.floor(ht.start/4),Wt=Math.ceil(ht.count/4),te=Ft%p.width,L=Math.floor(Ft/p.width),ut=Wt,tt=1;e.pixelStorei(n.UNPACK_SKIP_PIXELS,te),e.pixelStorei(n.UNPACK_SKIP_ROWS,L),e.texSubImage2D(n.TEXTURE_2D,0,te,L,ut,tt,R,k,p.data)}x.clearUpdateRanges(),e.pixelStorei(n.UNPACK_ROW_LENGTH,I),e.pixelStorei(n.UNPACK_SKIP_PIXELS,X),e.pixelStorei(n.UNPACK_SKIP_ROWS,dt)}}function Mt(x,p,R){let k=n.TEXTURE_2D;(p.isDataArrayTexture||p.isCompressedArrayTexture)&&(k=n.TEXTURE_2D_ARRAY),p.isData3DTexture&&(k=n.TEXTURE_3D);const Z=U(x,p),it=p.source;e.bindTexture(k,x.__webglTexture,n.TEXTURE0+R);const lt=i.get(it);if(it.version!==lt.__version||Z===!0){if(e.activeTexture(n.TEXTURE0+R),(typeof ImageBitmap<"u"&&p.image instanceof ImageBitmap)===!1){const tt=jt.getPrimaries(jt.workingColorSpace),St=p.colorSpace===si?null:jt.getPrimaries(p.colorSpace),pt=p.colorSpace===si||tt===St?n.NONE:n.BROWSER_DEFAULT_WEBGL;e.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,p.flipY),e.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,p.premultiplyAlpha),e.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,pt)}e.pixelStorei(n.UNPACK_ALIGNMENT,p.unpackAlignment);let X=f(p.image,!1,s.maxTextureSize);X=Y(p,X);const dt=r.convert(p.format,p.colorSpace),_t=r.convert(p.type);let ft=T(p.internalFormat,dt,_t,p.normalized,p.colorSpace,p.isVideoTexture);C(k,p);let ht;const Ft=p.mipmaps,Wt=p.isVideoTexture!==!0,te=lt.__version===void 0||Z===!0,L=it.dataReady,ut=b(p,X);if(p.isDepthTexture)ft=D(p.format===Ai,p.type),te&&(Wt?e.texStorage2D(n.TEXTURE_2D,1,ft,X.width,X.height):e.texImage2D(n.TEXTURE_2D,0,ft,X.width,X.height,0,dt,_t,null));else if(p.isDataTexture)if(Ft.length>0){Wt&&te&&e.texStorage2D(n.TEXTURE_2D,ut,ft,Ft[0].width,Ft[0].height);for(let tt=0,St=Ft.length;tt<St;tt++)ht=Ft[tt],Wt?L&&e.texSubImage2D(n.TEXTURE_2D,tt,0,0,ht.width,ht.height,dt,_t,ht.data):e.texImage2D(n.TEXTURE_2D,tt,ft,ht.width,ht.height,0,dt,_t,ht.data);p.generateMipmaps=!1}else Wt?(te&&e.texStorage2D(n.TEXTURE_2D,ut,ft,X.width,X.height),L&&st(p,X,dt,_t)):e.texImage2D(n.TEXTURE_2D,0,ft,X.width,X.height,0,dt,_t,X.data);else if(p.isCompressedTexture)if(p.isCompressedArrayTexture){Wt&&te&&e.texStorage3D(n.TEXTURE_2D_ARRAY,ut,ft,Ft[0].width,Ft[0].height,X.depth);for(let tt=0,St=Ft.length;tt<St;tt++)if(ht=Ft[tt],p.format!==pn)if(dt!==null)if(Wt){if(L)if(p.layerUpdates.size>0){const pt=Uc(ht.width,ht.height,p.format,p.type);for(const rt of p.layerUpdates){const Pt=ht.data.subarray(rt*pt/ht.data.BYTES_PER_ELEMENT,(rt+1)*pt/ht.data.BYTES_PER_ELEMENT);e.compressedTexSubImage3D(n.TEXTURE_2D_ARRAY,tt,0,0,rt,ht.width,ht.height,1,dt,Pt)}p.clearLayerUpdates()}else e.compressedTexSubImage3D(n.TEXTURE_2D_ARRAY,tt,0,0,0,ht.width,ht.height,X.depth,dt,ht.data)}else e.compressedTexImage3D(n.TEXTURE_2D_ARRAY,tt,ft,ht.width,ht.height,X.depth,0,ht.data,0,0);else It("WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()");else Wt?L&&e.texSubImage3D(n.TEXTURE_2D_ARRAY,tt,0,0,0,ht.width,ht.height,X.depth,dt,_t,ht.data):e.texImage3D(n.TEXTURE_2D_ARRAY,tt,ft,ht.width,ht.height,X.depth,0,dt,_t,ht.data)}else{Wt&&te&&e.texStorage2D(n.TEXTURE_2D,ut,ft,Ft[0].width,Ft[0].height);for(let tt=0,St=Ft.length;tt<St;tt++)ht=Ft[tt],p.format!==pn?dt!==null?Wt?L&&e.compressedTexSubImage2D(n.TEXTURE_2D,tt,0,0,ht.width,ht.height,dt,ht.data):e.compressedTexImage2D(n.TEXTURE_2D,tt,ft,ht.width,ht.height,0,ht.data):It("WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()"):Wt?L&&e.texSubImage2D(n.TEXTURE_2D,tt,0,0,ht.width,ht.height,dt,_t,ht.data):e.texImage2D(n.TEXTURE_2D,tt,ft,ht.width,ht.height,0,dt,_t,ht.data)}else if(p.isDataArrayTexture)if(Wt){if(te&&e.texStorage3D(n.TEXTURE_2D_ARRAY,ut,ft,X.width,X.height,X.depth),L)if(p.layerUpdates.size>0){const tt=Uc(X.width,X.height,p.format,p.type);for(const St of p.layerUpdates){const pt=X.data.subarray(St*tt/X.data.BYTES_PER_ELEMENT,(St+1)*tt/X.data.BYTES_PER_ELEMENT);e.texSubImage3D(n.TEXTURE_2D_ARRAY,0,0,0,St,X.width,X.height,1,dt,_t,pt)}p.clearLayerUpdates()}else e.texSubImage3D(n.TEXTURE_2D_ARRAY,0,0,0,0,X.width,X.height,X.depth,dt,_t,X.data)}else e.texImage3D(n.TEXTURE_2D_ARRAY,0,ft,X.width,X.height,X.depth,0,dt,_t,X.data);else if(p.isData3DTexture)Wt?(te&&e.texStorage3D(n.TEXTURE_3D,ut,ft,X.width,X.height,X.depth),L&&e.texSubImage3D(n.TEXTURE_3D,0,0,0,0,X.width,X.height,X.depth,dt,_t,X.data)):e.texImage3D(n.TEXTURE_3D,0,ft,X.width,X.height,X.depth,0,dt,_t,X.data);else if(p.isFramebufferTexture){if(te)if(Wt)e.texStorage2D(n.TEXTURE_2D,ut,ft,X.width,X.height);else{let tt=X.width,St=X.height;for(let pt=0;pt<ut;pt++)e.texImage2D(n.TEXTURE_2D,pt,ft,tt,St,0,dt,_t,null),tt>>=1,St>>=1}}else if(p.isHTMLTexture){if("texElementImage2D"in n){const tt=n.canvas;if(tt.hasAttribute("layoutsubtree")||tt.setAttribute("layoutsubtree","true"),X.parentNode!==tt){tt.appendChild(X),g.add(p),tt.onpaint=Gt=>{const xe=Gt.changedElements;for(const re of g)xe.includes(re.image)&&(re.needsUpdate=!0)},tt.requestPaint();return}const St=0,pt=n.RGBA,rt=n.RGBA,Pt=n.UNSIGNED_BYTE;n.texElementImage2D(n.TEXTURE_2D,St,pt,rt,Pt,X),n.texParameteri(n.TEXTURE_2D,n.TEXTURE_MIN_FILTER,n.LINEAR),n.texParameteri(n.TEXTURE_2D,n.TEXTURE_WRAP_S,n.CLAMP_TO_EDGE),n.texParameteri(n.TEXTURE_2D,n.TEXTURE_WRAP_T,n.CLAMP_TO_EDGE)}}else if(Ft.length>0){if(Wt&&te){const tt=vt(Ft[0]);e.texStorage2D(n.TEXTURE_2D,ut,ft,tt.width,tt.height)}for(let tt=0,St=Ft.length;tt<St;tt++)ht=Ft[tt],Wt?L&&e.texSubImage2D(n.TEXTURE_2D,tt,0,0,dt,_t,ht):e.texImage2D(n.TEXTURE_2D,tt,ft,dt,_t,ht);p.generateMipmaps=!1}else if(Wt){if(te){const tt=vt(X);e.texStorage2D(n.TEXTURE_2D,ut,ft,tt.width,tt.height)}L&&e.texSubImage2D(n.TEXTURE_2D,0,0,0,dt,_t,X)}else e.texImage2D(n.TEXTURE_2D,0,ft,dt,_t,X);d(p)&&E(k),lt.__version=it.version,p.onUpdate&&p.onUpdate(p)}x.__version=p.version}function bt(x,p,R){if(p.image.length!==6)return;const k=U(x,p),Z=p.source;e.bindTexture(n.TEXTURE_CUBE_MAP,x.__webglTexture,n.TEXTURE0+R);const it=i.get(Z);if(Z.version!==it.__version||k===!0){e.activeTexture(n.TEXTURE0+R);const lt=jt.getPrimaries(jt.workingColorSpace),I=p.colorSpace===si?null:jt.getPrimaries(p.colorSpace),X=p.colorSpace===si||lt===I?n.NONE:n.BROWSER_DEFAULT_WEBGL;e.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,p.flipY),e.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,p.premultiplyAlpha),e.pixelStorei(n.UNPACK_ALIGNMENT,p.unpackAlignment),e.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,X);const dt=p.isCompressedTexture||p.image[0].isCompressedTexture,_t=p.image[0]&&p.image[0].isDataTexture,ft=[];for(let rt=0;rt<6;rt++)!dt&&!_t?ft[rt]=f(p.image[rt],!0,s.maxCubemapSize):ft[rt]=_t?p.image[rt].image:p.image[rt],ft[rt]=Y(p,ft[rt]);const ht=ft[0],Ft=r.convert(p.format,p.colorSpace),Wt=r.convert(p.type),te=T(p.internalFormat,Ft,Wt,p.normalized,p.colorSpace),L=p.isVideoTexture!==!0,ut=it.__version===void 0||k===!0,tt=Z.dataReady;let St=b(p,ht);C(n.TEXTURE_CUBE_MAP,p);let pt;if(dt){L&&ut&&e.texStorage2D(n.TEXTURE_CUBE_MAP,St,te,ht.width,ht.height);for(let rt=0;rt<6;rt++){pt=ft[rt].mipmaps;for(let Pt=0;Pt<pt.length;Pt++){const Gt=pt[Pt];p.format!==pn?Ft!==null?L?tt&&e.compressedTexSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,Pt,0,0,Gt.width,Gt.height,Ft,Gt.data):e.compressedTexImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,Pt,te,Gt.width,Gt.height,0,Gt.data):It("WebGLRenderer: Attempt to load unsupported compressed texture format in .setTextureCube()"):L?tt&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,Pt,0,0,Gt.width,Gt.height,Ft,Wt,Gt.data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,Pt,te,Gt.width,Gt.height,0,Ft,Wt,Gt.data)}}}else{if(pt=p.mipmaps,L&&ut){pt.length>0&&St++;const rt=vt(ft[0]);e.texStorage2D(n.TEXTURE_CUBE_MAP,St,te,rt.width,rt.height)}for(let rt=0;rt<6;rt++)if(_t){L?tt&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,0,0,0,ft[rt].width,ft[rt].height,Ft,Wt,ft[rt].data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,0,te,ft[rt].width,ft[rt].height,0,Ft,Wt,ft[rt].data);for(let Pt=0;Pt<pt.length;Pt++){const xe=pt[Pt].image[rt].image;L?tt&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,Pt+1,0,0,xe.width,xe.height,Ft,Wt,xe.data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,Pt+1,te,xe.width,xe.height,0,Ft,Wt,xe.data)}}else{L?tt&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,0,0,0,Ft,Wt,ft[rt]):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,0,te,Ft,Wt,ft[rt]);for(let Pt=0;Pt<pt.length;Pt++){const Gt=pt[Pt];L?tt&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,Pt+1,0,0,Ft,Wt,Gt.image[rt]):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+rt,Pt+1,te,Ft,Wt,Gt.image[rt])}}}d(p)&&E(n.TEXTURE_CUBE_MAP),it.__version=Z.version,p.onUpdate&&p.onUpdate(p)}x.__version=p.version}function Rt(x,p,R,k,Z,it){const lt=r.convert(R.format,R.colorSpace),I=r.convert(R.type),X=T(R.internalFormat,lt,I,R.normalized,R.colorSpace),dt=i.get(p),_t=i.get(R);if(_t.__renderTarget=p,!dt.__hasExternalTextures){const ft=Math.max(1,p.width>>it),ht=Math.max(1,p.height>>it);Z===n.TEXTURE_3D||Z===n.TEXTURE_2D_ARRAY?e.texImage3D(Z,it,X,ft,ht,p.depth,0,lt,I,null):e.texImage2D(Z,it,X,ft,ht,0,lt,I,null)}e.bindFramebuffer(n.FRAMEBUFFER,x),Q(p)?o.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,k,Z,_t.__webglTexture,0,j(p)):(Z===n.TEXTURE_2D||Z>=n.TEXTURE_CUBE_MAP_POSITIVE_X&&Z<=n.TEXTURE_CUBE_MAP_NEGATIVE_Z)&&n.framebufferTexture2D(n.FRAMEBUFFER,k,Z,_t.__webglTexture,it),e.bindFramebuffer(n.FRAMEBUFFER,null)}function qt(x,p,R){if(n.bindRenderbuffer(n.RENDERBUFFER,x),p.depthBuffer){const k=p.depthTexture,Z=k&&k.isDepthTexture?k.type:null,it=D(p.stencilBuffer,Z),lt=p.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT;Q(p)?o.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,j(p),it,p.width,p.height):R?n.renderbufferStorageMultisample(n.RENDERBUFFER,j(p),it,p.width,p.height):n.renderbufferStorage(n.RENDERBUFFER,it,p.width,p.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,lt,n.RENDERBUFFER,x)}else{const k=p.textures;for(let Z=0;Z<k.length;Z++){const it=k[Z],lt=r.convert(it.format,it.colorSpace),I=r.convert(it.type),X=T(it.internalFormat,lt,I,it.normalized,it.colorSpace);Q(p)?o.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,j(p),X,p.width,p.height):R?n.renderbufferStorageMultisample(n.RENDERBUFFER,j(p),X,p.width,p.height):n.renderbufferStorage(n.RENDERBUFFER,X,p.width,p.height)}}n.bindRenderbuffer(n.RENDERBUFFER,null)}function Ut(x,p,R){const k=p.isWebGLCubeRenderTarget===!0;if(e.bindFramebuffer(n.FRAMEBUFFER,x),!(p.depthTexture&&p.depthTexture.isDepthTexture))throw new Error("renderTarget.depthTexture must be an instance of THREE.DepthTexture");const Z=i.get(p.depthTexture);if(Z.__renderTarget=p,(!Z.__webglTexture||p.depthTexture.image.width!==p.width||p.depthTexture.image.height!==p.height)&&(p.depthTexture.image.width=p.width,p.depthTexture.image.height=p.height,p.depthTexture.needsUpdate=!0),k){if(Z.__webglInit===void 0&&(Z.__webglInit=!0,p.depthTexture.addEventListener("dispose",N)),Z.__webglTexture===void 0){Z.__webglTexture=n.createTexture(),e.bindTexture(n.TEXTURE_CUBE_MAP,Z.__webglTexture),C(n.TEXTURE_CUBE_MAP,p.depthTexture);const dt=r.convert(p.depthTexture.format),_t=r.convert(p.depthTexture.type);let ft;p.depthTexture.format===Wn?ft=n.DEPTH_COMPONENT24:p.depthTexture.format===Ai&&(ft=n.DEPTH24_STENCIL8);for(let ht=0;ht<6;ht++)n.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ht,0,ft,p.width,p.height,0,dt,_t,null)}}else et(p.depthTexture,0);const it=Z.__webglTexture,lt=j(p),I=k?n.TEXTURE_CUBE_MAP_POSITIVE_X+R:n.TEXTURE_2D,X=p.depthTexture.format===Ai?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT;if(p.depthTexture.format===Wn)Q(p)?o.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,X,I,it,0,lt):n.framebufferTexture2D(n.FRAMEBUFFER,X,I,it,0);else if(p.depthTexture.format===Ai)Q(p)?o.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,X,I,it,0,lt):n.framebufferTexture2D(n.FRAMEBUFFER,X,I,it,0);else throw new Error("Unknown depthTexture format")}function ee(x){const p=i.get(x),R=x.isWebGLCubeRenderTarget===!0;if(p.__boundDepthTexture!==x.depthTexture){const k=x.depthTexture;if(p.__depthDisposeCallback&&p.__depthDisposeCallback(),k){const Z=()=>{delete p.__boundDepthTexture,delete p.__depthDisposeCallback,k.removeEventListener("dispose",Z)};k.addEventListener("dispose",Z),p.__depthDisposeCallback=Z}p.__boundDepthTexture=k}if(x.depthTexture&&!p.__autoAllocateDepthBuffer)if(R)for(let k=0;k<6;k++)Ut(p.__webglFramebuffer[k],x,k);else{const k=x.texture.mipmaps;k&&k.length>0?Ut(p.__webglFramebuffer[0],x,0):Ut(p.__webglFramebuffer,x,0)}else if(R){p.__webglDepthbuffer=[];for(let k=0;k<6;k++)if(e.bindFramebuffer(n.FRAMEBUFFER,p.__webglFramebuffer[k]),p.__webglDepthbuffer[k]===void 0)p.__webglDepthbuffer[k]=n.createRenderbuffer(),qt(p.__webglDepthbuffer[k],x,!1);else{const Z=x.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,it=p.__webglDepthbuffer[k];n.bindRenderbuffer(n.RENDERBUFFER,it),n.framebufferRenderbuffer(n.FRAMEBUFFER,Z,n.RENDERBUFFER,it)}}else{const k=x.texture.mipmaps;if(k&&k.length>0?e.bindFramebuffer(n.FRAMEBUFFER,p.__webglFramebuffer[0]):e.bindFramebuffer(n.FRAMEBUFFER,p.__webglFramebuffer),p.__webglDepthbuffer===void 0)p.__webglDepthbuffer=n.createRenderbuffer(),qt(p.__webglDepthbuffer,x,!1);else{const Z=x.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,it=p.__webglDepthbuffer;n.bindRenderbuffer(n.RENDERBUFFER,it),n.framebufferRenderbuffer(n.FRAMEBUFFER,Z,n.RENDERBUFFER,it)}}e.bindFramebuffer(n.FRAMEBUFFER,null)}function ce(x,p,R){const k=i.get(x);p!==void 0&&Rt(k.__webglFramebuffer,x,x.texture,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,0),R!==void 0&&ee(x)}function Ht(x){const p=x.texture,R=i.get(x),k=i.get(p);x.addEventListener("dispose",M);const Z=x.textures,it=x.isWebGLCubeRenderTarget===!0,lt=Z.length>1;if(lt||(k.__webglTexture===void 0&&(k.__webglTexture=n.createTexture()),k.__version=p.version,a.memory.textures++),it){R.__webglFramebuffer=[];for(let I=0;I<6;I++)if(p.mipmaps&&p.mipmaps.length>0){R.__webglFramebuffer[I]=[];for(let X=0;X<p.mipmaps.length;X++)R.__webglFramebuffer[I][X]=n.createFramebuffer()}else R.__webglFramebuffer[I]=n.createFramebuffer()}else{if(p.mipmaps&&p.mipmaps.length>0){R.__webglFramebuffer=[];for(let I=0;I<p.mipmaps.length;I++)R.__webglFramebuffer[I]=n.createFramebuffer()}else R.__webglFramebuffer=n.createFramebuffer();if(lt)for(let I=0,X=Z.length;I<X;I++){const dt=i.get(Z[I]);dt.__webglTexture===void 0&&(dt.__webglTexture=n.createTexture(),a.memory.textures++)}if(x.samples>0&&Q(x)===!1){R.__webglMultisampledFramebuffer=n.createFramebuffer(),R.__webglColorRenderbuffer=[],e.bindFramebuffer(n.FRAMEBUFFER,R.__webglMultisampledFramebuffer);for(let I=0;I<Z.length;I++){const X=Z[I];R.__webglColorRenderbuffer[I]=n.createRenderbuffer(),n.bindRenderbuffer(n.RENDERBUFFER,R.__webglColorRenderbuffer[I]);const dt=r.convert(X.format,X.colorSpace),_t=r.convert(X.type),ft=T(X.internalFormat,dt,_t,X.normalized,X.colorSpace,x.isXRRenderTarget===!0),ht=j(x);n.renderbufferStorageMultisample(n.RENDERBUFFER,ht,ft,x.width,x.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+I,n.RENDERBUFFER,R.__webglColorRenderbuffer[I])}n.bindRenderbuffer(n.RENDERBUFFER,null),x.depthBuffer&&(R.__webglDepthRenderbuffer=n.createRenderbuffer(),qt(R.__webglDepthRenderbuffer,x,!0)),e.bindFramebuffer(n.FRAMEBUFFER,null)}}if(it){e.bindTexture(n.TEXTURE_CUBE_MAP,k.__webglTexture),C(n.TEXTURE_CUBE_MAP,p);for(let I=0;I<6;I++)if(p.mipmaps&&p.mipmaps.length>0)for(let X=0;X<p.mipmaps.length;X++)Rt(R.__webglFramebuffer[I][X],x,p,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+I,X);else Rt(R.__webglFramebuffer[I],x,p,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+I,0);d(p)&&E(n.TEXTURE_CUBE_MAP),e.unbindTexture()}else if(lt){for(let I=0,X=Z.length;I<X;I++){const dt=Z[I],_t=i.get(dt);let ft=n.TEXTURE_2D;(x.isWebGL3DRenderTarget||x.isWebGLArrayRenderTarget)&&(ft=x.isWebGL3DRenderTarget?n.TEXTURE_3D:n.TEXTURE_2D_ARRAY),e.bindTexture(ft,_t.__webglTexture),C(ft,dt),Rt(R.__webglFramebuffer,x,dt,n.COLOR_ATTACHMENT0+I,ft,0),d(dt)&&E(ft)}e.unbindTexture()}else{let I=n.TEXTURE_2D;if((x.isWebGL3DRenderTarget||x.isWebGLArrayRenderTarget)&&(I=x.isWebGL3DRenderTarget?n.TEXTURE_3D:n.TEXTURE_2D_ARRAY),e.bindTexture(I,k.__webglTexture),C(I,p),p.mipmaps&&p.mipmaps.length>0)for(let X=0;X<p.mipmaps.length;X++)Rt(R.__webglFramebuffer[X],x,p,n.COLOR_ATTACHMENT0,I,X);else Rt(R.__webglFramebuffer,x,p,n.COLOR_ATTACHMENT0,I,0);d(p)&&E(I),e.unbindTexture()}x.depthBuffer&&ee(x)}function pe(x){const p=x.textures;for(let R=0,k=p.length;R<k;R++){const Z=p[R];if(d(Z)){const it=A(x),lt=i.get(Z).__webglTexture;e.bindTexture(it,lt),E(it),e.unbindTexture()}}}const se=[],be=[];function S(x){if(x.samples>0){if(Q(x)===!1){const p=x.textures,R=x.width,k=x.height;let Z=n.COLOR_BUFFER_BIT;const it=x.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,lt=i.get(x),I=p.length>1;if(I)for(let dt=0;dt<p.length;dt++)e.bindFramebuffer(n.FRAMEBUFFER,lt.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+dt,n.RENDERBUFFER,null),e.bindFramebuffer(n.FRAMEBUFFER,lt.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+dt,n.TEXTURE_2D,null,0);e.bindFramebuffer(n.READ_FRAMEBUFFER,lt.__webglMultisampledFramebuffer);const X=x.texture.mipmaps;X&&X.length>0?e.bindFramebuffer(n.DRAW_FRAMEBUFFER,lt.__webglFramebuffer[0]):e.bindFramebuffer(n.DRAW_FRAMEBUFFER,lt.__webglFramebuffer);for(let dt=0;dt<p.length;dt++){if(x.resolveDepthBuffer&&(x.depthBuffer&&(Z|=n.DEPTH_BUFFER_BIT),x.stencilBuffer&&x.resolveStencilBuffer&&(Z|=n.STENCIL_BUFFER_BIT)),I){n.framebufferRenderbuffer(n.READ_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.RENDERBUFFER,lt.__webglColorRenderbuffer[dt]);const _t=i.get(p[dt]).__webglTexture;n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,_t,0)}n.blitFramebuffer(0,0,R,k,0,0,R,k,Z,n.NEAREST),c===!0&&(se.length=0,be.length=0,se.push(n.COLOR_ATTACHMENT0+dt),x.depthBuffer&&x.resolveDepthBuffer===!1&&(se.push(it),be.push(it),n.invalidateFramebuffer(n.DRAW_FRAMEBUFFER,be)),n.invalidateFramebuffer(n.READ_FRAMEBUFFER,se))}if(e.bindFramebuffer(n.READ_FRAMEBUFFER,null),e.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),I)for(let dt=0;dt<p.length;dt++){e.bindFramebuffer(n.FRAMEBUFFER,lt.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+dt,n.RENDERBUFFER,lt.__webglColorRenderbuffer[dt]);const _t=i.get(p[dt]).__webglTexture;e.bindFramebuffer(n.FRAMEBUFFER,lt.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+dt,n.TEXTURE_2D,_t,0)}e.bindFramebuffer(n.DRAW_FRAMEBUFFER,lt.__webglMultisampledFramebuffer)}else if(x.depthBuffer&&x.resolveDepthBuffer===!1&&c){const p=x.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT;n.invalidateFramebuffer(n.DRAW_FRAMEBUFFER,[p])}}}function j(x){return Math.min(s.maxSamples,x.samples)}function Q(x){const p=i.get(x);return x.samples>0&&t.has("WEBGL_multisampled_render_to_texture")===!0&&p.__useRenderToTexture!==!1}function ot(x){const p=a.render.frame;u.get(x)!==p&&(u.set(x,p),x.update())}function Y(x,p){const R=x.colorSpace,k=x.format,Z=x.type;return x.isCompressedTexture===!0||x.isVideoTexture===!0||R!==ia&&R!==si&&(jt.getTransfer(R)===ie?(k!==pn||Z!==an)&&It("WebGLTextures: sRGB encoded textures have to use RGBAFormat and UnsignedByteType."):Kt("WebGLTextures: Unsupported texture color space:",R)),p}function vt(x){return typeof HTMLImageElement<"u"&&x instanceof HTMLImageElement?(l.width=x.naturalWidth||x.width,l.height=x.naturalHeight||x.height):typeof VideoFrame<"u"&&x instanceof VideoFrame?(l.width=x.displayWidth,l.height=x.displayHeight):(l.width=x.width,l.height=x.height),l}this.allocateTextureUnit=W,this.resetTextureUnits=J,this.getTextureUnits=K,this.setTextureUnits=B,this.setTexture2D=et,this.setTexture2DArray=at,this.setTexture3D=ct,this.setTextureCube=yt,this.rebindTextures=ce,this.setupRenderTarget=Ht,this.updateRenderTargetMipmap=pe,this.updateMultisampleRenderTarget=S,this.setupDepthRenderbuffer=ee,this.setupFrameBufferTexture=Rt,this.useMultisampledRTT=Q,this.isReversedDepthBuffer=function(){return e.buffers.depth.getReversed()}}function l0(n,t){function e(i,s=si){let r;const a=jt.getTransfer(s);if(i===an)return n.UNSIGNED_BYTE;if(i===xl)return n.UNSIGNED_SHORT_4_4_4_4;if(i===vl)return n.UNSIGNED_SHORT_5_5_5_1;if(i===Ch)return n.UNSIGNED_INT_5_9_9_9_REV;if(i===Ph)return n.UNSIGNED_INT_10F_11F_11F_REV;if(i===wh)return n.BYTE;if(i===Rh)return n.SHORT;if(i===ks)return n.UNSIGNED_SHORT;if(i===_l)return n.INT;if(i===Ln)return n.UNSIGNED_INT;if(i===An)return n.FLOAT;if(i===Vn)return n.HALF_FLOAT;if(i===Lh)return n.ALPHA;if(i===Dh)return n.RGB;if(i===pn)return n.RGBA;if(i===Wn)return n.DEPTH_COMPONENT;if(i===Ai)return n.DEPTH_STENCIL;if(i===Ih)return n.RED;if(i===Ml)return n.RED_INTEGER;if(i===Li)return n.RG;if(i===Sl)return n.RG_INTEGER;if(i===yl)return n.RGBA_INTEGER;if(i===kr||i===Hr||i===Gr||i===Vr)if(a===ie)if(r=t.get("WEBGL_compressed_texture_s3tc_srgb"),r!==null){if(i===kr)return r.COMPRESSED_SRGB_S3TC_DXT1_EXT;if(i===Hr)return r.COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT;if(i===Gr)return r.COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT;if(i===Vr)return r.COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT}else return null;else if(r=t.get("WEBGL_compressed_texture_s3tc"),r!==null){if(i===kr)return r.COMPRESSED_RGB_S3TC_DXT1_EXT;if(i===Hr)return r.COMPRESSED_RGBA_S3TC_DXT1_EXT;if(i===Gr)return r.COMPRESSED_RGBA_S3TC_DXT3_EXT;if(i===Vr)return r.COMPRESSED_RGBA_S3TC_DXT5_EXT}else return null;if(i===So||i===yo||i===Eo||i===bo)if(r=t.get("WEBGL_compressed_texture_pvrtc"),r!==null){if(i===So)return r.COMPRESSED_RGB_PVRTC_4BPPV1_IMG;if(i===yo)return r.COMPRESSED_RGB_PVRTC_2BPPV1_IMG;if(i===Eo)return r.COMPRESSED_RGBA_PVRTC_4BPPV1_IMG;if(i===bo)return r.COMPRESSED_RGBA_PVRTC_2BPPV1_IMG}else return null;if(i===To||i===Ao||i===wo||i===Ro||i===Co||i===ea||i===Po)if(r=t.get("WEBGL_compressed_texture_etc"),r!==null){if(i===To||i===Ao)return a===ie?r.COMPRESSED_SRGB8_ETC2:r.COMPRESSED_RGB8_ETC2;if(i===wo)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ETC2_EAC:r.COMPRESSED_RGBA8_ETC2_EAC;if(i===Ro)return r.COMPRESSED_R11_EAC;if(i===Co)return r.COMPRESSED_SIGNED_R11_EAC;if(i===ea)return r.COMPRESSED_RG11_EAC;if(i===Po)return r.COMPRESSED_SIGNED_RG11_EAC}else return null;if(i===Lo||i===Do||i===Io||i===Uo||i===No||i===Fo||i===Oo||i===Bo||i===zo||i===ko||i===Ho||i===Go||i===Vo||i===Wo)if(r=t.get("WEBGL_compressed_texture_astc"),r!==null){if(i===Lo)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR:r.COMPRESSED_RGBA_ASTC_4x4_KHR;if(i===Do)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR:r.COMPRESSED_RGBA_ASTC_5x4_KHR;if(i===Io)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR:r.COMPRESSED_RGBA_ASTC_5x5_KHR;if(i===Uo)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR:r.COMPRESSED_RGBA_ASTC_6x5_KHR;if(i===No)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR:r.COMPRESSED_RGBA_ASTC_6x6_KHR;if(i===Fo)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR:r.COMPRESSED_RGBA_ASTC_8x5_KHR;if(i===Oo)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR:r.COMPRESSED_RGBA_ASTC_8x6_KHR;if(i===Bo)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR:r.COMPRESSED_RGBA_ASTC_8x8_KHR;if(i===zo)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR:r.COMPRESSED_RGBA_ASTC_10x5_KHR;if(i===ko)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR:r.COMPRESSED_RGBA_ASTC_10x6_KHR;if(i===Ho)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR:r.COMPRESSED_RGBA_ASTC_10x8_KHR;if(i===Go)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR:r.COMPRESSED_RGBA_ASTC_10x10_KHR;if(i===Vo)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR:r.COMPRESSED_RGBA_ASTC_12x10_KHR;if(i===Wo)return a===ie?r.COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR:r.COMPRESSED_RGBA_ASTC_12x12_KHR}else return null;if(i===Xo||i===$o||i===qo)if(r=t.get("EXT_texture_compression_bptc"),r!==null){if(i===Xo)return a===ie?r.COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT:r.COMPRESSED_RGBA_BPTC_UNORM_EXT;if(i===$o)return r.COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT;if(i===qo)return r.COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT}else return null;if(i===Yo||i===jo||i===na||i===Ko)if(r=t.get("EXT_texture_compression_rgtc"),r!==null){if(i===Yo)return r.COMPRESSED_RED_RGTC1_EXT;if(i===jo)return r.COMPRESSED_SIGNED_RED_RGTC1_EXT;if(i===na)return r.COMPRESSED_RED_GREEN_RGTC2_EXT;if(i===Ko)return r.COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT}else return null;return i===Hs?n.UNSIGNED_INT_24_8:n[i]!==void 0?n[i]:null}return{convert:e}}const c0=`
void main() {

	gl_Position = vec4( position, 1.0 );

}`,h0=`
uniform sampler2DArray depthColor;
uniform float depthWidth;
uniform float depthHeight;

void main() {

	vec2 coord = vec2( gl_FragCoord.x / depthWidth, gl_FragCoord.y / depthHeight );

	if ( coord.x >= 1.0 ) {

		gl_FragDepth = texture( depthColor, vec3( coord.x - 1.0, coord.y, 1 ) ).r;

	} else {

		gl_FragDepth = texture( depthColor, vec3( coord.x, coord.y, 0 ) ).r;

	}

}`;class u0{constructor(){this.texture=null,this.mesh=null,this.depthNear=0,this.depthFar=0}init(t,e){if(this.texture===null){const i=new qh(t.texture);(t.depthNear!==e.depthNear||t.depthFar!==e.depthFar)&&(this.depthNear=t.depthNear,this.depthFar=t.depthFar),this.texture=i}}getMesh(t){if(this.texture!==null&&this.mesh===null){const e=t.cameras[0].viewport,i=new gn({vertexShader:c0,fragmentShader:h0,uniforms:{depthColor:{value:this.texture},depthWidth:{value:e.z},depthHeight:{value:e.w}}});this.mesh=new Xn(new xa(20,20),i)}return this.mesh}reset(){this.texture=null,this.mesh=null}getDepthTexture(){return this.texture}}class d0 extends fi{constructor(t,e){super();const i=this;let s=null,r=1,a=null,o="local-floor",c=1,l=null,u=null,g=null,h=null,m=null,_=null;const y=typeof XRWebGLBinding<"u",f=new u0,d={},E=e.getContextAttributes();let A=null,T=null;const D=[],b=[],N=new Nt;let M=null;const w=new rn;w.viewport=new ye;const z=new rn;z.viewport=new ye;const P=[w,z],V=new Sf;let J=null,K=null;this.cameraAutoUpdate=!0,this.enabled=!1,this.isPresenting=!1,this.getController=function(U){let nt=D[U];return nt===void 0&&(nt=new Ua,D[U]=nt),nt.getTargetRaySpace()},this.getControllerGrip=function(U){let nt=D[U];return nt===void 0&&(nt=new Ua,D[U]=nt),nt.getGripSpace()},this.getHand=function(U){let nt=D[U];return nt===void 0&&(nt=new Ua,D[U]=nt),nt.getHandSpace()};function B(U){const nt=b.indexOf(U.inputSource);if(nt===-1)return;const st=D[nt];st!==void 0&&(st.update(U.inputSource,U.frame,l||a),st.dispatchEvent({type:U.type,data:U.inputSource}))}function W(){s.removeEventListener("select",B),s.removeEventListener("selectstart",B),s.removeEventListener("selectend",B),s.removeEventListener("squeeze",B),s.removeEventListener("squeezestart",B),s.removeEventListener("squeezeend",B),s.removeEventListener("end",W),s.removeEventListener("inputsourceschange",$);for(let U=0;U<D.length;U++){const nt=b[U];nt!==null&&(b[U]=null,D[U].disconnect(nt))}J=null,K=null,f.reset();for(const U in d)delete d[U];t.setRenderTarget(A),m=null,h=null,g=null,s=null,T=null,C.stop(),i.isPresenting=!1,t.setPixelRatio(M),t.setSize(N.width,N.height,!1),i.dispatchEvent({type:"sessionend"})}this.setFramebufferScaleFactor=function(U){r=U,i.isPresenting===!0&&It("WebXRManager: Cannot change framebuffer scale while presenting.")},this.setReferenceSpaceType=function(U){o=U,i.isPresenting===!0&&It("WebXRManager: Cannot change reference space type while presenting.")},this.getReferenceSpace=function(){return l||a},this.setReferenceSpace=function(U){l=U},this.getBaseLayer=function(){return h!==null?h:m},this.getBinding=function(){return g===null&&y&&(g=new XRWebGLBinding(s,e)),g},this.getFrame=function(){return _},this.getSession=function(){return s},this.setSession=async function(U){if(s=U,s!==null){if(A=t.getRenderTarget(),s.addEventListener("select",B),s.addEventListener("selectstart",B),s.addEventListener("selectend",B),s.addEventListener("squeeze",B),s.addEventListener("squeezestart",B),s.addEventListener("squeezeend",B),s.addEventListener("end",W),s.addEventListener("inputsourceschange",$),E.xrCompatible!==!0&&await e.makeXRCompatible(),M=t.getPixelRatio(),t.getSize(N),y&&"createProjectionLayer"in XRWebGLBinding.prototype){let st=null,Mt=null,bt=null;E.depth&&(bt=E.stencil?e.DEPTH24_STENCIL8:e.DEPTH_COMPONENT24,st=E.stencil?Ai:Wn,Mt=E.stencil?Hs:Ln);const Rt={colorFormat:e.RGBA8,depthFormat:bt,scaleFactor:r};g=this.getBinding(),h=g.createProjectionLayer(Rt),s.updateRenderState({layers:[h]}),t.setPixelRatio(1),t.setSize(h.textureWidth,h.textureHeight,!1),T=new Pn(h.textureWidth,h.textureHeight,{format:pn,type:an,depthTexture:new gs(h.textureWidth,h.textureHeight,Mt,void 0,void 0,void 0,void 0,void 0,void 0,st),stencilBuffer:E.stencil,colorSpace:t.outputColorSpace,samples:E.antialias?4:0,resolveDepthBuffer:h.ignoreDepthValues===!1,resolveStencilBuffer:h.ignoreDepthValues===!1})}else{const st={antialias:E.antialias,alpha:!0,depth:E.depth,stencil:E.stencil,framebufferScaleFactor:r};m=new XRWebGLLayer(s,e,st),s.updateRenderState({baseLayer:m}),t.setPixelRatio(1),t.setSize(m.framebufferWidth,m.framebufferHeight,!1),T=new Pn(m.framebufferWidth,m.framebufferHeight,{format:pn,type:an,colorSpace:t.outputColorSpace,stencilBuffer:E.stencil,resolveDepthBuffer:m.ignoreDepthValues===!1,resolveStencilBuffer:m.ignoreDepthValues===!1})}T.isXRRenderTarget=!0,this.setFoveation(c),l=null,a=await s.requestReferenceSpace(o),C.setContext(s),C.start(),i.isPresenting=!0,i.dispatchEvent({type:"sessionstart"})}},this.getEnvironmentBlendMode=function(){if(s!==null)return s.environmentBlendMode},this.getDepthTexture=function(){return f.getDepthTexture()};function $(U){for(let nt=0;nt<U.removed.length;nt++){const st=U.removed[nt],Mt=b.indexOf(st);Mt>=0&&(b[Mt]=null,D[Mt].disconnect(st))}for(let nt=0;nt<U.added.length;nt++){const st=U.added[nt];let Mt=b.indexOf(st);if(Mt===-1){for(let Rt=0;Rt<D.length;Rt++)if(Rt>=b.length){b.push(st),Mt=Rt;break}else if(b[Rt]===null){b[Rt]=st,Mt=Rt;break}if(Mt===-1)break}const bt=D[Mt];bt&&bt.connect(st)}}const et=new F,at=new F;function ct(U,nt,st){et.setFromMatrixPosition(nt.matrixWorld),at.setFromMatrixPosition(st.matrixWorld);const Mt=et.distanceTo(at),bt=nt.projectionMatrix.elements,Rt=st.projectionMatrix.elements,qt=bt[14]/(bt[10]-1),Ut=bt[14]/(bt[10]+1),ee=(bt[9]+1)/bt[5],ce=(bt[9]-1)/bt[5],Ht=(bt[8]-1)/bt[0],pe=(Rt[8]+1)/Rt[0],se=qt*Ht,be=qt*pe,S=Mt/(-Ht+pe),j=S*-Ht;if(nt.matrixWorld.decompose(U.position,U.quaternion,U.scale),U.translateX(j),U.translateZ(S),U.matrixWorld.compose(U.position,U.quaternion,U.scale),U.matrixWorldInverse.copy(U.matrixWorld).invert(),bt[10]===-1)U.projectionMatrix.copy(nt.projectionMatrix),U.projectionMatrixInverse.copy(nt.projectionMatrixInverse);else{const Q=qt+S,ot=Ut+S,Y=se-j,vt=be+(Mt-j),x=ee*Ut/ot*Q,p=ce*Ut/ot*Q;U.projectionMatrix.makePerspective(Y,vt,x,p,Q,ot),U.projectionMatrixInverse.copy(U.projectionMatrix).invert()}}function yt(U,nt){nt===null?U.matrixWorld.copy(U.matrix):U.matrixWorld.multiplyMatrices(nt.matrixWorld,U.matrix),U.matrixWorldInverse.copy(U.matrixWorld).invert()}this.updateCamera=function(U){if(s===null)return;let nt=U.near,st=U.far;f.texture!==null&&(f.depthNear>0&&(nt=f.depthNear),f.depthFar>0&&(st=f.depthFar)),V.near=z.near=w.near=nt,V.far=z.far=w.far=st,(J!==V.near||K!==V.far)&&(s.updateRenderState({depthNear:V.near,depthFar:V.far}),J=V.near,K=V.far),V.layers.mask=U.layers.mask|6,w.layers.mask=V.layers.mask&-5,z.layers.mask=V.layers.mask&-3;const Mt=U.parent,bt=V.cameras;yt(V,Mt);for(let Rt=0;Rt<bt.length;Rt++)yt(bt[Rt],Mt);bt.length===2?ct(V,w,z):V.projectionMatrix.copy(w.projectionMatrix),wt(U,V,Mt)};function wt(U,nt,st){st===null?U.matrix.copy(nt.matrixWorld):(U.matrix.copy(st.matrixWorld),U.matrix.invert(),U.matrix.multiply(nt.matrixWorld)),U.matrix.decompose(U.position,U.quaternion,U.scale),U.updateMatrixWorld(!0),U.projectionMatrix.copy(nt.projectionMatrix),U.projectionMatrixInverse.copy(nt.projectionMatrixInverse),U.isPerspectiveCamera&&(U.fov=Qo*2*Math.atan(1/U.projectionMatrix.elements[5]),U.zoom=1)}this.getCamera=function(){return V},this.getFoveation=function(){if(!(h===null&&m===null))return c},this.setFoveation=function(U){c=U,h!==null&&(h.fixedFoveation=U),m!==null&&m.fixedFoveation!==void 0&&(m.fixedFoveation=U)},this.hasDepthSensing=function(){return f.texture!==null},this.getDepthSensingMesh=function(){return f.getMesh(V)},this.getCameraTexture=function(U){return d[U]};let kt=null;function Qt(U,nt){if(u=nt.getViewerPose(l||a),_=nt,u!==null){const st=u.views;m!==null&&(t.setRenderTargetFramebuffer(T,m.framebuffer),t.setRenderTarget(T));let Mt=!1;st.length!==V.cameras.length&&(V.cameras.length=0,Mt=!0);for(let Ut=0;Ut<st.length;Ut++){const ee=st[Ut];let ce=null;if(m!==null)ce=m.getViewport(ee);else{const pe=g.getViewSubImage(h,ee);ce=pe.viewport,Ut===0&&(t.setRenderTargetTextures(T,pe.colorTexture,pe.depthStencilTexture),t.setRenderTarget(T))}let Ht=P[Ut];Ht===void 0&&(Ht=new rn,Ht.layers.enable(Ut),Ht.viewport=new ye,P[Ut]=Ht),Ht.matrix.fromArray(ee.transform.matrix),Ht.matrix.decompose(Ht.position,Ht.quaternion,Ht.scale),Ht.projectionMatrix.fromArray(ee.projectionMatrix),Ht.projectionMatrixInverse.copy(Ht.projectionMatrix).invert(),Ht.viewport.set(ce.x,ce.y,ce.width,ce.height),Ut===0&&(V.matrix.copy(Ht.matrix),V.matrix.decompose(V.position,V.quaternion,V.scale)),Mt===!0&&V.cameras.push(Ht)}const bt=s.enabledFeatures;if(bt&&bt.includes("depth-sensing")&&s.depthUsage=="gpu-optimized"&&y){g=i.getBinding();const Ut=g.getDepthInformation(st[0]);Ut&&Ut.isValid&&Ut.texture&&f.init(Ut,s.renderState)}if(bt&&bt.includes("camera-access")&&y){t.state.unbindTexture(),g=i.getBinding();for(let Ut=0;Ut<st.length;Ut++){const ee=st[Ut].camera;if(ee){let ce=d[ee];ce||(ce=new qh,d[ee]=ce);const Ht=g.getCameraImage(ee);ce.sourceTexture=Ht}}}}for(let st=0;st<D.length;st++){const Mt=b[st],bt=D[st];Mt!==null&&bt!==void 0&&bt.update(Mt,nt,l||a)}kt&&kt(U,nt),nt.detectedPlanes&&i.dispatchEvent({type:"planesdetected",data:nt}),_=null}const C=new Jh;C.setAnimationLoop(Qt),this.setAnimationLoop=function(U){kt=U},this.dispose=function(){}}}const f0=new me,ru=new zt;ru.set(-1,0,0,0,1,0,0,0,1);function p0(n,t){function e(f,d){f.matrixAutoUpdate===!0&&f.updateMatrix(),d.value.copy(f.matrix)}function i(f,d){d.color.getRGB(f.fogColor.value,Yh(n)),d.isFog?(f.fogNear.value=d.near,f.fogFar.value=d.far):d.isFogExp2&&(f.fogDensity.value=d.density)}function s(f,d,E,A,T){d.isNodeMaterial?d.uniformsNeedUpdate=!1:d.isMeshBasicMaterial?r(f,d):d.isMeshLambertMaterial?(r(f,d),d.envMap&&(f.envMapIntensity.value=d.envMapIntensity)):d.isMeshToonMaterial?(r(f,d),g(f,d)):d.isMeshPhongMaterial?(r(f,d),u(f,d),d.envMap&&(f.envMapIntensity.value=d.envMapIntensity)):d.isMeshStandardMaterial?(r(f,d),h(f,d),d.isMeshPhysicalMaterial&&m(f,d,T)):d.isMeshMatcapMaterial?(r(f,d),_(f,d)):d.isMeshDepthMaterial?r(f,d):d.isMeshDistanceMaterial?(r(f,d),y(f,d)):d.isMeshNormalMaterial?r(f,d):d.isLineBasicMaterial?(a(f,d),d.isLineDashedMaterial&&o(f,d)):d.isPointsMaterial?c(f,d,E,A):d.isSpriteMaterial?l(f,d):d.isShadowMaterial?(f.color.value.copy(d.color),f.opacity.value=d.opacity):d.isShaderMaterial&&(d.uniformsNeedUpdate=!1)}function r(f,d){f.opacity.value=d.opacity,d.color&&f.diffuse.value.copy(d.color),d.emissive&&f.emissive.value.copy(d.emissive).multiplyScalar(d.emissiveIntensity),d.map&&(f.map.value=d.map,e(d.map,f.mapTransform)),d.alphaMap&&(f.alphaMap.value=d.alphaMap,e(d.alphaMap,f.alphaMapTransform)),d.bumpMap&&(f.bumpMap.value=d.bumpMap,e(d.bumpMap,f.bumpMapTransform),f.bumpScale.value=d.bumpScale,d.side===Ke&&(f.bumpScale.value*=-1)),d.normalMap&&(f.normalMap.value=d.normalMap,e(d.normalMap,f.normalMapTransform),f.normalScale.value.copy(d.normalScale),d.side===Ke&&f.normalScale.value.negate()),d.displacementMap&&(f.displacementMap.value=d.displacementMap,e(d.displacementMap,f.displacementMapTransform),f.displacementScale.value=d.displacementScale,f.displacementBias.value=d.displacementBias),d.emissiveMap&&(f.emissiveMap.value=d.emissiveMap,e(d.emissiveMap,f.emissiveMapTransform)),d.specularMap&&(f.specularMap.value=d.specularMap,e(d.specularMap,f.specularMapTransform)),d.alphaTest>0&&(f.alphaTest.value=d.alphaTest);const E=t.get(d),A=E.envMap,T=E.envMapRotation;A&&(f.envMap.value=A,f.envMapRotation.value.setFromMatrix4(f0.makeRotationFromEuler(T)).transpose(),A.isCubeTexture&&A.isRenderTargetTexture===!1&&f.envMapRotation.value.premultiply(ru),f.reflectivity.value=d.reflectivity,f.ior.value=d.ior,f.refractionRatio.value=d.refractionRatio),d.lightMap&&(f.lightMap.value=d.lightMap,f.lightMapIntensity.value=d.lightMapIntensity,e(d.lightMap,f.lightMapTransform)),d.aoMap&&(f.aoMap.value=d.aoMap,f.aoMapIntensity.value=d.aoMapIntensity,e(d.aoMap,f.aoMapTransform))}function a(f,d){f.diffuse.value.copy(d.color),f.opacity.value=d.opacity,d.map&&(f.map.value=d.map,e(d.map,f.mapTransform))}function o(f,d){f.dashSize.value=d.dashSize,f.totalSize.value=d.dashSize+d.gapSize,f.scale.value=d.scale}function c(f,d,E,A){f.diffuse.value.copy(d.color),f.opacity.value=d.opacity,f.size.value=d.size*E,f.scale.value=A*.5,d.map&&(f.map.value=d.map,e(d.map,f.uvTransform)),d.alphaMap&&(f.alphaMap.value=d.alphaMap,e(d.alphaMap,f.alphaMapTransform)),d.alphaTest>0&&(f.alphaTest.value=d.alphaTest)}function l(f,d){f.diffuse.value.copy(d.color),f.opacity.value=d.opacity,f.rotation.value=d.rotation,d.map&&(f.map.value=d.map,e(d.map,f.mapTransform)),d.alphaMap&&(f.alphaMap.value=d.alphaMap,e(d.alphaMap,f.alphaMapTransform)),d.alphaTest>0&&(f.alphaTest.value=d.alphaTest)}function u(f,d){f.specular.value.copy(d.specular),f.shininess.value=Math.max(d.shininess,1e-4)}function g(f,d){d.gradientMap&&(f.gradientMap.value=d.gradientMap)}function h(f,d){f.metalness.value=d.metalness,d.metalnessMap&&(f.metalnessMap.value=d.metalnessMap,e(d.metalnessMap,f.metalnessMapTransform)),f.roughness.value=d.roughness,d.roughnessMap&&(f.roughnessMap.value=d.roughnessMap,e(d.roughnessMap,f.roughnessMapTransform)),d.envMap&&(f.envMapIntensity.value=d.envMapIntensity)}function m(f,d,E){f.ior.value=d.ior,d.sheen>0&&(f.sheenColor.value.copy(d.sheenColor).multiplyScalar(d.sheen),f.sheenRoughness.value=d.sheenRoughness,d.sheenColorMap&&(f.sheenColorMap.value=d.sheenColorMap,e(d.sheenColorMap,f.sheenColorMapTransform)),d.sheenRoughnessMap&&(f.sheenRoughnessMap.value=d.sheenRoughnessMap,e(d.sheenRoughnessMap,f.sheenRoughnessMapTransform))),d.clearcoat>0&&(f.clearcoat.value=d.clearcoat,f.clearcoatRoughness.value=d.clearcoatRoughness,d.clearcoatMap&&(f.clearcoatMap.value=d.clearcoatMap,e(d.clearcoatMap,f.clearcoatMapTransform)),d.clearcoatRoughnessMap&&(f.clearcoatRoughnessMap.value=d.clearcoatRoughnessMap,e(d.clearcoatRoughnessMap,f.clearcoatRoughnessMapTransform)),d.clearcoatNormalMap&&(f.clearcoatNormalMap.value=d.clearcoatNormalMap,e(d.clearcoatNormalMap,f.clearcoatNormalMapTransform),f.clearcoatNormalScale.value.copy(d.clearcoatNormalScale),d.side===Ke&&f.clearcoatNormalScale.value.negate())),d.dispersion>0&&(f.dispersion.value=d.dispersion),d.iridescence>0&&(f.iridescence.value=d.iridescence,f.iridescenceIOR.value=d.iridescenceIOR,f.iridescenceThicknessMinimum.value=d.iridescenceThicknessRange[0],f.iridescenceThicknessMaximum.value=d.iridescenceThicknessRange[1],d.iridescenceMap&&(f.iridescenceMap.value=d.iridescenceMap,e(d.iridescenceMap,f.iridescenceMapTransform)),d.iridescenceThicknessMap&&(f.iridescenceThicknessMap.value=d.iridescenceThicknessMap,e(d.iridescenceThicknessMap,f.iridescenceThicknessMapTransform))),d.transmission>0&&(f.transmission.value=d.transmission,f.transmissionSamplerMap.value=E.texture,f.transmissionSamplerSize.value.set(E.width,E.height),d.transmissionMap&&(f.transmissionMap.value=d.transmissionMap,e(d.transmissionMap,f.transmissionMapTransform)),f.thickness.value=d.thickness,d.thicknessMap&&(f.thicknessMap.value=d.thicknessMap,e(d.thicknessMap,f.thicknessMapTransform)),f.attenuationDistance.value=d.attenuationDistance,f.attenuationColor.value.copy(d.attenuationColor)),d.anisotropy>0&&(f.anisotropyVector.value.set(d.anisotropy*Math.cos(d.anisotropyRotation),d.anisotropy*Math.sin(d.anisotropyRotation)),d.anisotropyMap&&(f.anisotropyMap.value=d.anisotropyMap,e(d.anisotropyMap,f.anisotropyMapTransform))),f.specularIntensity.value=d.specularIntensity,f.specularColor.value.copy(d.specularColor),d.specularColorMap&&(f.specularColorMap.value=d.specularColorMap,e(d.specularColorMap,f.specularColorMapTransform)),d.specularIntensityMap&&(f.specularIntensityMap.value=d.specularIntensityMap,e(d.specularIntensityMap,f.specularIntensityMapTransform))}function _(f,d){d.matcap&&(f.matcap.value=d.matcap)}function y(f,d){const E=t.get(d).light;f.referencePosition.value.setFromMatrixPosition(E.matrixWorld),f.nearDistance.value=E.shadow.camera.near,f.farDistance.value=E.shadow.camera.far}return{refreshFogUniforms:i,refreshMaterialUniforms:s}}function m0(n,t,e,i){let s={},r={},a=[];const o=n.getParameter(n.MAX_UNIFORM_BUFFER_BINDINGS);function c(E,A){const T=A.program;i.uniformBlockBinding(E,T)}function l(E,A){let T=s[E.id];T===void 0&&(_(E),T=u(E),s[E.id]=T,E.addEventListener("dispose",f));const D=A.program;i.updateUBOMapping(E,D);const b=t.render.frame;r[E.id]!==b&&(h(E),r[E.id]=b)}function u(E){const A=g();E.__bindingPointIndex=A;const T=n.createBuffer(),D=E.__size,b=E.usage;return n.bindBuffer(n.UNIFORM_BUFFER,T),n.bufferData(n.UNIFORM_BUFFER,D,b),n.bindBuffer(n.UNIFORM_BUFFER,null),n.bindBufferBase(n.UNIFORM_BUFFER,A,T),T}function g(){for(let E=0;E<o;E++)if(a.indexOf(E)===-1)return a.push(E),E;return Kt("WebGLRenderer: Maximum number of simultaneously usable uniforms groups reached."),0}function h(E){const A=s[E.id],T=E.uniforms,D=E.__cache;n.bindBuffer(n.UNIFORM_BUFFER,A);for(let b=0,N=T.length;b<N;b++){const M=Array.isArray(T[b])?T[b]:[T[b]];for(let w=0,z=M.length;w<z;w++){const P=M[w];if(m(P,b,w,D)===!0){const V=P.__offset,J=Array.isArray(P.value)?P.value:[P.value];let K=0;for(let B=0;B<J.length;B++){const W=J[B],$=y(W);typeof W=="number"||typeof W=="boolean"?(P.__data[0]=W,n.bufferSubData(n.UNIFORM_BUFFER,V+K,P.__data)):W.isMatrix3?(P.__data[0]=W.elements[0],P.__data[1]=W.elements[1],P.__data[2]=W.elements[2],P.__data[3]=0,P.__data[4]=W.elements[3],P.__data[5]=W.elements[4],P.__data[6]=W.elements[5],P.__data[7]=0,P.__data[8]=W.elements[6],P.__data[9]=W.elements[7],P.__data[10]=W.elements[8],P.__data[11]=0):ArrayBuffer.isView(W)?P.__data.set(new W.constructor(W.buffer,W.byteOffset,P.__data.length)):(W.toArray(P.__data,K),K+=$.storage/Float32Array.BYTES_PER_ELEMENT)}n.bufferSubData(n.UNIFORM_BUFFER,V,P.__data)}}}n.bindBuffer(n.UNIFORM_BUFFER,null)}function m(E,A,T,D){const b=E.value,N=A+"_"+T;if(D[N]===void 0)return typeof b=="number"||typeof b=="boolean"?D[N]=b:ArrayBuffer.isView(b)?D[N]=b.slice():D[N]=b.clone(),!0;{const M=D[N];if(typeof b=="number"||typeof b=="boolean"){if(M!==b)return D[N]=b,!0}else{if(ArrayBuffer.isView(b))return!0;if(M.equals(b)===!1)return M.copy(b),!0}}return!1}function _(E){const A=E.uniforms;let T=0;const D=16;for(let N=0,M=A.length;N<M;N++){const w=Array.isArray(A[N])?A[N]:[A[N]];for(let z=0,P=w.length;z<P;z++){const V=w[z],J=Array.isArray(V.value)?V.value:[V.value];for(let K=0,B=J.length;K<B;K++){const W=J[K],$=y(W),et=T%D,at=et%$.boundary,ct=et+at;T+=at,ct!==0&&D-ct<$.storage&&(T+=D-ct),V.__data=new Float32Array($.storage/Float32Array.BYTES_PER_ELEMENT),V.__offset=T,T+=$.storage}}}const b=T%D;return b>0&&(T+=D-b),E.__size=T,E.__cache={},this}function y(E){const A={boundary:0,storage:0};return typeof E=="number"||typeof E=="boolean"?(A.boundary=4,A.storage=4):E.isVector2?(A.boundary=8,A.storage=8):E.isVector3||E.isColor?(A.boundary=16,A.storage=12):E.isVector4?(A.boundary=16,A.storage=16):E.isMatrix3?(A.boundary=48,A.storage=48):E.isMatrix4?(A.boundary=64,A.storage=64):E.isTexture?It("WebGLRenderer: Texture samplers can not be part of an uniforms group."):ArrayBuffer.isView(E)?(A.boundary=16,A.storage=E.byteLength):It("WebGLRenderer: Unsupported uniform value type.",E),A}function f(E){const A=E.target;A.removeEventListener("dispose",f);const T=a.indexOf(A.__bindingPointIndex);a.splice(T,1),n.deleteBuffer(s[A.id]),delete s[A.id],delete r[A.id]}function d(){for(const E in s)n.deleteBuffer(s[E]);a=[],s={},r={}}return{bind:c,update:l,dispose:d}}const g0=new Uint16Array([12469,15057,12620,14925,13266,14620,13807,14376,14323,13990,14545,13625,14713,13328,14840,12882,14931,12528,14996,12233,15039,11829,15066,11525,15080,11295,15085,10976,15082,10705,15073,10495,13880,14564,13898,14542,13977,14430,14158,14124,14393,13732,14556,13410,14702,12996,14814,12596,14891,12291,14937,11834,14957,11489,14958,11194,14943,10803,14921,10506,14893,10278,14858,9960,14484,14039,14487,14025,14499,13941,14524,13740,14574,13468,14654,13106,14743,12678,14818,12344,14867,11893,14889,11509,14893,11180,14881,10751,14852,10428,14812,10128,14765,9754,14712,9466,14764,13480,14764,13475,14766,13440,14766,13347,14769,13070,14786,12713,14816,12387,14844,11957,14860,11549,14868,11215,14855,10751,14825,10403,14782,10044,14729,9651,14666,9352,14599,9029,14967,12835,14966,12831,14963,12804,14954,12723,14936,12564,14917,12347,14900,11958,14886,11569,14878,11247,14859,10765,14828,10401,14784,10011,14727,9600,14660,9289,14586,8893,14508,8533,15111,12234,15110,12234,15104,12216,15092,12156,15067,12010,15028,11776,14981,11500,14942,11205,14902,10752,14861,10393,14812,9991,14752,9570,14682,9252,14603,8808,14519,8445,14431,8145,15209,11449,15208,11451,15202,11451,15190,11438,15163,11384,15117,11274,15055,10979,14994,10648,14932,10343,14871,9936,14803,9532,14729,9218,14645,8742,14556,8381,14461,8020,14365,7603,15273,10603,15272,10607,15267,10619,15256,10631,15231,10614,15182,10535,15118,10389,15042,10167,14963,9787,14883,9447,14800,9115,14710,8665,14615,8318,14514,7911,14411,7507,14279,7198,15314,9675,15313,9683,15309,9712,15298,9759,15277,9797,15229,9773,15166,9668,15084,9487,14995,9274,14898,8910,14800,8539,14697,8234,14590,7790,14479,7409,14367,7067,14178,6621,15337,8619,15337,8631,15333,8677,15325,8769,15305,8871,15264,8940,15202,8909,15119,8775,15022,8565,14916,8328,14804,8009,14688,7614,14569,7287,14448,6888,14321,6483,14088,6171,15350,7402,15350,7419,15347,7480,15340,7613,15322,7804,15287,7973,15229,8057,15148,8012,15046,7846,14933,7611,14810,7357,14682,7069,14552,6656,14421,6316,14251,5948,14007,5528,15356,5942,15356,5977,15353,6119,15348,6294,15332,6551,15302,6824,15249,7044,15171,7122,15070,7050,14949,6861,14818,6611,14679,6349,14538,6067,14398,5651,14189,5311,13935,4958,15359,4123,15359,4153,15356,4296,15353,4646,15338,5160,15311,5508,15263,5829,15188,6042,15088,6094,14966,6001,14826,5796,14678,5543,14527,5287,14377,4985,14133,4586,13869,4257,15360,1563,15360,1642,15358,2076,15354,2636,15341,3350,15317,4019,15273,4429,15203,4732,15105,4911,14981,4932,14836,4818,14679,4621,14517,4386,14359,4156,14083,3795,13808,3437,15360,122,15360,137,15358,285,15355,636,15344,1274,15322,2177,15281,2765,15215,3223,15120,3451,14995,3569,14846,3567,14681,3466,14511,3305,14344,3121,14037,2800,13753,2467,15360,0,15360,1,15359,21,15355,89,15346,253,15325,479,15287,796,15225,1148,15133,1492,15008,1749,14856,1882,14685,1886,14506,1783,14324,1608,13996,1398,13702,1183]);let yn=null;function _0(){return yn===null&&(yn=new of(g0,16,16,Li,Vn),yn.name="DFG_LUT",yn.minFilter=Be,yn.magFilter=Be,yn.wrapS=kn,yn.wrapT=kn,yn.generateMipmaps=!1,yn.needsUpdate=!0),yn}class x0{constructor(t={}){const{canvas:e=Ud(),context:i=null,depth:s=!0,stencil:r=!1,alpha:a=!1,antialias:o=!1,premultipliedAlpha:c=!0,preserveDrawingBuffer:l=!1,powerPreference:u="default",failIfMajorPerformanceCaveat:g=!1,reversedDepthBuffer:h=!1,outputBufferType:m=an}=t;this.isWebGLRenderer=!0;let _;if(i!==null){if(typeof WebGLRenderingContext<"u"&&i instanceof WebGLRenderingContext)throw new Error("THREE.WebGLRenderer: WebGL 1 is not supported since r163.");_=i.getContextAttributes().alpha}else _=a;const y=m,f=new Set([yl,Sl,Ml]),d=new Set([an,Ln,ks,Hs,xl,vl]),E=new Uint32Array(4),A=new Int32Array(4),T=new F;let D=null,b=null;const N=[],M=[];let w=null;this.domElement=e,this.debug={checkShaderErrors:!0,onShaderError:null},this.autoClear=!0,this.autoClearColor=!0,this.autoClearDepth=!0,this.autoClearStencil=!0,this.sortObjects=!0,this.clippingPlanes=[],this.localClippingEnabled=!1,this.toneMapping=Cn,this.toneMappingExposure=1,this.transmissionResolutionScale=1;const z=this;let P=!1,V=null;this._outputColorSpace=sn;let J=0,K=0,B=null,W=-1,$=null;const et=new ye,at=new ye;let ct=null;const yt=new Bt(0);let wt=0,kt=e.width,Qt=e.height,C=1,U=null,nt=null;const st=new ye(0,0,kt,Qt),Mt=new ye(0,0,kt,Qt);let bt=!1;const Rt=new Vh;let qt=!1,Ut=!1;const ee=new me,ce=new F,Ht=new ye,pe={background:null,fog:null,environment:null,overrideMaterial:null,isScene:!0};let se=!1;function be(){return B===null?C:1}let S=i;function j(v,O){return e.getContext(v,O)}try{const v={alpha:!0,depth:s,stencil:r,antialias:o,premultipliedAlpha:c,preserveDrawingBuffer:l,powerPreference:u,failIfMajorPerformanceCaveat:g};if("setAttribute"in e&&e.setAttribute("data-engine",`three.js r${gl}`),e.addEventListener("webglcontextlost",rt,!1),e.addEventListener("webglcontextrestored",Pt,!1),e.addEventListener("webglcontextcreationerror",Gt,!1),S===null){const O="webgl2";if(S=j(O,v),S===null)throw j(O)?new Error("Error creating WebGL context with your selected attributes."):new Error("Error creating WebGL context.")}}catch(v){throw Kt("WebGLRenderer: "+v.message),v}let Q,ot,Y,vt,x,p,R,k,Z,it,lt,I,X,dt,_t,ft,ht,Ft,Wt,te,L,ut,tt;function St(){Q=new _g(S),Q.init(),L=new l0(S,Q),ot=new cg(S,Q,t,L),Y=new a0(S,Q),ot.reversedDepthBuffer&&h&&Y.buffers.depth.setReversed(!0),vt=new Mg(S),x=new $_,p=new o0(S,Q,Y,x,ot,L,vt),R=new gg(z),k=new bf(S),ut=new og(S,k),Z=new xg(S,k,vt,ut),it=new yg(S,Z,k,ut,vt),Ft=new Sg(S,ot,p),_t=new hg(x),lt=new X_(z,R,Q,ot,ut,_t),I=new p0(z,x),X=new Y_,dt=new t0(Q),ht=new ag(z,R,Y,it,_,c),ft=new r0(z,it,ot),tt=new m0(S,vt,ot,Y),Wt=new lg(S,Q,vt),te=new vg(S,Q,vt),vt.programs=lt.programs,z.capabilities=ot,z.extensions=Q,z.properties=x,z.renderLists=X,z.shadowMap=ft,z.state=Y,z.info=vt}St(),y!==an&&(w=new bg(y,e.width,e.height,s,r));const pt=new d0(z,S);this.xr=pt,this.getContext=function(){return S},this.getContextAttributes=function(){return S.getContextAttributes()},this.forceContextLoss=function(){const v=Q.get("WEBGL_lose_context");v&&v.loseContext()},this.forceContextRestore=function(){const v=Q.get("WEBGL_lose_context");v&&v.restoreContext()},this.getPixelRatio=function(){return C},this.setPixelRatio=function(v){v!==void 0&&(C=v,this.setSize(kt,Qt,!1))},this.getSize=function(v){return v.set(kt,Qt)},this.setSize=function(v,O,q=!0){if(pt.isPresenting){It("WebGLRenderer: Can't change size while VR device is presenting.");return}kt=v,Qt=O,e.width=Math.floor(v*C),e.height=Math.floor(O*C),q===!0&&(e.style.width=v+"px",e.style.height=O+"px"),w!==null&&w.setSize(e.width,e.height),this.setViewport(0,0,v,O)},this.getDrawingBufferSize=function(v){return v.set(kt*C,Qt*C).floor()},this.setDrawingBufferSize=function(v,O,q){kt=v,Qt=O,C=q,e.width=Math.floor(v*q),e.height=Math.floor(O*q),this.setViewport(0,0,v,O)},this.setEffects=function(v){if(y===an){Kt("THREE.WebGLRenderer: setEffects() requires outputBufferType set to HalfFloatType or FloatType.");return}if(v){for(let O=0;O<v.length;O++)if(v[O].isOutputPass===!0){It("THREE.WebGLRenderer: OutputPass is not needed in setEffects(). Tone mapping and color space conversion are applied automatically.");break}}w.setEffects(v||[])},this.getCurrentViewport=function(v){return v.copy(et)},this.getViewport=function(v){return v.copy(st)},this.setViewport=function(v,O,q,H){v.isVector4?st.set(v.x,v.y,v.z,v.w):st.set(v,O,q,H),Y.viewport(et.copy(st).multiplyScalar(C).round())},this.getScissor=function(v){return v.copy(Mt)},this.setScissor=function(v,O,q,H){v.isVector4?Mt.set(v.x,v.y,v.z,v.w):Mt.set(v,O,q,H),Y.scissor(at.copy(Mt).multiplyScalar(C).round())},this.getScissorTest=function(){return bt},this.setScissorTest=function(v){Y.setScissorTest(bt=v)},this.setOpaqueSort=function(v){U=v},this.setTransparentSort=function(v){nt=v},this.getClearColor=function(v){return v.copy(ht.getClearColor())},this.setClearColor=function(){ht.setClearColor(...arguments)},this.getClearAlpha=function(){return ht.getClearAlpha()},this.setClearAlpha=function(){ht.setClearAlpha(...arguments)},this.clear=function(v=!0,O=!0,q=!0){let H=0;if(v){let G=!1;if(B!==null){const xt=B.texture.format;G=f.has(xt)}if(G){const xt=B.texture.type,Tt=d.has(xt),gt=ht.getClearColor(),Ct=ht.getClearAlpha(),Lt=gt.r,Vt=gt.g,$t=gt.b;Tt?(E[0]=Lt,E[1]=Vt,E[2]=$t,E[3]=Ct,S.clearBufferuiv(S.COLOR,0,E)):(A[0]=Lt,A[1]=Vt,A[2]=$t,A[3]=Ct,S.clearBufferiv(S.COLOR,0,A))}else H|=S.COLOR_BUFFER_BIT}O&&(H|=S.DEPTH_BUFFER_BIT,this.state.buffers.depth.setMask(!0)),q&&(H|=S.STENCIL_BUFFER_BIT,this.state.buffers.stencil.setMask(4294967295)),H!==0&&S.clear(H)},this.clearColor=function(){this.clear(!0,!1,!1)},this.clearDepth=function(){this.clear(!1,!0,!1)},this.clearStencil=function(){this.clear(!1,!1,!0)},this.setNodesHandler=function(v){v.setRenderer(this),V=v},this.dispose=function(){e.removeEventListener("webglcontextlost",rt,!1),e.removeEventListener("webglcontextrestored",Pt,!1),e.removeEventListener("webglcontextcreationerror",Gt,!1),ht.dispose(),X.dispose(),dt.dispose(),x.dispose(),R.dispose(),it.dispose(),ut.dispose(),tt.dispose(),lt.dispose(),pt.dispose(),pt.removeEventListener("sessionstart",ql),pt.removeEventListener("sessionend",Yl),mi.stop()};function rt(v){v.preventDefault(),oa("WebGLRenderer: Context Lost."),P=!0}function Pt(){oa("WebGLRenderer: Context Restored."),P=!1;const v=vt.autoReset,O=ft.enabled,q=ft.autoUpdate,H=ft.needsUpdate,G=ft.type;St(),vt.autoReset=v,ft.enabled=O,ft.autoUpdate=q,ft.needsUpdate=H,ft.type=G}function Gt(v){Kt("WebGLRenderer: A WebGL context could not be created. Reason: ",v.statusMessage)}function xe(v){const O=v.target;O.removeEventListener("dispose",xe),re(O)}function re(v){Dn(v),x.remove(v)}function Dn(v){const O=x.get(v).programs;O!==void 0&&(O.forEach(function(q){lt.releaseProgram(q)}),v.isShaderMaterial&&lt.releaseShaderCache(v))}this.renderBufferDirect=function(v,O,q,H,G,xt){O===null&&(O=pe);const Tt=G.isMesh&&G.matrixWorld.determinant()<0,gt=Yu(v,O,q,H,G);Y.setMaterial(H,Tt);let Ct=q.index,Lt=1;if(H.wireframe===!0){if(Ct=Z.getWireframeAttribute(q),Ct===void 0)return;Lt=2}const Vt=q.drawRange,$t=q.attributes.position;let Dt=Vt.start*Lt,ae=(Vt.start+Vt.count)*Lt;xt!==null&&(Dt=Math.max(Dt,xt.start*Lt),ae=Math.min(ae,(xt.start+xt.count)*Lt)),Ct!==null?(Dt=Math.max(Dt,0),ae=Math.min(ae,Ct.count)):$t!=null&&(Dt=Math.max(Dt,0),ae=Math.min(ae,$t.count));const ve=ae-Dt;if(ve<0||ve===1/0)return;ut.setup(G,H,gt,q,Ct);let ge,he=Wt;if(Ct!==null&&(ge=k.get(Ct),he=te,he.setIndex(ge)),G.isMesh)H.wireframe===!0?(Y.setLineWidth(H.wireframeLinewidth*be()),he.setMode(S.LINES)):he.setMode(S.TRIANGLES);else if(G.isLine){let Ne=H.linewidth;Ne===void 0&&(Ne=1),Y.setLineWidth(Ne*be()),G.isLineSegments?he.setMode(S.LINES):G.isLineLoop?he.setMode(S.LINE_LOOP):he.setMode(S.LINE_STRIP)}else G.isPoints?he.setMode(S.POINTS):G.isSprite&&he.setMode(S.TRIANGLES);if(G.isBatchedMesh)if(Q.get("WEBGL_multi_draw"))he.renderMultiDraw(G._multiDrawStarts,G._multiDrawCounts,G._multiDrawCount);else{const Ne=G._multiDrawStarts,Et=G._multiDrawCounts,Ze=G._multiDrawCount,Jt=Ct?k.get(Ct).bytesPerElement:1,en=x.get(H).currentProgram.getUniforms();for(let Mn=0;Mn<Ze;Mn++)en.setValue(S,"_gl_DrawID",Mn),he.render(Ne[Mn]/Jt,Et[Mn])}else if(G.isInstancedMesh)he.renderInstances(Dt,ve,G.count);else if(q.isInstancedBufferGeometry){const Ne=q._maxInstanceCount!==void 0?q._maxInstanceCount:1/0,Et=Math.min(q.instanceCount,Ne);he.renderInstances(Dt,ve,Et)}else he.render(Dt,ve)};function vn(v,O,q){v.transparent===!0&&v.side===Bn&&v.forceSinglePass===!1?(v.side=Ke,v.needsUpdate=!0,tr(v,O,q),v.side=ci,v.needsUpdate=!0,tr(v,O,q),v.side=Bn):tr(v,O,q)}this.compile=function(v,O,q=null){q===null&&(q=v),b=dt.get(q),b.init(O),M.push(b),q.traverseVisible(function(G){G.isLight&&G.layers.test(O.layers)&&(b.pushLight(G),G.castShadow&&b.pushShadow(G))}),v!==q&&v.traverseVisible(function(G){G.isLight&&G.layers.test(O.layers)&&(b.pushLight(G),G.castShadow&&b.pushShadow(G))}),b.setupLights();const H=new Set;return v.traverse(function(G){if(!(G.isMesh||G.isPoints||G.isLine||G.isSprite))return;const xt=G.material;if(xt)if(Array.isArray(xt))for(let Tt=0;Tt<xt.length;Tt++){const gt=xt[Tt];vn(gt,q,G),H.add(gt)}else vn(xt,q,G),H.add(xt)}),b=M.pop(),H},this.compileAsync=function(v,O,q=null){const H=this.compile(v,O,q);return new Promise(G=>{function xt(){if(H.forEach(function(Tt){x.get(Tt).currentProgram.isReady()&&H.delete(Tt)}),H.size===0){G(v);return}setTimeout(xt,10)}Q.get("KHR_parallel_shader_compile")!==null?xt():setTimeout(xt,10)})};let Ea=null;function $u(v){Ea&&Ea(v)}function ql(){mi.stop()}function Yl(){mi.start()}const mi=new Jh;mi.setAnimationLoop($u),typeof self<"u"&&mi.setContext(self),this.setAnimationLoop=function(v){Ea=v,pt.setAnimationLoop(v),v===null?mi.stop():mi.start()},pt.addEventListener("sessionstart",ql),pt.addEventListener("sessionend",Yl),this.render=function(v,O){if(O!==void 0&&O.isCamera!==!0){Kt("WebGLRenderer.render: camera is not an instance of THREE.Camera.");return}if(P===!0)return;V!==null&&V.renderStart(v,O);const q=pt.enabled===!0&&pt.isPresenting===!0,H=w!==null&&(B===null||q)&&w.begin(z,B);if(v.matrixWorldAutoUpdate===!0&&v.updateMatrixWorld(),O.parent===null&&O.matrixWorldAutoUpdate===!0&&O.updateMatrixWorld(),pt.enabled===!0&&pt.isPresenting===!0&&(w===null||w.isCompositing()===!1)&&(pt.cameraAutoUpdate===!0&&pt.updateCamera(O),O=pt.getCamera()),v.isScene===!0&&v.onBeforeRender(z,v,O,B),b=dt.get(v,M.length),b.init(O),b.state.textureUnits=p.getTextureUnits(),M.push(b),ee.multiplyMatrices(O.projectionMatrix,O.matrixWorldInverse),Rt.setFromProjectionMatrix(ee,wn,O.reversedDepth),Ut=this.localClippingEnabled,qt=_t.init(this.clippingPlanes,Ut),D=X.get(v,N.length),D.init(),N.push(D),pt.enabled===!0&&pt.isPresenting===!0){const Tt=z.xr.getDepthSensingMesh();Tt!==null&&ba(Tt,O,-1/0,z.sortObjects)}ba(v,O,0,z.sortObjects),D.finish(),z.sortObjects===!0&&D.sort(U,nt),se=pt.enabled===!1||pt.isPresenting===!1||pt.hasDepthSensing()===!1,se&&ht.addToRenderList(D,v),this.info.render.frame++,qt===!0&&_t.beginShadows();const G=b.state.shadowsArray;if(ft.render(G,v,O),qt===!0&&_t.endShadows(),this.info.autoReset===!0&&this.info.reset(),(H&&w.hasRenderPass())===!1){const Tt=D.opaque,gt=D.transmissive;if(b.setupLights(),O.isArrayCamera){const Ct=O.cameras;if(gt.length>0)for(let Lt=0,Vt=Ct.length;Lt<Vt;Lt++){const $t=Ct[Lt];Kl(Tt,gt,v,$t)}se&&ht.render(v);for(let Lt=0,Vt=Ct.length;Lt<Vt;Lt++){const $t=Ct[Lt];jl(D,v,$t,$t.viewport)}}else gt.length>0&&Kl(Tt,gt,v,O),se&&ht.render(v),jl(D,v,O)}B!==null&&K===0&&(p.updateMultisampleRenderTarget(B),p.updateRenderTargetMipmap(B)),H&&w.end(z),v.isScene===!0&&v.onAfterRender(z,v,O),ut.resetDefaultState(),W=-1,$=null,M.pop(),M.length>0?(b=M[M.length-1],p.setTextureUnits(b.state.textureUnits),qt===!0&&_t.setGlobalState(z.clippingPlanes,b.state.camera)):b=null,N.pop(),N.length>0?D=N[N.length-1]:D=null,V!==null&&V.renderEnd()};function ba(v,O,q,H){if(v.visible===!1)return;if(v.layers.test(O.layers)){if(v.isGroup)q=v.renderOrder;else if(v.isLOD)v.autoUpdate===!0&&v.update(O);else if(v.isLightProbeGrid)b.pushLightProbeGrid(v);else if(v.isLight)b.pushLight(v),v.castShadow&&b.pushShadow(v);else if(v.isSprite){if(!v.frustumCulled||Rt.intersectsSprite(v)){H&&Ht.setFromMatrixPosition(v.matrixWorld).applyMatrix4(ee);const Tt=it.update(v),gt=v.material;gt.visible&&D.push(v,Tt,gt,q,Ht.z,null)}}else if((v.isMesh||v.isLine||v.isPoints)&&(!v.frustumCulled||Rt.intersectsObject(v))){const Tt=it.update(v),gt=v.material;if(H&&(v.boundingSphere!==void 0?(v.boundingSphere===null&&v.computeBoundingSphere(),Ht.copy(v.boundingSphere.center)):(Tt.boundingSphere===null&&Tt.computeBoundingSphere(),Ht.copy(Tt.boundingSphere.center)),Ht.applyMatrix4(v.matrixWorld).applyMatrix4(ee)),Array.isArray(gt)){const Ct=Tt.groups;for(let Lt=0,Vt=Ct.length;Lt<Vt;Lt++){const $t=Ct[Lt],Dt=gt[$t.materialIndex];Dt&&Dt.visible&&D.push(v,Tt,Dt,q,Ht.z,$t)}}else gt.visible&&D.push(v,Tt,gt,q,Ht.z,null)}}const xt=v.children;for(let Tt=0,gt=xt.length;Tt<gt;Tt++)ba(xt[Tt],O,q,H)}function jl(v,O,q,H){const{opaque:G,transmissive:xt,transparent:Tt}=v;b.setupLightsView(q),qt===!0&&_t.setGlobalState(z.clippingPlanes,q),H&&Y.viewport(et.copy(H)),G.length>0&&Qs(G,O,q),xt.length>0&&Qs(xt,O,q),Tt.length>0&&Qs(Tt,O,q),Y.buffers.depth.setTest(!0),Y.buffers.depth.setMask(!0),Y.buffers.color.setMask(!0),Y.setPolygonOffset(!1)}function Kl(v,O,q,H){if((q.isScene===!0?q.overrideMaterial:null)!==null)return;if(b.state.transmissionRenderTarget[H.id]===void 0){const Dt=Q.has("EXT_color_buffer_half_float")||Q.has("EXT_color_buffer_float");b.state.transmissionRenderTarget[H.id]=new Pn(1,1,{generateMipmaps:!0,type:Dt?Vn:an,minFilter:Ti,samples:Math.max(4,ot.samples),stencilBuffer:r,resolveDepthBuffer:!1,resolveStencilBuffer:!1,colorSpace:jt.workingColorSpace})}const xt=b.state.transmissionRenderTarget[H.id],Tt=H.viewport||et;xt.setSize(Tt.z*z.transmissionResolutionScale,Tt.w*z.transmissionResolutionScale);const gt=z.getRenderTarget(),Ct=z.getActiveCubeFace(),Lt=z.getActiveMipmapLevel();z.setRenderTarget(xt),z.getClearColor(yt),wt=z.getClearAlpha(),wt<1&&z.setClearColor(16777215,.5),z.clear(),se&&ht.render(q);const Vt=z.toneMapping;z.toneMapping=Cn;const $t=H.viewport;if(H.viewport!==void 0&&(H.viewport=void 0),b.setupLightsView(H),qt===!0&&_t.setGlobalState(z.clippingPlanes,H),Qs(v,q,H),p.updateMultisampleRenderTarget(xt),p.updateRenderTargetMipmap(xt),Q.has("WEBGL_multisampled_render_to_texture")===!1){let Dt=!1;for(let ae=0,ve=O.length;ae<ve;ae++){const ge=O[ae],{object:he,geometry:Ne,material:Et,group:Ze}=ge;if(Et.side===Bn&&he.layers.test(H.layers)){const Jt=Et.side;Et.side=Ke,Et.needsUpdate=!0,Zl(he,q,H,Ne,Et,Ze),Et.side=Jt,Et.needsUpdate=!0,Dt=!0}}Dt===!0&&(p.updateMultisampleRenderTarget(xt),p.updateRenderTargetMipmap(xt))}z.setRenderTarget(gt,Ct,Lt),z.setClearColor(yt,wt),$t!==void 0&&(H.viewport=$t),z.toneMapping=Vt}function Qs(v,O,q){const H=O.isScene===!0?O.overrideMaterial:null;for(let G=0,xt=v.length;G<xt;G++){const Tt=v[G],{object:gt,geometry:Ct,group:Lt}=Tt;let Vt=Tt.material;Vt.allowOverride===!0&&H!==null&&(Vt=H),gt.layers.test(q.layers)&&Zl(gt,O,q,Ct,Vt,Lt)}}function Zl(v,O,q,H,G,xt){v.onBeforeRender(z,O,q,H,G,xt),v.modelViewMatrix.multiplyMatrices(q.matrixWorldInverse,v.matrixWorld),v.normalMatrix.getNormalMatrix(v.modelViewMatrix),G.onBeforeRender(z,O,q,H,v,xt),G.transparent===!0&&G.side===Bn&&G.forceSinglePass===!1?(G.side=Ke,G.needsUpdate=!0,z.renderBufferDirect(q,O,H,G,v,xt),G.side=ci,G.needsUpdate=!0,z.renderBufferDirect(q,O,H,G,v,xt),G.side=Bn):z.renderBufferDirect(q,O,H,G,v,xt),v.onAfterRender(z,O,q,H,G,xt)}function tr(v,O,q){O.isScene!==!0&&(O=pe);const H=x.get(v),G=b.state.lights,xt=b.state.shadowsArray,Tt=G.state.version,gt=lt.getParameters(v,G.state,xt,O,q,b.state.lightProbeGridArray),Ct=lt.getProgramCacheKey(gt);let Lt=H.programs;H.environment=v.isMeshStandardMaterial||v.isMeshLambertMaterial||v.isMeshPhongMaterial?O.environment:null,H.fog=O.fog;const Vt=v.isMeshStandardMaterial||v.isMeshLambertMaterial&&!v.envMap||v.isMeshPhongMaterial&&!v.envMap;H.envMap=R.get(v.envMap||H.environment,Vt),H.envMapRotation=H.environment!==null&&v.envMap===null?O.environmentRotation:v.envMapRotation,Lt===void 0&&(v.addEventListener("dispose",xe),Lt=new Map,H.programs=Lt);let $t=Lt.get(Ct);if($t!==void 0){if(H.currentProgram===$t&&H.lightsStateVersion===Tt)return Ql(v,gt),$t}else gt.uniforms=lt.getUniforms(v),V!==null&&v.isNodeMaterial&&V.build(v,q,gt),v.onBeforeCompile(gt,z),$t=lt.acquireProgram(gt,Ct),Lt.set(Ct,$t),H.uniforms=gt.uniforms;const Dt=H.uniforms;return(!v.isShaderMaterial&&!v.isRawShaderMaterial||v.clipping===!0)&&(Dt.clippingPlanes=_t.uniform),Ql(v,gt),H.needsLights=Ku(v),H.lightsStateVersion=Tt,H.needsLights&&(Dt.ambientLightColor.value=G.state.ambient,Dt.lightProbe.value=G.state.probe,Dt.directionalLights.value=G.state.directional,Dt.directionalLightShadows.value=G.state.directionalShadow,Dt.spotLights.value=G.state.spot,Dt.spotLightShadows.value=G.state.spotShadow,Dt.rectAreaLights.value=G.state.rectArea,Dt.ltc_1.value=G.state.rectAreaLTC1,Dt.ltc_2.value=G.state.rectAreaLTC2,Dt.pointLights.value=G.state.point,Dt.pointLightShadows.value=G.state.pointShadow,Dt.hemisphereLights.value=G.state.hemi,Dt.directionalShadowMatrix.value=G.state.directionalShadowMatrix,Dt.spotLightMatrix.value=G.state.spotLightMatrix,Dt.spotLightMap.value=G.state.spotLightMap,Dt.pointShadowMatrix.value=G.state.pointShadowMatrix),H.lightProbeGrid=b.state.lightProbeGridArray.length>0,H.currentProgram=$t,H.uniformsList=null,$t}function Jl(v){if(v.uniformsList===null){const O=v.currentProgram.getUniforms();v.uniformsList=Xr.seqWithValue(O.seq,v.uniforms)}return v.uniformsList}function Ql(v,O){const q=x.get(v);q.outputColorSpace=O.outputColorSpace,q.batching=O.batching,q.batchingColor=O.batchingColor,q.instancing=O.instancing,q.instancingColor=O.instancingColor,q.instancingMorph=O.instancingMorph,q.skinning=O.skinning,q.morphTargets=O.morphTargets,q.morphNormals=O.morphNormals,q.morphColors=O.morphColors,q.morphTargetsCount=O.morphTargetsCount,q.numClippingPlanes=O.numClippingPlanes,q.numIntersection=O.numClipIntersection,q.vertexAlphas=O.vertexAlphas,q.vertexTangents=O.vertexTangents,q.toneMapping=O.toneMapping}function qu(v,O){if(v.length===0)return null;if(v.length===1)return v[0].texture!==null?v[0]:null;T.setFromMatrixPosition(O.matrixWorld);for(let q=0,H=v.length;q<H;q++){const G=v[q];if(G.texture!==null&&G.boundingBox.containsPoint(T))return G}return null}function Yu(v,O,q,H,G){O.isScene!==!0&&(O=pe),p.resetTextureUnits();const xt=O.fog,Tt=H.isMeshStandardMaterial||H.isMeshLambertMaterial||H.isMeshPhongMaterial?O.environment:null,gt=B===null?z.outputColorSpace:B.isXRRenderTarget===!0?B.texture.colorSpace:jt.workingColorSpace,Ct=H.isMeshStandardMaterial||H.isMeshLambertMaterial&&!H.envMap||H.isMeshPhongMaterial&&!H.envMap,Lt=R.get(H.envMap||Tt,Ct),Vt=H.vertexColors===!0&&!!q.attributes.color&&q.attributes.color.itemSize===4,$t=!!q.attributes.tangent&&(!!H.normalMap||H.anisotropy>0),Dt=!!q.morphAttributes.position,ae=!!q.morphAttributes.normal,ve=!!q.morphAttributes.color;let ge=Cn;H.toneMapped&&(B===null||B.isXRRenderTarget===!0)&&(ge=z.toneMapping);const he=q.morphAttributes.position||q.morphAttributes.normal||q.morphAttributes.color,Ne=he!==void 0?he.length:0,Et=x.get(H),Ze=b.state.lights;if(qt===!0&&(Ut===!0||v!==$)){const de=v===$&&H.id===W;_t.setState(H,v,de)}let Jt=!1;H.version===Et.__version?(Et.needsLights&&Et.lightsStateVersion!==Ze.state.version||Et.outputColorSpace!==gt||G.isBatchedMesh&&Et.batching===!1||!G.isBatchedMesh&&Et.batching===!0||G.isBatchedMesh&&Et.batchingColor===!0&&G.colorTexture===null||G.isBatchedMesh&&Et.batchingColor===!1&&G.colorTexture!==null||G.isInstancedMesh&&Et.instancing===!1||!G.isInstancedMesh&&Et.instancing===!0||G.isSkinnedMesh&&Et.skinning===!1||!G.isSkinnedMesh&&Et.skinning===!0||G.isInstancedMesh&&Et.instancingColor===!0&&G.instanceColor===null||G.isInstancedMesh&&Et.instancingColor===!1&&G.instanceColor!==null||G.isInstancedMesh&&Et.instancingMorph===!0&&G.morphTexture===null||G.isInstancedMesh&&Et.instancingMorph===!1&&G.morphTexture!==null||Et.envMap!==Lt||H.fog===!0&&Et.fog!==xt||Et.numClippingPlanes!==void 0&&(Et.numClippingPlanes!==_t.numPlanes||Et.numIntersection!==_t.numIntersection)||Et.vertexAlphas!==Vt||Et.vertexTangents!==$t||Et.morphTargets!==Dt||Et.morphNormals!==ae||Et.morphColors!==ve||Et.toneMapping!==ge||Et.morphTargetsCount!==Ne||!!Et.lightProbeGrid!=b.state.lightProbeGridArray.length>0)&&(Jt=!0):(Jt=!0,Et.__version=H.version);let en=Et.currentProgram;Jt===!0&&(en=tr(H,O,G),V&&H.isNodeMaterial&&V.onUpdateProgram(H,en,Et));let Mn=!1,Yn=!1,Bi=!1;const ue=en.getUniforms(),Me=Et.uniforms;if(Y.useProgram(en.program)&&(Mn=!0,Yn=!0,Bi=!0),H.id!==W&&(W=H.id,Yn=!0),Et.needsLights){const de=qu(b.state.lightProbeGridArray,G);Et.lightProbeGrid!==de&&(Et.lightProbeGrid=de,Yn=!0)}if(Mn||$!==v){Y.buffers.depth.getReversed()&&v.reversedDepth!==!0&&(v._reversedDepth=!0,v.updateProjectionMatrix()),ue.setValue(S,"projectionMatrix",v.projectionMatrix),ue.setValue(S,"viewMatrix",v.matrixWorldInverse);const Kn=ue.map.cameraPosition;Kn!==void 0&&Kn.setValue(S,ce.setFromMatrixPosition(v.matrixWorld)),ot.logarithmicDepthBuffer&&ue.setValue(S,"logDepthBufFC",2/(Math.log(v.far+1)/Math.LN2)),(H.isMeshPhongMaterial||H.isMeshToonMaterial||H.isMeshLambertMaterial||H.isMeshBasicMaterial||H.isMeshStandardMaterial||H.isShaderMaterial)&&ue.setValue(S,"isOrthographic",v.isOrthographicCamera===!0),$!==v&&($=v,Yn=!0,Bi=!0)}if(Et.needsLights&&(Ze.state.directionalShadowMap.length>0&&ue.setValue(S,"directionalShadowMap",Ze.state.directionalShadowMap,p),Ze.state.spotShadowMap.length>0&&ue.setValue(S,"spotShadowMap",Ze.state.spotShadowMap,p),Ze.state.pointShadowMap.length>0&&ue.setValue(S,"pointShadowMap",Ze.state.pointShadowMap,p)),G.isSkinnedMesh){ue.setOptional(S,G,"bindMatrix"),ue.setOptional(S,G,"bindMatrixInverse");const de=G.skeleton;de&&(de.boneTexture===null&&de.computeBoneTexture(),ue.setValue(S,"boneTexture",de.boneTexture,p))}G.isBatchedMesh&&(ue.setOptional(S,G,"batchingTexture"),ue.setValue(S,"batchingTexture",G._matricesTexture,p),ue.setOptional(S,G,"batchingIdTexture"),ue.setValue(S,"batchingIdTexture",G._indirectTexture,p),ue.setOptional(S,G,"batchingColorTexture"),G._colorsTexture!==null&&ue.setValue(S,"batchingColorTexture",G._colorsTexture,p));const jn=q.morphAttributes;if((jn.position!==void 0||jn.normal!==void 0||jn.color!==void 0)&&Ft.update(G,q,en),(Yn||Et.receiveShadow!==G.receiveShadow)&&(Et.receiveShadow=G.receiveShadow,ue.setValue(S,"receiveShadow",G.receiveShadow)),(H.isMeshStandardMaterial||H.isMeshLambertMaterial||H.isMeshPhongMaterial)&&H.envMap===null&&O.environment!==null&&(Me.envMapIntensity.value=O.environmentIntensity),Me.dfgLUT!==void 0&&(Me.dfgLUT.value=_0()),Yn){if(ue.setValue(S,"toneMappingExposure",z.toneMappingExposure),Et.needsLights&&ju(Me,Bi),xt&&H.fog===!0&&I.refreshFogUniforms(Me,xt),I.refreshMaterialUniforms(Me,H,C,Qt,b.state.transmissionRenderTarget[v.id]),Et.needsLights&&Et.lightProbeGrid){const de=Et.lightProbeGrid;Me.probesSH.value=de.texture,Me.probesMin.value.copy(de.boundingBox.min),Me.probesMax.value.copy(de.boundingBox.max),Me.probesResolution.value.copy(de.resolution)}Xr.upload(S,Jl(Et),Me,p)}if(H.isShaderMaterial&&H.uniformsNeedUpdate===!0&&(Xr.upload(S,Jl(Et),Me,p),H.uniformsNeedUpdate=!1),H.isSpriteMaterial&&ue.setValue(S,"center",G.center),ue.setValue(S,"modelViewMatrix",G.modelViewMatrix),ue.setValue(S,"normalMatrix",G.normalMatrix),ue.setValue(S,"modelMatrix",G.matrixWorld),H.uniformsGroups!==void 0){const de=H.uniformsGroups;for(let Kn=0,zi=de.length;Kn<zi;Kn++){const tc=de[Kn];tt.update(tc,en),tt.bind(tc,en)}}return en}function ju(v,O){v.ambientLightColor.needsUpdate=O,v.lightProbe.needsUpdate=O,v.directionalLights.needsUpdate=O,v.directionalLightShadows.needsUpdate=O,v.pointLights.needsUpdate=O,v.pointLightShadows.needsUpdate=O,v.spotLights.needsUpdate=O,v.spotLightShadows.needsUpdate=O,v.rectAreaLights.needsUpdate=O,v.hemisphereLights.needsUpdate=O}function Ku(v){return v.isMeshLambertMaterial||v.isMeshToonMaterial||v.isMeshPhongMaterial||v.isMeshStandardMaterial||v.isShadowMaterial||v.isShaderMaterial&&v.lights===!0}this.getActiveCubeFace=function(){return J},this.getActiveMipmapLevel=function(){return K},this.getRenderTarget=function(){return B},this.setRenderTargetTextures=function(v,O,q){const H=x.get(v);H.__autoAllocateDepthBuffer=v.resolveDepthBuffer===!1,H.__autoAllocateDepthBuffer===!1&&(H.__useRenderToTexture=!1),x.get(v.texture).__webglTexture=O,x.get(v.depthTexture).__webglTexture=H.__autoAllocateDepthBuffer?void 0:q,H.__hasExternalTextures=!0},this.setRenderTargetFramebuffer=function(v,O){const q=x.get(v);q.__webglFramebuffer=O,q.__useDefaultFramebuffer=O===void 0};const Zu=S.createFramebuffer();this.setRenderTarget=function(v,O=0,q=0){B=v,J=O,K=q;let H=null,G=!1,xt=!1;if(v){const gt=x.get(v);if(gt.__useDefaultFramebuffer!==void 0){Y.bindFramebuffer(S.FRAMEBUFFER,gt.__webglFramebuffer),et.copy(v.viewport),at.copy(v.scissor),ct=v.scissorTest,Y.viewport(et),Y.scissor(at),Y.setScissorTest(ct),W=-1;return}else if(gt.__webglFramebuffer===void 0)p.setupRenderTarget(v);else if(gt.__hasExternalTextures)p.rebindTextures(v,x.get(v.texture).__webglTexture,x.get(v.depthTexture).__webglTexture);else if(v.depthBuffer){const Vt=v.depthTexture;if(gt.__boundDepthTexture!==Vt){if(Vt!==null&&x.has(Vt)&&(v.width!==Vt.image.width||v.height!==Vt.image.height))throw new Error("WebGLRenderTarget: Attached DepthTexture is initialized to the incorrect size.");p.setupDepthRenderbuffer(v)}}const Ct=v.texture;(Ct.isData3DTexture||Ct.isDataArrayTexture||Ct.isCompressedArrayTexture)&&(xt=!0);const Lt=x.get(v).__webglFramebuffer;v.isWebGLCubeRenderTarget?(Array.isArray(Lt[O])?H=Lt[O][q]:H=Lt[O],G=!0):v.samples>0&&p.useMultisampledRTT(v)===!1?H=x.get(v).__webglMultisampledFramebuffer:Array.isArray(Lt)?H=Lt[q]:H=Lt,et.copy(v.viewport),at.copy(v.scissor),ct=v.scissorTest}else et.copy(st).multiplyScalar(C).floor(),at.copy(Mt).multiplyScalar(C).floor(),ct=bt;if(q!==0&&(H=Zu),Y.bindFramebuffer(S.FRAMEBUFFER,H)&&Y.drawBuffers(v,H),Y.viewport(et),Y.scissor(at),Y.setScissorTest(ct),G){const gt=x.get(v.texture);S.framebufferTexture2D(S.FRAMEBUFFER,S.COLOR_ATTACHMENT0,S.TEXTURE_CUBE_MAP_POSITIVE_X+O,gt.__webglTexture,q)}else if(xt){const gt=O;for(let Ct=0;Ct<v.textures.length;Ct++){const Lt=x.get(v.textures[Ct]);S.framebufferTextureLayer(S.FRAMEBUFFER,S.COLOR_ATTACHMENT0+Ct,Lt.__webglTexture,q,gt)}}else if(v!==null&&q!==0){const gt=x.get(v.texture);S.framebufferTexture2D(S.FRAMEBUFFER,S.COLOR_ATTACHMENT0,S.TEXTURE_2D,gt.__webglTexture,q)}W=-1},this.readRenderTargetPixels=function(v,O,q,H,G,xt,Tt,gt=0){if(!(v&&v.isWebGLRenderTarget)){Kt("WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.");return}let Ct=x.get(v).__webglFramebuffer;if(v.isWebGLCubeRenderTarget&&Tt!==void 0&&(Ct=Ct[Tt]),Ct){Y.bindFramebuffer(S.FRAMEBUFFER,Ct);try{const Lt=v.textures[gt],Vt=Lt.format,$t=Lt.type;if(v.textures.length>1&&S.readBuffer(S.COLOR_ATTACHMENT0+gt),!ot.textureFormatReadable(Vt)){Kt("WebGLRenderer.readRenderTargetPixels: renderTarget is not in RGBA or implementation defined format.");return}if(!ot.textureTypeReadable($t)){Kt("WebGLRenderer.readRenderTargetPixels: renderTarget is not in UnsignedByteType or implementation defined type.");return}O>=0&&O<=v.width-H&&q>=0&&q<=v.height-G&&S.readPixels(O,q,H,G,L.convert(Vt),L.convert($t),xt)}finally{const Lt=B!==null?x.get(B).__webglFramebuffer:null;Y.bindFramebuffer(S.FRAMEBUFFER,Lt)}}},this.readRenderTargetPixelsAsync=async function(v,O,q,H,G,xt,Tt,gt=0){if(!(v&&v.isWebGLRenderTarget))throw new Error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.");let Ct=x.get(v).__webglFramebuffer;if(v.isWebGLCubeRenderTarget&&Tt!==void 0&&(Ct=Ct[Tt]),Ct)if(O>=0&&O<=v.width-H&&q>=0&&q<=v.height-G){Y.bindFramebuffer(S.FRAMEBUFFER,Ct);const Lt=v.textures[gt],Vt=Lt.format,$t=Lt.type;if(v.textures.length>1&&S.readBuffer(S.COLOR_ATTACHMENT0+gt),!ot.textureFormatReadable(Vt))throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in RGBA or implementation defined format.");if(!ot.textureTypeReadable($t))throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in UnsignedByteType or implementation defined type.");const Dt=S.createBuffer();S.bindBuffer(S.PIXEL_PACK_BUFFER,Dt),S.bufferData(S.PIXEL_PACK_BUFFER,xt.byteLength,S.STREAM_READ),S.readPixels(O,q,H,G,L.convert(Vt),L.convert($t),0);const ae=B!==null?x.get(B).__webglFramebuffer:null;Y.bindFramebuffer(S.FRAMEBUFFER,ae);const ve=S.fenceSync(S.SYNC_GPU_COMMANDS_COMPLETE,0);return S.flush(),await Nd(S,ve,4),S.bindBuffer(S.PIXEL_PACK_BUFFER,Dt),S.getBufferSubData(S.PIXEL_PACK_BUFFER,0,xt),S.deleteBuffer(Dt),S.deleteSync(ve),xt}else throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: requested read bounds are out of range.")},this.copyFramebufferToTexture=function(v,O=null,q=0){const H=Math.pow(2,-q),G=Math.floor(v.image.width*H),xt=Math.floor(v.image.height*H),Tt=O!==null?O.x:0,gt=O!==null?O.y:0;p.setTexture2D(v,0),S.copyTexSubImage2D(S.TEXTURE_2D,q,0,0,Tt,gt,G,xt),Y.unbindTexture()};const Ju=S.createFramebuffer(),Qu=S.createFramebuffer();this.copyTextureToTexture=function(v,O,q=null,H=null,G=0,xt=0){let Tt,gt,Ct,Lt,Vt,$t,Dt,ae,ve;const ge=v.isCompressedTexture?v.mipmaps[xt]:v.image;if(q!==null)Tt=q.max.x-q.min.x,gt=q.max.y-q.min.y,Ct=q.isBox3?q.max.z-q.min.z:1,Lt=q.min.x,Vt=q.min.y,$t=q.isBox3?q.min.z:0;else{const Me=Math.pow(2,-G);Tt=Math.floor(ge.width*Me),gt=Math.floor(ge.height*Me),v.isDataArrayTexture?Ct=ge.depth:v.isData3DTexture?Ct=Math.floor(ge.depth*Me):Ct=1,Lt=0,Vt=0,$t=0}H!==null?(Dt=H.x,ae=H.y,ve=H.z):(Dt=0,ae=0,ve=0);const he=L.convert(O.format),Ne=L.convert(O.type);let Et;O.isData3DTexture?(p.setTexture3D(O,0),Et=S.TEXTURE_3D):O.isDataArrayTexture||O.isCompressedArrayTexture?(p.setTexture2DArray(O,0),Et=S.TEXTURE_2D_ARRAY):(p.setTexture2D(O,0),Et=S.TEXTURE_2D),Y.activeTexture(S.TEXTURE0),Y.pixelStorei(S.UNPACK_FLIP_Y_WEBGL,O.flipY),Y.pixelStorei(S.UNPACK_PREMULTIPLY_ALPHA_WEBGL,O.premultiplyAlpha),Y.pixelStorei(S.UNPACK_ALIGNMENT,O.unpackAlignment);const Ze=Y.getParameter(S.UNPACK_ROW_LENGTH),Jt=Y.getParameter(S.UNPACK_IMAGE_HEIGHT),en=Y.getParameter(S.UNPACK_SKIP_PIXELS),Mn=Y.getParameter(S.UNPACK_SKIP_ROWS),Yn=Y.getParameter(S.UNPACK_SKIP_IMAGES);Y.pixelStorei(S.UNPACK_ROW_LENGTH,ge.width),Y.pixelStorei(S.UNPACK_IMAGE_HEIGHT,ge.height),Y.pixelStorei(S.UNPACK_SKIP_PIXELS,Lt),Y.pixelStorei(S.UNPACK_SKIP_ROWS,Vt),Y.pixelStorei(S.UNPACK_SKIP_IMAGES,$t);const Bi=v.isDataArrayTexture||v.isData3DTexture,ue=O.isDataArrayTexture||O.isData3DTexture;if(v.isDepthTexture){const Me=x.get(v),jn=x.get(O),de=x.get(Me.__renderTarget),Kn=x.get(jn.__renderTarget);Y.bindFramebuffer(S.READ_FRAMEBUFFER,de.__webglFramebuffer),Y.bindFramebuffer(S.DRAW_FRAMEBUFFER,Kn.__webglFramebuffer);for(let zi=0;zi<Ct;zi++)Bi&&(S.framebufferTextureLayer(S.READ_FRAMEBUFFER,S.COLOR_ATTACHMENT0,x.get(v).__webglTexture,G,$t+zi),S.framebufferTextureLayer(S.DRAW_FRAMEBUFFER,S.COLOR_ATTACHMENT0,x.get(O).__webglTexture,xt,ve+zi)),S.blitFramebuffer(Lt,Vt,Tt,gt,Dt,ae,Tt,gt,S.DEPTH_BUFFER_BIT,S.NEAREST);Y.bindFramebuffer(S.READ_FRAMEBUFFER,null),Y.bindFramebuffer(S.DRAW_FRAMEBUFFER,null)}else if(G!==0||v.isRenderTargetTexture||x.has(v)){const Me=x.get(v),jn=x.get(O);Y.bindFramebuffer(S.READ_FRAMEBUFFER,Ju),Y.bindFramebuffer(S.DRAW_FRAMEBUFFER,Qu);for(let de=0;de<Ct;de++)Bi?S.framebufferTextureLayer(S.READ_FRAMEBUFFER,S.COLOR_ATTACHMENT0,Me.__webglTexture,G,$t+de):S.framebufferTexture2D(S.READ_FRAMEBUFFER,S.COLOR_ATTACHMENT0,S.TEXTURE_2D,Me.__webglTexture,G),ue?S.framebufferTextureLayer(S.DRAW_FRAMEBUFFER,S.COLOR_ATTACHMENT0,jn.__webglTexture,xt,ve+de):S.framebufferTexture2D(S.DRAW_FRAMEBUFFER,S.COLOR_ATTACHMENT0,S.TEXTURE_2D,jn.__webglTexture,xt),G!==0?S.blitFramebuffer(Lt,Vt,Tt,gt,Dt,ae,Tt,gt,S.COLOR_BUFFER_BIT,S.NEAREST):ue?S.copyTexSubImage3D(Et,xt,Dt,ae,ve+de,Lt,Vt,Tt,gt):S.copyTexSubImage2D(Et,xt,Dt,ae,Lt,Vt,Tt,gt);Y.bindFramebuffer(S.READ_FRAMEBUFFER,null),Y.bindFramebuffer(S.DRAW_FRAMEBUFFER,null)}else ue?v.isDataTexture||v.isData3DTexture?S.texSubImage3D(Et,xt,Dt,ae,ve,Tt,gt,Ct,he,Ne,ge.data):O.isCompressedArrayTexture?S.compressedTexSubImage3D(Et,xt,Dt,ae,ve,Tt,gt,Ct,he,ge.data):S.texSubImage3D(Et,xt,Dt,ae,ve,Tt,gt,Ct,he,Ne,ge):v.isDataTexture?S.texSubImage2D(S.TEXTURE_2D,xt,Dt,ae,Tt,gt,he,Ne,ge.data):v.isCompressedTexture?S.compressedTexSubImage2D(S.TEXTURE_2D,xt,Dt,ae,ge.width,ge.height,he,ge.data):S.texSubImage2D(S.TEXTURE_2D,xt,Dt,ae,Tt,gt,he,Ne,ge);Y.pixelStorei(S.UNPACK_ROW_LENGTH,Ze),Y.pixelStorei(S.UNPACK_IMAGE_HEIGHT,Jt),Y.pixelStorei(S.UNPACK_SKIP_PIXELS,en),Y.pixelStorei(S.UNPACK_SKIP_ROWS,Mn),Y.pixelStorei(S.UNPACK_SKIP_IMAGES,Yn),xt===0&&O.generateMipmaps&&S.generateMipmap(Et),Y.unbindTexture()},this.initRenderTarget=function(v){x.get(v).__webglFramebuffer===void 0&&p.setupRenderTarget(v)},this.initTexture=function(v){v.isCubeTexture?p.setTextureCube(v,0):v.isData3DTexture?p.setTexture3D(v,0):v.isDataArrayTexture||v.isCompressedArrayTexture?p.setTexture2DArray(v,0):p.setTexture2D(v,0),Y.unbindTexture()},this.resetState=function(){J=0,K=0,B=null,Y.reset(),ut.reset()},typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}get coordinateSystem(){return wn}get outputColorSpace(){return this._outputColorSpace}set outputColorSpace(t){this._outputColorSpace=t;const e=this.getContext();e.drawingBufferColorSpace=jt._getDrawingBufferColorSpace(t),e.unpackColorSpace=jt._getUnpackColorSpace()}}const sh={type:"change"},Pl={type:"start"},au={type:"end"},Lr=new _a,rh=new ii,v0=Math.cos(70*Bd.DEG2RAD),we=new F,qe=2*Math.PI,le={NONE:-1,ROTATE:0,DOLLY:1,PAN:2,TOUCH_ROTATE:3,TOUCH_PAN:4,TOUCH_DOLLY_PAN:5,TOUCH_DOLLY_ROTATE:6},ro=1e-6;class M0 extends yf{constructor(t,e=null){super(t,e),this.state=le.NONE,this.target=new F,this.cursor=new F,this.minDistance=0,this.maxDistance=1/0,this.minZoom=0,this.maxZoom=1/0,this.minTargetRadius=0,this.maxTargetRadius=1/0,this.minPolarAngle=0,this.maxPolarAngle=Math.PI,this.minAzimuthAngle=-1/0,this.maxAzimuthAngle=1/0,this.enableDamping=!1,this.dampingFactor=.05,this.enableZoom=!0,this.zoomSpeed=1,this.enableRotate=!0,this.rotateSpeed=1,this.keyRotateSpeed=1,this.enablePan=!0,this.panSpeed=1,this.screenSpacePanning=!0,this.keyPanSpeed=7,this.zoomToCursor=!1,this.autoRotate=!1,this.autoRotateSpeed=2,this.keys={LEFT:"ArrowLeft",UP:"ArrowUp",RIGHT:"ArrowRight",BOTTOM:"ArrowDown"},this.mouseButtons={LEFT:hs.ROTATE,MIDDLE:hs.DOLLY,RIGHT:hs.PAN},this.touches={ONE:rs.ROTATE,TWO:rs.DOLLY_PAN},this.target0=this.target.clone(),this.position0=this.object.position.clone(),this.zoom0=this.object.zoom,this._cursorStyle="auto",this._domElementKeyEvents=null,this._lastPosition=new F,this._lastQuaternion=new hi,this._lastTargetPosition=new F,this._quat=new hi().setFromUnitVectors(t.up,new F(0,1,0)),this._quatInverse=this._quat.clone().invert(),this._spherical=new Ic,this._sphericalDelta=new Ic,this._scale=1,this._panOffset=new F,this._rotateStart=new Nt,this._rotateEnd=new Nt,this._rotateDelta=new Nt,this._panStart=new Nt,this._panEnd=new Nt,this._panDelta=new Nt,this._dollyStart=new Nt,this._dollyEnd=new Nt,this._dollyDelta=new Nt,this._dollyDirection=new F,this._mouse=new Nt,this._performCursorZoom=!1,this._pointers=[],this._pointerPositions={},this._controlActive=!1,this._onPointerMove=y0.bind(this),this._onPointerDown=S0.bind(this),this._onPointerUp=E0.bind(this),this._onContextMenu=P0.bind(this),this._onMouseWheel=A0.bind(this),this._onKeyDown=w0.bind(this),this._onTouchStart=R0.bind(this),this._onTouchMove=C0.bind(this),this._onMouseDown=b0.bind(this),this._onMouseMove=T0.bind(this),this._interceptControlDown=L0.bind(this),this._interceptControlUp=D0.bind(this),this.domElement!==null&&this.connect(this.domElement),this.update()}set cursorStyle(t){this._cursorStyle=t,t==="grab"?this.domElement.style.cursor="grab":this.domElement.style.cursor="auto"}get cursorStyle(){return this._cursorStyle}connect(t){super.connect(t),this.domElement.addEventListener("pointerdown",this._onPointerDown),this.domElement.addEventListener("pointercancel",this._onPointerUp),this.domElement.addEventListener("contextmenu",this._onContextMenu),this.domElement.addEventListener("wheel",this._onMouseWheel,{passive:!1}),this.domElement.getRootNode().addEventListener("keydown",this._interceptControlDown,{passive:!0,capture:!0}),this.domElement.style.touchAction="none"}disconnect(){this.domElement.removeEventListener("pointerdown",this._onPointerDown),this.domElement.ownerDocument.removeEventListener("pointermove",this._onPointerMove),this.domElement.ownerDocument.removeEventListener("pointerup",this._onPointerUp),this.domElement.removeEventListener("pointercancel",this._onPointerUp),this.domElement.removeEventListener("wheel",this._onMouseWheel),this.domElement.removeEventListener("contextmenu",this._onContextMenu),this.stopListenToKeyEvents(),this.domElement.getRootNode().removeEventListener("keydown",this._interceptControlDown,{capture:!0}),this.domElement.style.touchAction=""}dispose(){this.disconnect()}getPolarAngle(){return this._spherical.phi}getAzimuthalAngle(){return this._spherical.theta}getDistance(){return this.object.position.distanceTo(this.target)}listenToKeyEvents(t){t.addEventListener("keydown",this._onKeyDown),this._domElementKeyEvents=t}stopListenToKeyEvents(){this._domElementKeyEvents!==null&&(this._domElementKeyEvents.removeEventListener("keydown",this._onKeyDown),this._domElementKeyEvents=null)}saveState(){this.target0.copy(this.target),this.position0.copy(this.object.position),this.zoom0=this.object.zoom}reset(){this.target.copy(this.target0),this.object.position.copy(this.position0),this.object.zoom=this.zoom0,this.object.updateProjectionMatrix(),this.dispatchEvent(sh),this.update(),this.state=le.NONE}pan(t,e){this._pan(t,e),this.update()}dollyIn(t){this._dollyIn(t),this.update()}dollyOut(t){this._dollyOut(t),this.update()}rotateLeft(t){this._rotateLeft(t),this.update()}rotateUp(t){this._rotateUp(t),this.update()}update(t=null){const e=this.object.position;we.copy(e).sub(this.target),we.applyQuaternion(this._quat),this._spherical.setFromVector3(we),this.autoRotate&&this.state===le.NONE&&this._rotateLeft(this._getAutoRotationAngle(t)),this.enableDamping?(this._spherical.theta+=this._sphericalDelta.theta*this.dampingFactor,this._spherical.phi+=this._sphericalDelta.phi*this.dampingFactor):(this._spherical.theta+=this._sphericalDelta.theta,this._spherical.phi+=this._sphericalDelta.phi);let i=this.minAzimuthAngle,s=this.maxAzimuthAngle;isFinite(i)&&isFinite(s)&&(i<-Math.PI?i+=qe:i>Math.PI&&(i-=qe),s<-Math.PI?s+=qe:s>Math.PI&&(s-=qe),i<=s?this._spherical.theta=Math.max(i,Math.min(s,this._spherical.theta)):this._spherical.theta=this._spherical.theta>(i+s)/2?Math.max(i,this._spherical.theta):Math.min(s,this._spherical.theta)),this._spherical.phi=Math.max(this.minPolarAngle,Math.min(this.maxPolarAngle,this._spherical.phi)),this._spherical.makeSafe(),this.enableDamping===!0?this.target.addScaledVector(this._panOffset,this.dampingFactor):this.target.add(this._panOffset),this.target.sub(this.cursor),this.target.clampLength(this.minTargetRadius,this.maxTargetRadius),this.target.add(this.cursor);let r=!1;if(this.zoomToCursor&&this._performCursorZoom||this.object.isOrthographicCamera)this._spherical.radius=this._clampDistance(this._spherical.radius);else{const a=this._spherical.radius;this._spherical.radius=this._clampDistance(this._spherical.radius*this._scale),r=a!=this._spherical.radius}if(we.setFromSpherical(this._spherical),we.applyQuaternion(this._quatInverse),e.copy(this.target).add(we),this.object.lookAt(this.target),this.enableDamping===!0?(this._sphericalDelta.theta*=1-this.dampingFactor,this._sphericalDelta.phi*=1-this.dampingFactor,this._panOffset.multiplyScalar(1-this.dampingFactor)):(this._sphericalDelta.set(0,0,0),this._panOffset.set(0,0,0)),this.zoomToCursor&&this._performCursorZoom){let a=null;if(this.object.isPerspectiveCamera){const o=we.length();a=this._clampDistance(o*this._scale);const c=o-a;this.object.position.addScaledVector(this._dollyDirection,c),this.object.updateMatrixWorld(),r=!!c}else if(this.object.isOrthographicCamera){const o=new F(this._mouse.x,this._mouse.y,0);o.unproject(this.object);const c=this.object.zoom;this.object.zoom=Math.max(this.minZoom,Math.min(this.maxZoom,this.object.zoom/this._scale)),this.object.updateProjectionMatrix(),r=c!==this.object.zoom;const l=new F(this._mouse.x,this._mouse.y,0);l.unproject(this.object),this.object.position.sub(l).add(o),this.object.updateMatrixWorld(),a=we.length()}else console.warn("WARNING: OrbitControls.js encountered an unknown camera type - zoom to cursor disabled."),this.zoomToCursor=!1;a!==null&&(this.screenSpacePanning?this.target.set(0,0,-1).transformDirection(this.object.matrix).multiplyScalar(a).add(this.object.position):(Lr.origin.copy(this.object.position),Lr.direction.set(0,0,-1).transformDirection(this.object.matrix),Math.abs(this.object.up.dot(Lr.direction))<v0?this.object.lookAt(this.target):(rh.setFromNormalAndCoplanarPoint(this.object.up,this.target),Lr.intersectPlane(rh,this.target))))}else if(this.object.isOrthographicCamera){const a=this.object.zoom;this.object.zoom=Math.max(this.minZoom,Math.min(this.maxZoom,this.object.zoom/this._scale)),a!==this.object.zoom&&(this.object.updateProjectionMatrix(),r=!0)}return this._scale=1,this._performCursorZoom=!1,r||this._lastPosition.distanceToSquared(this.object.position)>ro||8*(1-this._lastQuaternion.dot(this.object.quaternion))>ro||this._lastTargetPosition.distanceToSquared(this.target)>ro?(this.dispatchEvent(sh),this._lastPosition.copy(this.object.position),this._lastQuaternion.copy(this.object.quaternion),this._lastTargetPosition.copy(this.target),!0):!1}_getAutoRotationAngle(t){return t!==null?qe/60*this.autoRotateSpeed*t:qe/60/60*this.autoRotateSpeed}_getZoomScale(t){const e=Math.abs(t*.01);return Math.pow(.95,this.zoomSpeed*e)}_rotateLeft(t){this._sphericalDelta.theta-=t}_rotateUp(t){this._sphericalDelta.phi-=t}_panLeft(t,e){we.setFromMatrixColumn(e,0),we.multiplyScalar(-t),this._panOffset.add(we)}_panUp(t,e){this.screenSpacePanning===!0?we.setFromMatrixColumn(e,1):(we.setFromMatrixColumn(e,0),we.crossVectors(this.object.up,we)),we.multiplyScalar(t),this._panOffset.add(we)}_pan(t,e){const i=this.domElement;if(this.object.isPerspectiveCamera){const s=this.object.position;we.copy(s).sub(this.target);let r=we.length();r*=Math.tan(this.object.fov/2*Math.PI/180),this._panLeft(2*t*r/i.clientHeight,this.object.matrix),this._panUp(2*e*r/i.clientHeight,this.object.matrix)}else this.object.isOrthographicCamera?(this._panLeft(t*(this.object.right-this.object.left)/this.object.zoom/i.clientWidth,this.object.matrix),this._panUp(e*(this.object.top-this.object.bottom)/this.object.zoom/i.clientHeight,this.object.matrix)):(console.warn("WARNING: OrbitControls.js encountered an unknown camera type - pan disabled."),this.enablePan=!1)}_dollyOut(t){this.object.isPerspectiveCamera||this.object.isOrthographicCamera?this._scale/=t:(console.warn("WARNING: OrbitControls.js encountered an unknown camera type - dolly/zoom disabled."),this.enableZoom=!1)}_dollyIn(t){this.object.isPerspectiveCamera||this.object.isOrthographicCamera?this._scale*=t:(console.warn("WARNING: OrbitControls.js encountered an unknown camera type - dolly/zoom disabled."),this.enableZoom=!1)}_updateZoomParameters(t,e){if(!this.zoomToCursor)return;this._performCursorZoom=!0;const i=this.domElement.getBoundingClientRect(),s=t-i.left,r=e-i.top,a=i.width,o=i.height;this._mouse.x=s/a*2-1,this._mouse.y=-(r/o)*2+1,this._dollyDirection.set(this._mouse.x,this._mouse.y,1).unproject(this.object).sub(this.object.position).normalize()}_clampDistance(t){return Math.max(this.minDistance,Math.min(this.maxDistance,t))}_handleMouseDownRotate(t){this._rotateStart.set(t.clientX,t.clientY)}_handleMouseDownDolly(t){this._updateZoomParameters(t.clientX,t.clientX),this._dollyStart.set(t.clientX,t.clientY)}_handleMouseDownPan(t){this._panStart.set(t.clientX,t.clientY)}_handleMouseMoveRotate(t){this._rotateEnd.set(t.clientX,t.clientY),this._rotateDelta.subVectors(this._rotateEnd,this._rotateStart).multiplyScalar(this.rotateSpeed);const e=this.domElement;this._rotateLeft(qe*this._rotateDelta.x/e.clientHeight),this._rotateUp(qe*this._rotateDelta.y/e.clientHeight),this._rotateStart.copy(this._rotateEnd),this.update()}_handleMouseMoveDolly(t){this._dollyEnd.set(t.clientX,t.clientY),this._dollyDelta.subVectors(this._dollyEnd,this._dollyStart),this._dollyDelta.y>0?this._dollyOut(this._getZoomScale(this._dollyDelta.y)):this._dollyDelta.y<0&&this._dollyIn(this._getZoomScale(this._dollyDelta.y)),this._dollyStart.copy(this._dollyEnd),this.update()}_handleMouseMovePan(t){this._panEnd.set(t.clientX,t.clientY),this._panDelta.subVectors(this._panEnd,this._panStart).multiplyScalar(this.panSpeed),this._pan(this._panDelta.x,this._panDelta.y),this._panStart.copy(this._panEnd),this.update()}_handleMouseWheel(t){this._updateZoomParameters(t.clientX,t.clientY),t.deltaY<0?this._dollyIn(this._getZoomScale(t.deltaY)):t.deltaY>0&&this._dollyOut(this._getZoomScale(t.deltaY)),this.update()}_handleKeyDown(t){let e=!1;switch(t.code){case this.keys.UP:t.ctrlKey||t.metaKey||t.shiftKey?this.enableRotate&&this._rotateUp(qe*this.keyRotateSpeed/this.domElement.clientHeight):this.enablePan&&this._pan(0,this.keyPanSpeed),e=!0;break;case this.keys.BOTTOM:t.ctrlKey||t.metaKey||t.shiftKey?this.enableRotate&&this._rotateUp(-qe*this.keyRotateSpeed/this.domElement.clientHeight):this.enablePan&&this._pan(0,-this.keyPanSpeed),e=!0;break;case this.keys.LEFT:t.ctrlKey||t.metaKey||t.shiftKey?this.enableRotate&&this._rotateLeft(qe*this.keyRotateSpeed/this.domElement.clientHeight):this.enablePan&&this._pan(this.keyPanSpeed,0),e=!0;break;case this.keys.RIGHT:t.ctrlKey||t.metaKey||t.shiftKey?this.enableRotate&&this._rotateLeft(-qe*this.keyRotateSpeed/this.domElement.clientHeight):this.enablePan&&this._pan(-this.keyPanSpeed,0),e=!0;break}e&&(t.preventDefault(),this.update())}_handleTouchStartRotate(t){if(this._pointers.length===1)this._rotateStart.set(t.pageX,t.pageY);else{const e=this._getSecondPointerPosition(t),i=.5*(t.pageX+e.x),s=.5*(t.pageY+e.y);this._rotateStart.set(i,s)}}_handleTouchStartPan(t){if(this._pointers.length===1)this._panStart.set(t.pageX,t.pageY);else{const e=this._getSecondPointerPosition(t),i=.5*(t.pageX+e.x),s=.5*(t.pageY+e.y);this._panStart.set(i,s)}}_handleTouchStartDolly(t){const e=this._getSecondPointerPosition(t),i=t.pageX-e.x,s=t.pageY-e.y,r=Math.sqrt(i*i+s*s);this._dollyStart.set(0,r)}_handleTouchStartDollyPan(t){this.enableZoom&&this._handleTouchStartDolly(t),this.enablePan&&this._handleTouchStartPan(t)}_handleTouchStartDollyRotate(t){this.enableZoom&&this._handleTouchStartDolly(t),this.enableRotate&&this._handleTouchStartRotate(t)}_handleTouchMoveRotate(t){if(this._pointers.length==1)this._rotateEnd.set(t.pageX,t.pageY);else{const i=this._getSecondPointerPosition(t),s=.5*(t.pageX+i.x),r=.5*(t.pageY+i.y);this._rotateEnd.set(s,r)}this._rotateDelta.subVectors(this._rotateEnd,this._rotateStart).multiplyScalar(this.rotateSpeed);const e=this.domElement;this._rotateLeft(qe*this._rotateDelta.x/e.clientHeight),this._rotateUp(qe*this._rotateDelta.y/e.clientHeight),this._rotateStart.copy(this._rotateEnd)}_handleTouchMovePan(t){if(this._pointers.length===1)this._panEnd.set(t.pageX,t.pageY);else{const e=this._getSecondPointerPosition(t),i=.5*(t.pageX+e.x),s=.5*(t.pageY+e.y);this._panEnd.set(i,s)}this._panDelta.subVectors(this._panEnd,this._panStart).multiplyScalar(this.panSpeed),this._pan(this._panDelta.x,this._panDelta.y),this._panStart.copy(this._panEnd)}_handleTouchMoveDolly(t){const e=this._getSecondPointerPosition(t),i=t.pageX-e.x,s=t.pageY-e.y,r=Math.sqrt(i*i+s*s);this._dollyEnd.set(0,r),this._dollyDelta.set(0,Math.pow(this._dollyEnd.y/this._dollyStart.y,this.zoomSpeed)),this._dollyOut(this._dollyDelta.y),this._dollyStart.copy(this._dollyEnd);const a=(t.pageX+e.x)*.5,o=(t.pageY+e.y)*.5;this._updateZoomParameters(a,o)}_handleTouchMoveDollyPan(t){this.enableZoom&&this._handleTouchMoveDolly(t),this.enablePan&&this._handleTouchMovePan(t)}_handleTouchMoveDollyRotate(t){this.enableZoom&&this._handleTouchMoveDolly(t),this.enableRotate&&this._handleTouchMoveRotate(t)}_addPointer(t){this._pointers.push(t.pointerId)}_removePointer(t){delete this._pointerPositions[t.pointerId];for(let e=0;e<this._pointers.length;e++)if(this._pointers[e]==t.pointerId){this._pointers.splice(e,1);return}}_isTrackingPointer(t){for(let e=0;e<this._pointers.length;e++)if(this._pointers[e]==t.pointerId)return!0;return!1}_trackPointer(t){let e=this._pointerPositions[t.pointerId];e===void 0&&(e=new Nt,this._pointerPositions[t.pointerId]=e),e.set(t.pageX,t.pageY)}_getSecondPointerPosition(t){const e=t.pointerId===this._pointers[0]?this._pointers[1]:this._pointers[0];return this._pointerPositions[e]}_customWheelEvent(t){const e=t.deltaMode,i={clientX:t.clientX,clientY:t.clientY,deltaY:t.deltaY};switch(e){case 1:i.deltaY*=16;break;case 2:i.deltaY*=100;break}return t.ctrlKey&&!this._controlActive&&(i.deltaY*=10),i}}function S0(n){this.enabled!==!1&&(this._pointers.length===0&&(this.domElement.setPointerCapture(n.pointerId),this.domElement.ownerDocument.addEventListener("pointermove",this._onPointerMove),this.domElement.ownerDocument.addEventListener("pointerup",this._onPointerUp)),!this._isTrackingPointer(n)&&(this._addPointer(n),n.pointerType==="touch"?this._onTouchStart(n):this._onMouseDown(n),this._cursorStyle==="grab"&&(this.domElement.style.cursor="grabbing")))}function y0(n){this.enabled!==!1&&(n.pointerType==="touch"?this._onTouchMove(n):this._onMouseMove(n))}function E0(n){switch(this._removePointer(n),this._pointers.length){case 0:this.domElement.releasePointerCapture(n.pointerId),this.domElement.ownerDocument.removeEventListener("pointermove",this._onPointerMove),this.domElement.ownerDocument.removeEventListener("pointerup",this._onPointerUp),this.dispatchEvent(au),this.state=le.NONE,this._cursorStyle==="grab"&&(this.domElement.style.cursor="grab");break;case 1:const t=this._pointers[0],e=this._pointerPositions[t];this._onTouchStart({pointerId:t,pageX:e.x,pageY:e.y});break}}function b0(n){let t;switch(n.button){case 0:t=this.mouseButtons.LEFT;break;case 1:t=this.mouseButtons.MIDDLE;break;case 2:t=this.mouseButtons.RIGHT;break;default:t=-1}switch(t){case hs.DOLLY:if(this.enableZoom===!1)return;this._handleMouseDownDolly(n),this.state=le.DOLLY;break;case hs.ROTATE:if(n.ctrlKey||n.metaKey||n.shiftKey){if(this.enablePan===!1)return;this._handleMouseDownPan(n),this.state=le.PAN}else{if(this.enableRotate===!1)return;this._handleMouseDownRotate(n),this.state=le.ROTATE}break;case hs.PAN:if(n.ctrlKey||n.metaKey||n.shiftKey){if(this.enableRotate===!1)return;this._handleMouseDownRotate(n),this.state=le.ROTATE}else{if(this.enablePan===!1)return;this._handleMouseDownPan(n),this.state=le.PAN}break;default:this.state=le.NONE}this.state!==le.NONE&&this.dispatchEvent(Pl)}function T0(n){switch(this.state){case le.ROTATE:if(this.enableRotate===!1)return;this._handleMouseMoveRotate(n);break;case le.DOLLY:if(this.enableZoom===!1)return;this._handleMouseMoveDolly(n);break;case le.PAN:if(this.enablePan===!1)return;this._handleMouseMovePan(n);break}}function A0(n){this.enabled===!1||this.enableZoom===!1||this.state!==le.NONE||(n.preventDefault(),this.dispatchEvent(Pl),this._handleMouseWheel(this._customWheelEvent(n)),this.dispatchEvent(au))}function w0(n){this.enabled!==!1&&this._handleKeyDown(n)}function R0(n){switch(this._trackPointer(n),this._pointers.length){case 1:switch(this.touches.ONE){case rs.ROTATE:if(this.enableRotate===!1)return;this._handleTouchStartRotate(n),this.state=le.TOUCH_ROTATE;break;case rs.PAN:if(this.enablePan===!1)return;this._handleTouchStartPan(n),this.state=le.TOUCH_PAN;break;default:this.state=le.NONE}break;case 2:switch(this.touches.TWO){case rs.DOLLY_PAN:if(this.enableZoom===!1&&this.enablePan===!1)return;this._handleTouchStartDollyPan(n),this.state=le.TOUCH_DOLLY_PAN;break;case rs.DOLLY_ROTATE:if(this.enableZoom===!1&&this.enableRotate===!1)return;this._handleTouchStartDollyRotate(n),this.state=le.TOUCH_DOLLY_ROTATE;break;default:this.state=le.NONE}break;default:this.state=le.NONE}this.state!==le.NONE&&this.dispatchEvent(Pl)}function C0(n){switch(this._trackPointer(n),this.state){case le.TOUCH_ROTATE:if(this.enableRotate===!1)return;this._handleTouchMoveRotate(n),this.update();break;case le.TOUCH_PAN:if(this.enablePan===!1)return;this._handleTouchMovePan(n),this.update();break;case le.TOUCH_DOLLY_PAN:if(this.enableZoom===!1&&this.enablePan===!1)return;this._handleTouchMoveDollyPan(n),this.update();break;case le.TOUCH_DOLLY_ROTATE:if(this.enableZoom===!1&&this.enableRotate===!1)return;this._handleTouchMoveDollyRotate(n),this.update();break;default:this.state=le.NONE}}function P0(n){this.enabled!==!1&&n.preventDefault()}function L0(n){n.key==="Control"&&(this._controlActive=!0,this.domElement.getRootNode().addEventListener("keyup",this._interceptControlUp,{passive:!0,capture:!0}))}function D0(n){n.key==="Control"&&(this._controlActive=!1,this.domElement.getRootNode().removeEventListener("keyup",this._interceptControlUp,{passive:!0,capture:!0}))}const Dr=new Map;let ah=null;async function ou(n,t={}){const e=lo(),i=e?.snapshot_id??e;i!==ah&&(Dr.clear(),ah=i);const s={};for(const r of new Set(n))if(t.signal?.throwIfAborted(),!!r){if(!Dr.has(r)){const a=await ml.record(r,t);if(lo()!==e)throw new DOMException("Graph changed","AbortError");Dr.set(r,a)}s[r]=Dr.get(r)}return s}const Ir={core:"#caa24a",hot:"#c8402e",warm:"#7e96cf",cold:"#6f7da0",frozen:"#8a8578"},oh={derived_from:["derived from","grounds this"],refers_to:["refers to","referenced here"],supersedes:["supersedes","superseded by"],verifies:["verifies","verified in"],causal:["led to","follows from"],part_of:["part of","contains"],temporal:["before","after"],contradicts:["contradicts","contradicts"],blocks:["blocks","blocked by"],targets:["targets","targeted by"]},I0=470,U0=300,ao=300,N0=120,Ye=n=>document.getElementById(n),fe=n=>(n||"").replace(/[&<>]/g,t=>({"&":"&amp;","<":"&lt;",">":"&gt;"})[t]),Ve=n=>(n||"").replace(/-20\d\d-\d\d-\d\d$/,"").replace(/-[0-9a-f]{6}$/,"").replace(/-/g," "),F0=n=>{const t=new Map;n.forEach(i=>{if(!i)return;const s=Ve(i.name);t.set(s,(t.get(s)||0)+1)});const e={};return n.forEach(i=>{if(!i)return;const s=Ve(i.name),r=t.get(s)>1?(i.name.match(/-([0-9a-f]{6})$/)||[])[1]:null;e[i.id]=r?`${s} · ${r}`:s}),e},lh=(n,t,e)=>Math.min(e,Math.max(t,n));function O0(n,{fmtDateTime:t}){const e=Ye("rworld"),i=Ye("rtrack"),s=Ye("rcanvas"),r=Ye("manuscript"),a={},o={};n.nodes.filter(S=>S.type!=="chain"||!n.chains.some(j=>j.id===S.id)).forEach(S=>{a[S.id]={...S},o[S.id]=[]});for(const S of n.edges)!a[S.src]||!a[S.dst]||(o[S.src].push({o:S.dst,t:S.type,dir:"out"}),o[S.dst].push({o:S.src,t:S.type,dir:"in"}));let c=null,l=[],u={},g=new Map,h=new Set,m=[],_=60,y=40,f=.62,d="trail";const E=S=>fe(S).replace(/\[\[([^\]]+)\]\]/g,(j,Q)=>`<span class="rmark">${Ve(Q)}</span>`).replace(/`([^`]+)`/g,"<code>$1</code>"),A=S=>(S||"").trim().split(/\n{2,}/).map(j=>`<p>${E(j.trim()).replace(/\n/g,"<br>")}</p>`).join(""),T=(S,j)=>{const Q=(S||"").replace(/\s+/g," ").trim();return Q.length>j?E(Q.slice(0,j).replace(/\s\S*$/,""))+"…":E(Q)},D=S=>S.created_secs?t(S.created_secs):"—",b=S=>{const j=document.createElement("div");return j.innerHTML=S.trim(),j.firstChild};let N=null,M=0;async function w(S){N?.abort();const j=new AbortController;N=j;let Q;try{Q=await ou(S,{signal:j.signal})}catch(Y){return Y.name==="AbortError"||window.dispatchEvent(new CustomEvent("kaeru-api-error",{detail:Y})),!1}let ot=!1;for(const Y in Q)a[Y]&&a[Y].body!==Q[Y]&&(a[Y].body=Q[Y],ot=!0);return ot}async function z(S){const j=n.chains.find(Q=>Q.id===S);return j?w(j.members||[]):!1}const P=S=>{const j=n.chains.find(Q=>(Q.members||[]).includes(S));return j?j.members:[S]},V=()=>{const S=[];for(const j in o)for(const Q of o[j])Q.dir==="out"&&Q.t==="derived_from"&&S.push({from:Q.o,to:j});return S};function J(){u={};const S=V(),j={};h=new Set(l.map(I=>I.id));for(let I=0;I<2;I++)for(const X of S)h.has(X.from)&&h.add(X.to),h.has(X.to)&&h.add(X.from);for(const I of S)h.has(I.from)&&h.has(I.to)&&(j[I.to]=j[I.to]||[]).push(I.from);for(let I=1;I<l.length;I++){const X=l[I].id;(j[X]||[]).length||(j[X]=[l[I-1].id])}const Q={},ot=(I,X)=>{if(Q[I]!=null)return Q[I];if(X.has(I))return 0;X.add(I);const dt=(j[I]||[]).filter(_t=>h.has(_t));return Q[I]=dt.length?Math.max(...dt.map(_t=>ot(_t,X)))+1:0,Q[I]};[...h].forEach(I=>ot(I,new Set));const Y={};[...h].forEach(I=>{(Y[Q[I]]=Y[Q[I]]||[]).push(I)});const vt={},x=new Set;let p=0;Object.keys(Y).map(Number).sort((I,X)=>I-X).forEach(I=>{Y[I].sort((X,dt)=>(a[X]?.created_secs||0)-(a[dt]?.created_secs||0)),Y[I].forEach(X=>{const dt=(j[X]||[]).find(_t=>vt[_t]!=null&&!x.has(_t));dt!=null?(vt[X]=vt[dt],x.add(dt)):vt[X]=p++})});const R=new Set(l.map(I=>I.id)),k=new Set([...R].map(I=>vt[I]).filter(I=>I!=null)),Z=[...new Set(Object.values(vt))].sort((I,X)=>(k.has(X)?1:0)-(k.has(I)?1:0)||I-X),it={};Z.forEach((I,X)=>{it[I]=X}),[...h].forEach(I=>{u[I]={x:N0+Q[I]*I0,y:ao-70+it[vt[I]]*U0,d:Q[I],lane:it[vt[I]]}});const lt=new Map;[...h].forEach(I=>{for(const X of o[I]||[]){if(h.has(X.o))continue;lt.has(X.o)||lt.set(X.o,{n:0,steps:[]});const dt=lt.get(X.o);dt.n++,dt.steps.push(I)}}),g=new Map([...lt].filter(([,I])=>I.n>1))}let K={};function B(){if(e.querySelectorAll(".step,.sat,.lanetag,.bedlabel").forEach(j=>j.remove()),!l.length)return;J(),K=F0([...h].map(j=>a[j]).concat([...g.keys()].map(j=>a[j])));const S=new Set(l.map(j=>j.id));[...h].sort((j,Q)=>u[j].d-u[Q].d).forEach(j=>{const Q=a[j];if(!Q||!u[j])return;const{x:ot,y:Y}=u[j],vt=Ir[Q.layer]||"var(--dim)",x=S.has(j),p=l.findIndex(k=>k.id===j),R=b(`<button type="button" class="step${x?"":" aside"}" data-id="${j}"
          style="left:${ot}px;top:${Y}px;--acc:${vt}"
          aria-label="Read ${fe(K[j]||Ve(Q.name))}">
        <span class="meta"><span class="num">${x?String(p+1).padStart(2,"0"):"—"}</span>
          <span class="s">·</span><span class="t">${fe(Q.type)}</span>
          <span class="s">·</span><span>${fe(Q.layer)}</span></span>
        <span class="h2-like"></span>
        <h2>${fe(K[j]||Ve(Q.name))}</h2>
        <p class="gist">${T(Q.body,150)}</p>
        <span class="when">${D(Q)}</span>
      </button>`);e.appendChild(R),R.onclick=()=>kt(j)}),W(),requestAnimationFrame(nt)}function W(){e.querySelectorAll(".sat,.lanetag,.bedlabel").forEach(Y=>Y.remove());const S={};Object.entries(u).forEach(([Y,vt])=>{const x=e.querySelector(`.step[data-id="${Y}"]`);x&&(S[vt.lane]=S[vt.lane]||[]).push({id:Y,p:vt,el:x})});const j=64,Q=150;let ot=ao-70;Object.keys(S).map(Number).sort((Y,vt)=>Y-vt).forEach(Y=>{const vt=S[Y];vt.some(({id:x})=>$(x).length)&&(ot+=Q),vt.forEach(({p:x,el:p})=>{x.y=ot,p.style.top=`${ot}px`}),ot+=Math.max(...vt.map(({el:x})=>x.offsetHeight))+j}),Object.entries(u).forEach(([Y,vt])=>{const x=$(Y);x.length&&et(x.slice(0,1),vt.x,vt.y-130,Y)}),ct(new Set(l.map(Y=>Y.id))),yt(),wt()}const $=S=>(o[S]||[]).filter(j=>!h.has(j.o)&&!g.has(j.o)&&j.dir==="in"&&j.t==="refers_to");function et(S,j,Q,ot){S.forEach((Y,vt)=>{const x=a[Y.o];if(!x)return;const p=Ir[x.layer]||"var(--dim)",R=(oh[Y.t]||[Y.t,Y.t])[Y.dir==="out"?0:1],k=b(`<button type="button" class="sat" data-id="${x.id}" data-of="${ot}"
          style="left:${j+8+vt*310}px;top:${Q}px;--acc:${p}"
          aria-label="Read ${fe(K[x.id]||Ve(x.name))} — ${fe(R)}">
        <span class="rel">${fe(R)}</span><span class="h3">${fe(K[x.id]||Ve(x.name))}</span>
        <span class="ex">${T(x.body,110)}</span>
        <span class="tag">${fe(x.type)} · ${fe(x.tier)}</span></button>`);k.onclick=()=>Ht(x.id),e.appendChild(k)})}const at=S=>{let j="";for(let Q=S;;Q=Math.floor(Q/26)-1)if(j=String.fromCharCode(65+Q%26)+j,Q<26)return j};function ct(S){const j={};Object.entries(u).forEach(([ot,Y])=>{(!j[Y.lane]||Y.d<u[j[Y.lane]].d)&&(j[Y.lane]=ot)});const Q=l.length&&u[l[0].id]?u[l[0].id].lane:0;Object.entries(j).forEach(([ot,Y])=>{const vt=u[Y],x=+ot===Q;e.appendChild(b(`<div class="lanetag${x?" main":""}" style="left:${vt.x}px;top:${vt.y-30}px">
        <span class="ln">${at(+ot)}</span> · ${x?"main trail":"side line"}</div>`))})}function yt(){if(!g.size)return;const S=Object.values(u).map(x=>x.x);let j=ao;Object.entries(u).forEach(([x,p])=>{const R=e.querySelector(`.step[data-id="${x}"]`);j=Math.max(j,p.y+(R?R.offsetHeight:300))});const Q=j+90,ot=g.size,Y=(Math.min(...S)+Math.max(...S)+540)/2,vt=Y-ot*310/2;e.appendChild(b(`<div class="bedlabel" style="left:${vt}px;top:${Q-30}px">
      shared ground — more than one line rests on this</div>`)),[...g].forEach(([x,p],R)=>{const k=a[x];if(!k)return;const Z=Ir[k.layer]||"var(--dim)",it=b(`<button type="button" class="sat bed" data-id="${x}" data-steps="${p.steps.join(",")}"
          style="left:${vt+R*310}px;top:${Q}px;--acc:${Z}"
          aria-label="Read ${fe(K[x]||Ve(k.name))} — shared ground for ${p.n} steps">
        <span class="rel">shared ground · ${p.n}×</span><span class="h3">${fe(K[x]||Ve(k.name))}</span>
        <span class="ex">${T(k.body,130)}</span>
        <span class="tag">${fe(k.type)} · ${fe(k.tier)}</span></button>`);it.onclick=()=>Ht(x),e.appendChild(it)})}function wt(){if(!l.length){i.innerHTML="";return}const S=getComputedStyle(document.documentElement),j=S.getPropertyValue("--seal").trim(),Q=S.getPropertyValue("--line-2").trim(),ot=S.getPropertyValue("--line").trim(),Y=p=>{const R=e.querySelector(`.step[data-id="${p}"]`);if(!R)return null;const k=parseFloat(R.style.left),Z=parseFloat(R.style.top);return{x:k,y:Z,w:R.offsetWidth,h:R.offsetHeight,cy:Z+34}};let vt="";const x={};Object.entries(u).forEach(([p,R])=>{const k=e.querySelector(`.step[data-id="${p}"]`);if(!k)return;const Z=x[R.lane]=x[R.lane]||{y:R.y+34,min:1/0,max:-1/0};Z.min=Math.min(Z.min,R.x),Z.max=Math.max(Z.max,R.x+k.offsetWidth)}),Object.values(x).forEach(p=>{isFinite(p.min)&&(vt+=`<line x1="${p.min-60}" y1="${p.y}" x2="${p.max+60}" y2="${p.y}" stroke="${Q}" stroke-width="1" opacity=".5"/>`)}),V().forEach(p=>{const R=Y(p.from),k=Y(p.to);if(!R||!k)return;const Z=R.x+R.w,it=R.cy,lt=k.x-10,I=k.cy,X=Math.max(60,(lt-Z)*.45);vt+=`<path d="M ${Z} ${it} C ${Z+X} ${it}, ${lt-X} ${I}, ${lt} ${I}" fill="none" stroke="${j}" stroke-width="1.5" opacity=".6"/>`,vt+=`<path d="M ${lt-6} ${I-4} l 6 4 l -6 4" fill="none" stroke="${j}" stroke-width="1.5" opacity=".6"/>`,vt+=`<circle cx="${Z}" cy="${it}" r="3.5" fill="${j}" opacity=".75"/>`}),e.querySelectorAll(".sat.bed").forEach(p=>{const R=parseFloat(p.style.left)+145,k=parseFloat(p.style.top);(p.dataset.steps||"").split(",").filter(Boolean).forEach(Z=>{const it=Y(Z);if(!it)return;const lt=it.x+it.w/2,I=it.y+it.h;vt+=`<path d="M ${lt} ${I} C ${lt} ${I+80}, ${R} ${k-80}, ${R} ${k}" fill="none" stroke="${ot}" stroke-width="1" stroke-dasharray="3 5"/>`})}),e.querySelectorAll(".sat:not(.bed)").forEach(p=>{const R=Y(p.dataset.of);if(!R)return;const k=parseFloat(p.style.left)+40,Z=parseFloat(p.style.top)+p.offsetHeight;vt+=`<path d="M ${k} ${Z} L ${k} ${R.y}" stroke="${ot}" stroke-width="1" stroke-dasharray="3 4"/>`}),i.innerHTML=vt}function kt(S){qt("manuscript");const j=()=>{const Q=r.querySelector(`section[data-id="${S}"]`);if(!Q)return;r.querySelectorAll("section.here").forEach(Y=>Y.classList.remove("here")),Q.classList.add("here");const ot=Q.getBoundingClientRect().top-r.getBoundingClientRect().top;r.scrollTo({top:r.scrollTop+ot-96,behavior:Qt()?"auto":"smooth"})};j(),setTimeout(j,60)}const Qt=()=>matchMedia("(prefers-reduced-motion: reduce)").matches;function C(){if(!l.length){r.innerHTML="";return}let S=`<div class="ms"><div class="kind">${c.adhoc?"node":"reasoning chain"}</div>
      <h1>${fe(Ve(c.name))}</h1>
      <p class="lede">${fe(c.body||(c.adhoc?"A single node, read in full.":"A worked trail of reasoning, read end to end."))}</p>
      <div class="byline">${l.length} ${l.length===1?"step":"steps"} · ${D(l[0])}${l.length>1?" — "+D(l[l.length-1]):""}</div>`;const j=new Set(l.map(ot=>ot.id));[...l,...[...h].filter(ot=>!j.has(ot)).map(ot=>a[ot]).filter(Boolean).sort((ot,Y)=>(ot.created_secs||0)-(Y.created_secs||0))].forEach((ot,Y)=>{const vt=Ir[ot.layer]||"var(--dim)",x=j.has(ot.id);S+=`<section data-id="${ot.id}" style="--acc:${vt}">
        <div class="snum">${x?`STEP ${String(Y+1).padStart(2,"0")}`:"SIDE LINE"} · ${fe(ot.type)} · ${fe(ot.tier)}</div>
        <h2>${fe(K[ot.id]||Ve(ot.name))}</h2>
        <div class="when">asserted <b>${D(ot)}</b></div>
        <div class="prose">${A(ot.body||"—")}</div></section>`;const p=(o[ot.id]||[]).filter(R=>!j.has(R.o)).slice(0,3);p.length&&(S+=`<aside class="marg" style="--acc:${vt}"><div class="mtitle">in the margin</div>`,p.forEach(R=>{const k=a[R.o];if(!k)return;const Z=(oh[R.t]||[R.t,R.t])[R.dir==="out"?0:1];S+=`<button type="button" class="mlink" data-to="${k.id}"
            aria-label="Read ${fe(Ve(k.name))} — ${fe(Z)}">
            <span class="r">${R.dir==="out"?"↳ ":"↰ "}${fe(Z)}</span>
            <span class="n">${fe(Ve(k.name))}</span></button>`}),S+="</aside>")}),r.innerHTML=S+"</div>",r.querySelectorAll(".mlink").forEach(ot=>{ot.onclick=()=>{Ht(ot.dataset.to),qt("manuscript")}})}function U(){e.style.transform=`translate(${_}px,${y}px) scale(${f})`;const S=28*f;s.style.backgroundSize=`${S}px ${S}px`,s.style.backgroundPosition=`${_}px ${y}px`;const j=Ye("rzoom");j&&(j.textContent=Math.round(f*100)+"%")}function nt(){st()}function st(){const S=[...e.children].filter(k=>k.tagName!=="svg"&&k.offsetWidth);if(!S.length)return;let j=1e9,Q=1e9,ot=-1e9,Y=-1e9;S.forEach(k=>{const Z=parseFloat(k.style.left),it=parseFloat(k.style.top);j=Math.min(j,Z),Q=Math.min(Q,it),ot=Math.max(ot,Z+k.offsetWidth),Y=Math.max(Y,it+k.offsetHeight)});const vt=90,x=76,R=innerHeight-x-60;f=lh(Math.min(innerWidth/(ot-j+vt*2),R/(Y-Q+vt*2)),.34,1),_=innerWidth/2-(j+(ot-j)/2)*f,y=x+R/2-(Q+(Y-Q)/2)*f,U()}let Mt=!1,bt=0,Rt=0;s.addEventListener("pointerdown",S=>{S.target.closest("#rdock")||(Mt=!0,bt=S.clientX-_,Rt=S.clientY-y,s.classList.add("grabbing"))}),addEventListener("pointermove",S=>{Mt&&(_=S.clientX-bt,y=S.clientY-Rt,U())}),addEventListener("pointerup",()=>{Mt=!1,s.classList.remove("grabbing")}),s.addEventListener("wheel",S=>{S.preventDefault();const j=lh(f*Math.exp(-S.deltaY*.0012),.2,1.8),Q=s.getBoundingClientRect(),ot=S.clientX-Q.left,Y=S.clientY-Q.top;_=ot-(ot-_)*(j/f),y=Y-(Y-y)*(j/f),f=j,U()},{passive:!1});function qt(S){const j=Ye("rname");j&&c&&(j.textContent=Ve(c.name));const Q=Ye("rhint");Q&&(Q.textContent=S==="manuscript"?"reading the trail end to end":"click a step to read it"),d=S,r.classList.toggle("on",S==="manuscript"),s.style.display=S==="manuscript"?"none":"",Ye("rTrail").classList.toggle("on",S==="trail"),Ye("rMs").classList.toggle("on",S==="manuscript")}const Ut=Ye("rPrev");Ut&&(Ut.onclick=pe),addEventListener("keydown",S=>{Ye("reader").hidden||/^(INPUT|TEXTAREA)$/.test(S.target.tagName)||(S.key==="Backspace"||S.altKey&&S.key==="ArrowLeft")&&(S.preventDefault(),pe())}),Ye("rTrail").onclick=()=>qt("trail"),Ye("rMs").onclick=()=>qt("manuscript");const ee=()=>c?c.adhoc?{node:l[0]&&l[0].id}:{chain:c.id}:null;function ce(S){if(!S)return!1;if(S.chain){const j=n.chains.find(Q=>Q.id===S.chain);return j?be(j,j.members):!1}return S.node&&a[S.node]?be({name:a[S.node].name,adhoc:!0},[S.node]):!1}function Ht(S,j=!0){if(!a[S])return!1;const Q=ee(),ot=n.chains.find(x=>(x.members||[]).includes(S)),Y=ot?ot.members:[S],vt=ot?be(ot,Y):be({name:a[S].name,adhoc:!0},[S]);return vt&&w(Y).then(x=>{x&&c&&be(c,Y)}),vt&&j&&Q&&JSON.stringify(Q)!==JSON.stringify(ee())&&m.push(Q),se(),vt}function pe(){const S=m.pop();if(!S)return!1;const j=ce(S);return se(),j}function se(){const S=Ye("rPrev");if(!S)return;S.hidden=!m.length;const j=m[m.length-1],Q=j?j.chain?(n.chains.find(ot=>ot.id===j.chain)||{}).name:(a[j.node]||{}).name:"";S.title=Q?`Back to ${Ve(Q)}`:""}function be(S,j){return c=S,l=(j||[]).map(Q=>a[Q]).filter(Boolean),l.length?(B(),C(),qt(d),document.fonts&&document.fonts.status!=="loaded"&&document.fonts.ready.then(()=>{c&&l.length&&W()}),!0):!1}return{async show(S){const j=++M,Q=n.chains.find(ot=>ot.id===S)||n.chains.find(ot=>ot.name===S);return Q&&await z(Q.id),j!==M?!1:(m=[],se(),Q?be(Q,Q.members):!1)},async showNode(S){const j=++M;return await w(P(S)),j!==M?!1:(m=[],se(),Ht(S,!1))},cancel(){M++,N?.abort()},back:pe,canBack:()=>m.length>0,chainId:()=>c&&c.id,redraw:wt,hasChain:()=>!!c}}const Ur=[{key:"open",label:"Open",terminal:!1},{key:"in-progress",label:"In Progress",terminal:!1},{key:"done",label:"Done",terminal:!0}],B0=86400,$r=new Map;async function ch(n){if($r.has(n))return $r.get(n);let t=null;try{const i=(await ml.call("context",{initiative:n,view:"board",budget_chars:64e3})).data.sections.find(s=>s.name==="board");i?.details.columns?.length&&(t=i.details.columns)}catch{}return $r.set(n,t),t}const _e=n=>document.getElementById(n),Se=n=>(n||"").replace(/[&<>"']/g,t=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"})[t]),oo=n=>(n||"").replace(/-20\d\d-\d\d-\d\d$/,"").replace(/-[0-9a-f]{6}$/,"").replace(/-/g," "),hh=(n,t)=>(n.tags||[]).find(e=>e.startsWith(t)),uh=n=>n===1?"1 day":`${n} days`;function z0(n,{onOpenNode:t}){const e={},i={};n.nodes.forEach(C=>{e[C.id]=C});for(const C of n.edges)(i[C.src]=i[C.src]||[]).push(C.dst),(i[C.dst]=i[C.dst]||[]).push(C.src);const s=new Date().toISOString().slice(0,10),r=Date.now()/1e3,a=n.nodes.filter(C=>C.type==="task").map(C=>{const U=C.created_secs?Math.floor((r-C.created_secs)/B0):null,nt=(hh(C,"due:")||"").slice(4)||null,st=!!nt&&nt<s,Mt=(i[C.id]||[]).filter(Ut=>e[Ut]),bt=Mt.find(Ut=>e[Ut].type==="episode")||Mt[0]||null,Rt=(hh(C,"status:")||"").slice(7),qt=(C.body||"").trim()||oo(C.name);return{id:C.id,name:C.name,raw:Rt,age:U,due:nt,overdue:st,src:bt,island:Mt.length===0,init:(C.initiatives||[])[0]||null,text:qt,hay:(qt+" "+oo(C.name)).toLowerCase(),rot:(U||0)+(Mt.length===0?45:0)+(st?400:0)}});let o=Ur;const c=C=>o.some(U=>U.key===C.raw)?C.raw:o[0].key,l=C=>o.some(U=>U.key===C&&U.terminal),u=C=>$r.get(C.init)||Ur,g=C=>u(C).some(U=>U.key===C.raw&&U.terminal),h=C=>u(C).find(U=>U.terminal)?.key||"done";async function m(C){if(C!==f)return await ch(C)||Ur;const U=await Promise.all(y.map(([bt])=>ch(bt))),nt=new Map;for(const bt of[Ur,...U.filter(Boolean)])for(const Rt of bt)nt.has(Rt.key)||nt.set(Rt.key,Rt);const st=[...nt.values()],Mt=st.findIndex(bt=>bt.key==="done");return Mt>=0&&st.push(st.splice(Mt,1)[0]),st}function _(){const C=d;m(C).then(U=>{d===C&&(o=U,et(),ct())})}const y=[...a.reduce((C,U)=>C.set(U.init,(C.get(U.init)||0)+(U.raw==="done"?0:1)),new Map)].filter(([C])=>C).sort((C,U)=>U[1]-C[1]),f="*";let d=f,E=null,A=null,T="";const D=new Set,b=[{key:"overdue",label:"past due",f:C=>C.overdue},{key:"island",label:"no provenance",f:C=>C.island},{key:"old",label:"over a month",f:C=>C.age>30},{key:"owed",label:"not done",f:C=>!g(C)}],N=()=>d===f?a:a.filter(C=>C.init===d),M=C=>(!T||C.hay.includes(T))&&[...D].every(U=>b.find(nt=>nt.key===U).f(C)),w=()=>!!(T||D.size);function z(){_e("bsieves").innerHTML=b.map(C=>`<button type="button" class="sieve" data-sieve="${C.key}" aria-pressed="false">
         ${C.label} <span class="c"></span></button>`).join("")}function P(C){b.forEach(U=>{const nt=_e("bsieves").querySelector(`[data-sieve="${U.key}"]`);if(!nt)return;const st=D.has(U.key);nt.setAttribute("aria-pressed",String(st)),nt.classList.toggle("on",st),nt.querySelector(".c").textContent=C.filter(U.f).length}),_e("bClear").hidden=!w()}function V(){const C=[];return T&&C.push(`“${T}”`),D.forEach(U=>C.push(b.find(nt=>nt.key===U).label)),C.join(" + ")}function J(){T="",D.clear(),_e("bFind").value="",et(),ct()}const K=C=>{const U=C.replace(/\s+/g," ").trim(),nt=U.search(/[.!?](\s|$)/);return nt>24&&nt<150?U.slice(0,nt+1):U};function B(C){const U=[C.overdue?`<span class="due">due ${Se(C.due)}</span>`:C.due?`<span class="soft">due ${Se(C.due)}</span>`:"",C.island?'<span class="isle">no provenance</span>':""].filter(Boolean).join('<span class="sep">·</span>');return`<button type="button" class="card${C.overdue?" overdue":""}${E===C.id?" picked":""}"
        data-card="${Se(C.id)}" aria-pressed="${E===C.id}">
      <span class="what">${Se(K(C.text))}</span>
      <span class="meta">${d===f&&C.init?`<span class="owner">${Se(C.init)}</span>`:""}<span class="age">${C.age==null?"—":uh(C.age)}</span>${U?`<span class="sep">·</span>${U}`:""}</span>
    </button>`}const W=new Set(a.map(C=>C.raw));function $(C){return W.has(C.key)||C.key===o[0].key?`<p class="none">${C.label} is empty.</p>`:`<p class="none">Nothing has ever been in ${C.label}. The board reads;
      the vault writes — a card gets here from the chat:</p>
      <code class="cmd">task_update(task=&lt;uuid&gt;, status=${Se(C.key)})</code>`}function et(){const C=N(),U=C.filter(M);_e("bcols").innerHTML=o.map(bt=>{const Rt=U.filter(qt=>c(qt)===bt.key).sort((qt,Ut)=>l(bt.key)?(Ut.age==null)-(qt.age==null)||qt.age-Ut.age:Ut.rot-qt.rot);return`<section class="col" aria-labelledby="col-${bt.key}">
        <header class="colhead"><h2 id="col-${bt.key}">${bt.label}</h2><span class="n">${Rt.length}</span></header>
        <div class="stack">${Rt.length?Rt.map(B).join(""):$(bt)}</div>
      </section>`}).join(""),!U.length&&w()&&(_e("bcols").innerHTML=`<p class="nohits">No cards match ${Se(V())}.
        <button type="button" class="btn" id="bClear2">Clear filters</button></p>`);const nt=U.filter(bt=>!g(bt)).length,st=U.filter(bt=>bt.overdue&&!g(bt)).length,Mt=w()?`${U.length} of ${C.length} cards · `:"";_e("bcount").textContent=U.length?`${Mt}${nt} owed${st?`, ${st} past due`:""} · sorted by rot — deadline, then age, then isolation`:"",P(C),_e("bname").textContent=d===f?"all initiatives":d,_e("bsay").textContent=`${d===f?"all initiatives":d}: ${nt} open, ${U.length} cards total`}let at=null;function ct(){const C=a.find(Mt=>Mt.id===E),U=_e("bdetail");if(U.hidden=!C,!C)return;C.whole||(C.whole=!0,ou([C.id]).then(Mt=>{Object.hasOwn(Mt,C.id)&&Mt[C.id]!==C.text&&(C.text=Mt[C.id],E===C.id&&ct())}).catch(()=>{C.whole=!1}));const nt=o.find(Mt=>Mt.key===c(C)),st=C.island?'<span class="isle">nothing links to this card yet</span>':`<button type="button" class="nodelink" data-open="${Se(C.src)}">${Se(oo(e[C.src]?e[C.src].name:""))}</button>`;U.tabIndex=-1,U.innerHTML=`
      <header id="bdhead">
        <span class="k">card</span><span class="colname">${nt?nt.label:c(C)}</span>
        <button type="button" class="btn x" data-close="1" aria-label="Close card">✕</button>
      </header>
      <div id="bdbody">
        <p class="dbody">${Se(C.text)}</p>
        <dl>
          <dt>initiative</dt><dd>${Se(C.init||"—")}</dd>
          <dt>open for</dt><dd>${C.age==null?"—":uh(C.age)}</dd>
          ${C.due?`<dt>due</dt><dd>${C.overdue?`<span class="due">${Se(C.due)} · past due</span>`:`<span class="soft">${Se(C.due)}</span>`}</dd>`:""}
          <dt>came out of</dt><dd>${st}</dd>
        </dl>
      </div>
      <div id="bdacts">
        <button type="button" class="btn" data-open="${Se(C.id)}">Open in reader</button>
        <button type="button" class="btn cp" data-copy="task_update(initiative=${Se(C.init)}, task=${Se(C.id)}, status=in-progress)">Copy “task_update”</button>
        <button type="button" class="btn cp" data-copy="task_update(initiative=${Se(C.init)}, task=${Se(C.id)}, status=${Se(h(C))})">Copy completion</button>
      </div>`}function yt(){E=null,et(),ct(),at&&document.contains(at)&&at.focus(),at=null}function wt(C,U){if(E===C){yt();return}E=C,at=U||null,et(),ct(),_e("bdetail").focus()}_e("bcols").addEventListener("click",C=>{const U=C.target.closest("[data-card]");U&&wt(U.dataset.card,U)}),_e("bdetail").addEventListener("click",C=>{const U=C.target.closest("button");if(U){if(U.dataset.close){yt();return}if(U.dataset.open){t(U.dataset.open);return}U.dataset.copy&&(navigator.clipboard?.writeText(U.dataset.copy),clearTimeout(A),_e("bdetail").querySelectorAll(".cp").forEach(nt=>nt.classList.remove("ok")),U.classList.add("ok"),_e("bsay").textContent=`Copied: ${U.dataset.copy}`,A=setTimeout(()=>U.classList.remove("ok"),1400))}}),addEventListener("keydown",C=>{C.key!=="Escape"||_e("board").hidden||!E||(C.stopImmediatePropagation(),C.preventDefault(),yt())}),_e("bsieves").addEventListener("click",C=>{const U=C.target.closest("[data-sieve]");if(!U)return;const nt=U.dataset.sieve;D.has(nt)?D.delete(nt):D.add(nt),et(),ct()}),_e("bFind").addEventListener("input",C=>{T=C.target.value.trim().toLowerCase(),et(),ct()}),_e("bFind").addEventListener("keydown",C=>{C.key!=="Escape"||!C.target.value||(C.stopPropagation(),C.target.value="",T="",et(),ct())}),_e("bClear").addEventListener("click",J),_e("bcols").addEventListener("click",C=>{C.target.id==="bClear2"&&J()});const kt=_e("bInit");z();const Qt=a.filter(C=>!g(C)).length;return kt.innerHTML=`<option value="${f}">all initiatives (${Qt} open)</option>`+y.map(([C,U])=>`<option value="${Se(C)}">${Se(C)} (${U} open)</option>`).join(""),kt.addEventListener("change",()=>{d=kt.value,E=null,et(),ct(),_()}),{show(C){return(C===f||C&&y.some(([U])=>U===C))&&(d=C),d?(kt.value=d,kt._sync&&kt._sync(),E=null,et(),ct(),_(),!0):!1},initiative:()=>d}}const je=n=>String(n??"").replace(/[&<>]/g,t=>({"&":"&amp;","<":"&lt;",">":"&gt;"})[t]),ua=n=>new Date(n*1e3).toISOString().slice(0,10),lu=n=>{const t=new Date(n*1e3);return t.toISOString().slice(0,10).replace(/-/g,"·")+" "+t.toISOString().slice(11,16)},At=n=>document.getElementById(n);function k0(n){let t=2166136261;for(let e=0;e<n.length;e++)t^=n.charCodeAt(e),t=Math.imul(t,16777619);return t>>>0}function cu(n){let t=n>>>0;return()=>(t=t*1664525+1013904223>>>0,t/4294967296)}const H0={core:"#caa24a",hot:"#c8402e",warm:"#7e96cf",cold:"#6f7da0",frozen:"#566"},G0={derived_from:[.45,.78,.85],refers_to:[.55,.6,.82],causal:[.88,.66,.36],supersedes:[.85,.45,.45],verifies:[.42,.78,.6],falsifies:[.9,.45,.72],contradicts:[.9,.35,.35],part_of:[.66,.82,.45],temporal:[.6,.64,.78],consolidated_to:[.95,.78,.4],blocks:[.95,.58,.32],targets:[.4,.78,.95]},V0=[.4,.45,.55],ui={hub:{spread:0,size:24,mix:.12,a:1},core:{spread:75,size:22,mix:.12,a:1},hot:{spread:150,size:14,mix:.06,a:1},warm:{spread:240,size:10,mix:.02,a:.96},cold:{spread:300,size:7,mix:0,a:.72},frozen:{spread:345,size:5,mix:0,a:.52}},Ll={dark:{bg:2763311,fog:18e-5,sat:.95,light:.6,mix:new Bt(16774111),edgeMul:1,edgeGround:[0,0,0],edgeOpacity:.1,spoke:[.15,.16,.21],hotEdge:[.95,.88,.66],chainEdge:[1,.83,.4],bridge:13601582,bridgeRest:.6,bridgeDim:.14,particle:15116348,star:10131084,starOp:.4,ring:15526111,dimTo:new Bt(0),dimAmt:.8,hotLerp:new Bt(16777215),chainCur:16777215,chainVisited:16767082,chainLerp:.2},light:{bg:16579576,fog:9e-5,sat:.85,light:.43,mix:new Bt(2761499),edgeMul:.34,edgeGround:[.988,.984,.973],edgeOpacity:.4,spoke:[.84,.8,.72],hotEdge:[.42,.3,.12],chainEdge:[.55,.37,.06],bridge:9071396,bridgeRest:.7,bridgeDim:.18,particle:9071396,star:12103836,starOp:0,ring:3485994,dimTo:new Bt(16579576),dimAmt:.72,hotLerp:new Bt(1708554),chainCur:1513243,chainVisited:9067024,chainLerp:.28}};let wi=null;const hu=matchMedia("(prefers-reduced-motion: reduce)");let Gs=(()=>{try{return localStorage.getItem("kaeru-viz-theme")||"auto"}catch{return"auto"}})();const uu=matchMedia("(prefers-color-scheme: light)"),Dl=()=>Gs==="auto"?uu.matches?"light":"dark":Gs;let Ot=Ll[Dl()];async function W0(){return lo()||await ml.graph()}function X0(n){const t=document.getElementById("graph");t&&t.insertAdjacentHTML("beforeend",`<div style="position:absolute;inset:0;display:grid;place-items:center;color:#8b887e;font-family:'IBM Plex Mono',monospace;font-size:13px;letter-spacing:.04em;text-align:center;padding:24px;">${n}</div>`);const e=document.getElementById("stats");throw e&&(e.textContent="unavailable"),new Error("kaeru-viz: "+n)}const Zt=await W0();Zt&&td(Zt);(!Zt||!Array.isArray(Zt.nodes)||Zt.nodes.length===0)&&X0("no records available under this instance’s access settings");Zt.initiatives??=[];Zt.edges??=[];Zt.chains??=[];Zt.project_links??=[];Zt.meta??={};const Il=Zt.nodes.filter(n=>n.type!=="chain"),$0=n=>n.isHub?n.hubInit:n.initiatives&&n.initiatives[0]||"∅",Is=Zt.initiatives.map(n=>n.name),da={},du={},qr=1050;Is.forEach((n,t)=>{da[n]=t*137.508%360/360;const e=Is.length>1?1-t/(Is.length-1)*2:0,i=Math.sqrt(Math.max(0,1-e*e)),s=t*2.39996;du[n]=new F(Math.cos(s)*i*qr,e*qr*.82,Math.sin(s)*i*qr)});const q0=new F(0,0,0),fu=n=>du[n]||q0,Y0=Il.length/Math.max(1,Is.length),Ul={};for(const n of Zt.initiatives)Ul[n.name]=Math.min(2,Math.max(.7,Math.cbrt((n.node_count||1)/Y0)));const ne=[],as=new Map;for(const n of Il){const t=$0(n),e=fu(t),i=ui[n.layer]||ui.warm,s=Ul[t]||1,r=cu(k0(n.id)),a=()=>(r()+r()+r()-1.5)*.9,o=i.spread*s,c={...n,init:t,layer:n.layer||"warm",isHub:!1,pos:new F(e.x+a()*o,e.y+a()*o,e.z+a()*o)};ne.push(c),as.set(c.id,c)}const j0=Object.fromEntries(Zt.initiatives.map(n=>[n.name,n.node_count]));for(const n of Is){const t={id:"__hub__"+n,name:n,isHub:!0,hubInit:n,layer:"hub",type:"project core",tier:"—",initiatives:[n],count:j0[n]||1,pos:fu(n).clone()};ne.push(t),as.set(t.id,t)}const Ui=ne.length,tn=new Map(ne.map((n,t)=>[n.id,t])),$n=[];for(const n of Zt.edges)tn.has(n.src)&&tn.has(n.dst)&&$n.push({a:tn.get(n.src),b:tn.get(n.dst),type:n.type,kind:"real"});for(const n of ne){if(n.isHub)continue;const t="__hub__"+n.init;tn.has(t)&&$n.push({a:tn.get(n.id),b:tn.get(t),kind:"spoke"})}const Zs=[],K0=[...Zt.project_links||[]].sort((n,t)=>(t.weight||0)-(n.weight||0)).slice(0,20);for(const n of K0){const t="__hub__"+n.a,e="__hub__"+n.b;tn.has(t)&&tn.has(e)&&Zs.push({a:tn.get(t),b:tn.get(e),w:n.weight})}const fa=ne.map(()=>new Set);$n.forEach((n,t)=>{fa[n.a].add(t),fa[n.b].add(t)});const il=At("graph"),_n=new Jd;_n.fog=new Al(Ot.bg,Ot.fog);const $e=new rn(55,innerWidth/innerHeight,1,8e3),xn=new x0({antialias:!0});xn.setPixelRatio(Math.min(2,devicePixelRatio));xn.setSize(innerWidth,innerHeight);xn.setClearColor(Ot.bg,1);il.appendChild(xn.domElement);const Ie=new M0($e,xn.domElement);Ie.enableDamping=!0;Ie.dampingFactor=.08;Ie.enablePan=!1;Ie.autoRotate=!0;Ie.autoRotateSpeed=.4;Ie.minDistance=200;Ie.maxDistance=4e3;function Z0(){const n=document.createElement("canvas");n.width=n.height=64;const t=n.getContext("2d"),e=t.createRadialGradient(32,32,0,32,32,32);return e.addColorStop(0,"rgba(255,255,255,1)"),e.addColorStop(.55,"rgba(255,255,255,1)"),e.addColorStop(.82,"rgba(255,255,255,0.5)"),e.addColorStop(1,"rgba(255,255,255,0)"),t.fillStyle=e,t.fillRect(0,0,64,64),new $h(n)}function J0(){const n=document.createElement("canvas");n.width=n.height=128;const t=n.getContext("2d");t.strokeStyle="rgba(236,232,223,0.95)",t.lineWidth=4,t.beginPath(),t.arc(64,64,52,0,7),t.stroke();for(let e=0;e<4;e++){const i=e*Math.PI/2;t.beginPath(),t.moveTo(64+Math.cos(i)*52,64+Math.sin(i)*52),t.lineTo(64+Math.cos(i)*62,64+Math.sin(i)*62),t.stroke()}return new $h(n)}const Nl=Z0(),Ri=new Float32Array(Ui*3),Yr=new Float32Array(Ui*3),pu=new Float32Array(Ui),Fl=new Float32Array(Ui);ne.forEach((n,t)=>{Ri[t*3]=n.pos.x,Ri[t*3+1]=n.pos.y,Ri[t*3+2]=n.pos.z});const li=new Ue;li.setAttribute("position",new Ae(Ri,3));li.setAttribute("aColor",new Ae(Yr,3));li.setAttribute("aSize",new Ae(pu,1));li.setAttribute("aAlpha",new Ae(Fl,1));const Q0=new gn({uniforms:{map:{value:Nl}},transparent:!0,depthWrite:!1,vertexShader:`attribute vec3 aColor;attribute float aSize;attribute float aAlpha;
    varying vec3 vC;varying float vA;void main(){vC=aColor;vA=aAlpha;
    vec4 mv=modelViewMatrix*vec4(position,1.0);
    gl_PointSize=aSize*(1100.0/ -mv.z);gl_Position=projectionMatrix*mv;}`,fragmentShader:`uniform sampler2D map;varying vec3 vC;varying float vA;
    void main(){if(vA<0.01)discard;vec4 t=texture2D(map,gl_PointCoord);if(t.a<0.04)discard;gl_FragColor=vec4(vC,t.a*vA);}`}),mu=new Cl(li,Q0);mu.renderOrder=3;_n.add(mu);const gu=new Float32Array($n.length*6),jr=new Float32Array($n.length*6);$n.forEach((n,t)=>{const e=ne[n.a].pos,i=ne[n.b].pos;let s=i.x,r=i.y,a=i.z;if(n.kind==="spoke"){const o=e.x-i.x,c=e.y-i.y,l=e.z-i.z,u=Math.hypot(o,c,l)||1,g=Math.min(.45,48*(Ul[ne[n.b].hubInit]||1)/u);s+=o*g,r+=c*g,a+=l*g}gu.set([e.x,e.y,e.z,s,r,a],t*6)});const Sa=new Ue;Sa.setAttribute("position",new Ae(gu,3));Sa.setAttribute("color",new Ae(jr,3));const _u=new wl({vertexColors:!0,transparent:!0,opacity:Ot.edgeOpacity,depthWrite:!1}),xu=new Wh(Sa,_u);xu.renderOrder=0;_n.add(xu);const vu=new Ue,Mu=new Float32Array(Zs.length*6);Zs.forEach((n,t)=>{const e=ne[n.a].pos,i=ne[n.b].pos;Mu.set([e.x,e.y,e.z,i.x,i.y,i.z],t*6)});vu.setAttribute("position",new Ae(Mu,3));const Ol=new wl({color:Ot.bridge,transparent:!0,opacity:Ot.bridgeRest,depthWrite:!1}),Su=new Wh(vu,Ol);Su.renderOrder=1;_n.add(Su);const Bl=[],Nr=cu(99);for(let n=0;n<900;n++)Bl.push(new F(Nr()*2-1,Nr()*2-1,Nr()*2-1).normalize().multiplyScalar(2200+Nr()*1600));const yu=new Ue,Eu=new Float32Array(Bl.length*3);Bl.forEach((n,t)=>Eu.set([n.x,n.y,n.z],t*3));yu.setAttribute("position",new Ae(Eu,3));const sl=new Rl({size:3,map:Nl,color:Ot.star,transparent:!0,opacity:Ot.starOp,blending:ta,depthWrite:!1,sizeAttenuation:!0}),bu=new Cl(yu,sl);bu.renderOrder=-2;_n.add(bu);const tx=Zs.flatMap(()=>[.2,.55,.85]),zl=new Ue,Kr=new Float32Array(tx.length*3);zl.setAttribute("position",new Ae(Kr,3));const kl=new Cl(zl,new Rl({size:5,map:Nl,color:Ot.particle,transparent:!0,opacity:.8,blending:ta,depthWrite:!1,sizeAttenuation:!0}));kl.renderOrder=2;_n.add(kl);const ai=new rf(new kh({map:J0(),color:Ot.ring,transparent:!0,depthTest:!1,depthWrite:!1}));ai.visible=!1;ai.renderOrder=10;_n.add(ai);let Vs="initiative",Tu=!0,Ee=null;const Us=Zt.meta.earliest_secs??0,rl=Zt.meta.latest_secs??0;let Ns=1/0;const We=new Set;let Xe=-1,di=-1;const al={operational:[.03,1,0],archival:[.53,.9,-.02]},Au={core:[.12,1,.05],hot:[.03,1,0],warm:[.58,.92,0],cold:[.63,.55,-.04],frozen:[.09,.28,.14]},dh=n=>n<0?0:n>1?1:n,Fs=(n,t)=>{const[e,i,s]=n[t]||[.6,1,0];return new Bt().setHSL(e,dh(Ot.sat*i),dh(Ot.light+s))},ex=n=>new Bt().setHSL(da[n]??.6,Ot.sat,Ot.light),fh=n=>{if(Vs==="tier")return Fs(al,n.tier);if(Vs==="layer")return Fs(Au,n.layer);const t=da[n.init]??da[n.hubInit],e=t==null?new Bt("#9af"):new Bt().setHSL(t,Ot.sat,Ot.light),i=ui[n.layer]||ui.warm;return e.lerp(Ot.mix,i.mix)},Ws=n=>n.isHub?!0:(n.created_secs??0)<=Ns,Mi=new Bt;function qn(){const n=Ee&&!We.size,t=Xe>=0?Xe:di,e=Xe>=0?Zr:Js;for(let i=0;i<Ui;i++){const s=ne[i],r=ui[s.layer]||ui.warm;let a=r.size;Tu&&(s.layer==="core"||s.layer==="hot")&&(a*=1.45);let o=r.a,c=!1,l=!1;Ws(s)?We.size?We.has(s.id)?(l=!0,s.__cur&&(l="cur"),a=Math.max(a,16)):c=!0:t>=0?i===t?(l=!0,a*=1.6):e.has(i)?l=!0:c=!0:n&&s.init!==Ee&&s.hubInit!==Ee&&(c=!0):o=0,Mi.copy(fh(s)),l==="cur"?Mi.set(Ot.chainCur):l&&We.size?Mi.copy(s.__visited?new Bt(Ot.chainVisited):fh(s)).lerp(Ot.hotLerp,Ot.chainLerp):c&&Mi.lerp(Ot.dimTo,Ot.dimAmt),Yr[i*3]=Mi.r,Yr[i*3+1]=Mi.g,Yr[i*3+2]=Mi.b,pu[i]=a,Fl[i]=o}li.attributes.aColor.needsUpdate=li.attributes.aSize.needsUpdate=li.attributes.aAlpha.needsUpdate=!0,Ol.opacity=We.size||Xe>=0||di>=0||Ee?Ot.bridgeDim:Ot.bridgeRest,ix()}function nx(n){if(n.kind==="spoke")return Ot.spoke;const t=G0[n.type]||V0;return Ot.edgeMul===1?t:[t[0]*Ot.edgeMul,t[1]*Ot.edgeMul,t[2]*Ot.edgeMul]}function ix(){const n=Ee&&!We.size,t=Xe>=0?Xe:di;for(let e=0;e<$n.length;e++){const i=$n[e],s=ne[i.a],r=ne[i.b];let a=nx(i),o=1;if(!(Ws(s)&&Ws(r)))o=0;else if(We.size)o=We.has(s.id)&&We.has(r.id)&&i.kind!=="spoke"?1.4:.05,o>1&&(a=Ot.chainEdge);else if(t>=0)fa[t].has(e)?(o=1.5,a=Ot.hotEdge):o=.06;else if(n)o=(s.init===Ee||s.hubInit===Ee)&&(r.init===Ee||r.hubInit===Ee)?1:.05;else if(i.kind==="spoke"){const u=ne[i.a].layer;o=u==="cold"||u==="frozen"?0:.85}const l=Ot.edgeGround;for(let u=0;u<6;u+=3)jr[e*6+u]=l[0]+(a[0]-l[0])*o,jr[e*6+u+1]=l[1]+(a[1]-l[1])*o,jr[e*6+u+2]=l[2]+(a[2]-l[2])*o}Sa.attributes.color.needsUpdate=!0}const ol=At("readout");function ll(n,t,e){const i=t?`step <b>${t}</b> / ${e} · knowledge chain`:`node · ${je(n.type)}`,s=H0[n.layer]||"#caa24a",r=n.created_secs?lu(n.created_secs):"—",a=n.isHub?`<span class="tag k">${n.count} insights</span><span class="tag">project core</span>`:`<span class="tag k">${je(n.tier)}</span><span class="tag">${je(n.layer)}</span>${(n.initiatives||[]).map(o=>`<span class="tag">${je(o)}</span>`).join("")}`;ol.innerHTML=`
    <div class="bar" style="background:${s}"></div>
    <div class="body">
      <div class="kicker">${i}</div>
      <h2>${je(n.name)}</h2>
      <div class="tags">${a}</div>
      <p>${je(n.body||(n.redacted?"⟨body redacted⟩":n.isHub?"A project cluster — hover its nodes to explore.":"Open this record in the reader to load its text."))}</p>
      ${n.isHub?"":`<div class="ts">asserted <b>${r}</b></div>`}
      ${n.isHub?"":`<button class="btn go rd" data-read="${je(n.id)}">read this trail</button>`}
    </div>`,ol.classList.add("show")}const wu=()=>ol.classList.remove("show");let Zr=new Set,Js=new Set;const Tn=(()=>{const n=document.createElement("div");return n.id="nodechip",document.body.appendChild(n),n})();Tn.style.cssText="position:fixed;z-index:11;pointer-events:none;transform:translate(-50%,-145%);padding:5px 9px;font-family:var(--serif,Georgia,serif);font-size:13px;white-space:nowrap;display:none";let Ci=null,cl=0,Fr=null,hl=0,Hl=!1,ss=null,ph=-1,mh=0;const sx=1.1,rx=120;addEventListener("pointermove",n=>{const t=performance.now();if(Fr){const e=Math.max(1,t-cl);hl=Math.hypot(n.clientX-Fr.x,n.clientY-Fr.y)/e}Fr={x:n.clientX,y:n.clientY},cl=t,Ci=!Hl&&n.target===xn.domElement?{x:n.clientX,y:n.clientY}:null});addEventListener("pointerleave",()=>{Ci=null});addEventListener("pointerdown",n=>{Hl=!0,ss={x:n.clientX,y:n.clientY,onCanvas:n.target===xn.domElement}});addEventListener("pointerup",n=>{Hl=!1,ss&&ss.onCanvas&&Math.hypot(n.clientX-ss.x,n.clientY-ss.y)<5&&(Ci={x:n.clientX,y:n.clientY},Pu(Ru())),ss=null});const Rn=new F;function Ru(){if(!Ci)return-1;let n=-1,t=22*22;for(let e=0;e<Ui;e++){if(Fl[e]<.05||(Rn.set(Ri[e*3],Ri[e*3+1],Ri[e*3+2]).project($e),Rn.z>1))continue;const i=(Rn.x*.5+.5)*innerWidth,s=(-Rn.y*.5+.5)*innerHeight,r=i-Ci.x,a=s-Ci.y,o=ne[e].isHub?2:1,c=(r*r+a*a)/(o*o);c<t&&(t=c,n=e)}return n}function Cu(n){const t=new Set([n]);for(const e of fa[n]){const i=$n[e];t.add(i.a),t.add(i.b)}return t}function ul(){const n=Xe>=0?Xe:di;if(We.size||(n>=0?ll(ne[n]):wu()),n>=0){const t=ne[n];ai.position.copy(t.pos);const e=(ui[t.layer]||ui.warm).size*2.6;ai.scale.set(e,e,1),ai.visible=!0}else ai.visible=!1;qn()}function Gl(n){if(n===Xe)return;if(Xe=n,n<0){Zr=new Set,Tn.style.display="none",il.style.cursor="",ul();return}Zr=Cu(n);const t=ne[n];Rn.copy(t.pos).project($e),Tn.style.display="block",Tn.style.left=(Rn.x*.5+.5)*innerWidth+"px",Tn.style.top=(-Rn.y*.5+.5)*innerHeight+"px",Tn.innerHTML=`${je(t.name)}<div style="font-family:var(--mono,monospace);font-size:9px;letter-spacing:.12em;text-transform:uppercase;color:var(--dim);margin-top:2px">${je(t.type)} · ${Zr.size-1} neighbours</div>`,il.style.cursor="pointer",ul()}function Pu(n){di=n,Js=n>=0?Cu(n):new Set,ul()}let Jr=null;function Lu(n,t=1200){const e=n.length()||1,i=1+320/e;Ie.autoRotate=!1,Ie.target.lerp(n,.6),Vl({x:n.x*i,y:n.y*i,z:n.z*i},n,t)}let Os=null;function Vl(n,t,e){const i=$e.position.clone(),s=Ie.target.clone(),r=performance.now();Os=()=>{const a=Math.min(1,(performance.now()-r)/e),o=e?1-Math.pow(1-a,3):1;$e.position.set(i.x+(n.x-i.x)*o,i.y+(n.y-i.y)*o,i.z+(n.z-i.z)*o),Ie.target.set(s.x+(t.x-s.x)*o,s.y+(t.y-s.y)*o,s.z+(t.z-s.z)*o),a>=1&&(Os=null)}}function Du(n){Iu();const t=(n.members||[]).filter(s=>as.has(s));if(t.length<2)return;di=-1,Js=new Set,We.clear(),t.forEach(s=>We.add(s)),ne.forEach(s=>{s.__visited=!1,s.__cur=!1}),Ei();let e=0;const i=()=>{if(e>0){const r=as.get(t[e-1]);r.__cur=!1,r.__visited=!0}if(e>=t.length){const r=as.get(t[t.length-1]);ll(r,t.length,t.length),qn();return}const s=as.get(t[e]);s.__cur=!0,s.__visited=!0,ll(s,e+1,t.length),Lu(s.pos),qn(),e+=1,Jr=setTimeout(i,2400)};i()}function Iu(){Jr&&(clearTimeout(Jr),Jr=null)}function os(){Iu(),We.clear(),di=-1,Js=new Set,ne.forEach(n=>{n.__visited=!1,n.__cur=!1}),qn(),wu(),Ie.autoRotate=!0,fs(900)}const Xs=At("time"),dl=n=>At("timeLabel").textContent=n;function pa(n){n>=100?(Ns=1/0,dl("— full graph —")):(Ns=Us+n/100*(rl-Us),dl(`${ua(Us)} → ${ua(Ns)}`)),qn()}Xs.addEventListener("input",n=>pa(+n.target.value));let Bs=null;const ma=()=>{Bs&&(clearInterval(Bs),Bs=null)};function ax(){ma();let n=0;Xs.value=0,pa(0),Bs=setInterval(()=>{n+=1.5,n>=100&&(n=100,ma()),Xs.value=n,pa(n)},60)}function Ei(){ma(),Xs.value=100,pa(100)}At("timePlay").addEventListener("click",()=>Bs?ma():ax());function zn(n){Ee=n||null,At("focus").value=n||"",At("focus")._sync&&At("focus")._sync(),ox(),di=-1,Js=new Set,qn(),Ee?hx(Ee,1400):fs(1200)}function ls(n){Vs=n,At("colorMode").value=n,At("colorMode")._sync&&At("colorMode")._sync(),Xl(),qn()}function zs(n){Tu=n,At("glow").checked=n,qn()}At("colorMode").addEventListener("change",n=>ls(n.target.value));At("glow").addEventListener("change",n=>zs(n.target.checked));const $s=At("focus");Zt.initiatives.forEach(n=>{const t=document.createElement("option");t.value=n.name,t.textContent=`${n.name} (${n.node_count})`,$s.appendChild(t)});$s.addEventListener("change",n=>zn(n.target.value));const dn=At("chainPick");Zt.chains.forEach((n,t)=>{const e=document.createElement("option");e.value=t,e.textContent=`${n.name} (${(n.members||[]).length})`,dn.appendChild(e)});At("chainPlay").addEventListener("click",()=>{const n=Zt.chains[+dn.value||0];n&&Du(n)});At("chainReset").addEventListener("click",os);wi=O0(Zt,{fmtDateTime:lu});let Ni=!1,gh=0;const Ds=At("chainRead");async function Fi(n,t){const e=++gh;if(n||wi.cancel(),n){Ds.textContent="…",At("reader").hidden=!1;const s=t?await wi.showNode(t):await wi.show((Zt.chains[+dn.value||0]||{}).id);if(e!==gh)return;if(Ds.textContent=s?"galaxy":"read",!s){At("reader").hidden=!0;return}const r=wi.chainId();if(r){const a=Zt.chains.findIndex(o=>o.id===r);a>=0&&+dn.value!==a&&(dn.value=a,dn._sync&&dn._sync())}}Ni=n,At("reader").hidden=!n,n&&(Gl(-1),Tn.style.display="none"),Ds.classList.toggle("go",n),Ds.textContent=n?"galaxy":"read";const i=At("rBack");i&&(i.textContent=n&&vs?"← board":"galaxy"),At("panel").hidden=n,At("rail").hidden=n,At("readout").hidden=n,At("hud").hidden=n}Ds.addEventListener("click",()=>{vs=!1,Fi(!Ni)});At("rBack").addEventListener("click",()=>{const n=vs;Fi(!1),n&&pi(!0,Wl.initiative())});addEventListener("keydown",n=>{if(n.key!=="Escape"||!Ni||/^(INPUT|TEXTAREA)$/.test(n.target.tagName))return;n.preventDefault(),n.stopImmediatePropagation();const t=vs;Fi(!1),t&&pi(!0,Wl.initiative())});At("readout").addEventListener("click",n=>{const t=n.target.closest("[data-read]");t&&(vs=!1,Fi(!0,t.dataset.read))});dn.addEventListener("change",()=>{Ni&&Fi(!0)});const fl=At("boardBtn");let Oi=!1,vs=!1;const Wl=z0(Zt,{onOpenNode:n=>{pi(!1),vs=!0,Fi(!0,n)}});function pi(n,t){n&&!Wl.show(t||$s.value||"*")||(Oi=n,At("board").hidden=!n,n&&(Gl(-1),Tn.style.display="none"),fl.classList.toggle("go",n),fl.textContent=n?"galaxy":"board",At("panel").hidden=n,At("rail").hidden=n,At("readout").hidden=n,At("hud").hidden=n)}fl.addEventListener("click",()=>pi(!Oi));At("bBack").addEventListener("click",()=>pi(!1));addEventListener("keydown",n=>{n.key!=="Escape"||!Oi||/^(INPUT|TEXTAREA|SELECT)$/.test(n.target.tagName)||(n.preventDefault(),pi(!1))});$s.addEventListener("change",()=>{Oi&&pi(!0,$s.value||"*")});function Uu(n){n.style.display="none";const t=document.createElement("div");t.className="dd";const e=document.createElement("div");e.className="dd-trigger",e.tabIndex=0,e.setAttribute("role","combobox"),e.setAttribute("aria-haspopup","listbox"),e.setAttribute("aria-expanded","false");const i=n.id&&document.querySelector(`label[for="${n.id}"]`);i&&e.setAttribute("aria-label",i.textContent.trim());const s=document.createElement("span");s.className="dd-label";const r=document.createElement("span");r.className="dd-caret",e.append(s,r);const a=document.createElement("div");a.className="dd-menu",a.hidden=!0,a.setAttribute("role","listbox"),t.append(e,a),n.after(t);const o=()=>{a.hidden=!0,e.classList.remove("open"),e.setAttribute("aria-expanded","false"),document.removeEventListener("pointerdown",c,!0),removeEventListener("resize",l),removeEventListener("scroll",l,!0)},c=m=>{t.contains(m.target)||o()};function l(){const m=e.getBoundingClientRect(),_=t.classList.contains("down"),y=5,f=8,d=(_?innerHeight-m.bottom:m.top)-y-f;a.style.minWidth=`${Math.round(m.width)}px`,a.style.maxHeight=`${Math.round(Math.max(120,Math.min(300,d)))}px`;const E=a.getBoundingClientRect().width;a.style.left=`${Math.round(Math.min(Math.max(f,m.left),Math.max(f,innerWidth-E-f)))}px`,a.style.top=_?`${Math.round(m.bottom+y)}px`:"auto",a.style.bottom=_?"auto":`${Math.round(innerHeight-m.top+y)}px`}function u(){const m=n.selectedOptions[0];s.textContent=m?m.textContent:"",[...a.children].forEach(_=>{const y=_.dataset.value===n.value;_.classList.toggle("sel",y),_.setAttribute("aria-selected",String(y))})}function g(){a.innerHTML="";for(const m of n.options){const _=document.createElement("div");_.className="dd-opt",_.dataset.value=m.value,_.setAttribute("role","option"),_.setAttribute("aria-selected",String(m.value===n.value));const y=m.textContent.match(/^(.*?)\s*\(([^)]+)\)\s*$/),f=document.createElement("span");if(f.className="dd-optlabel",f.textContent=y?y[1]:m.textContent,_.append(f),y){const d=document.createElement("span");d.className="dd-meta",d.textContent=y[2],_.append(d)}_.addEventListener("click",()=>{n.value=m.value,n.dispatchEvent(new Event("change")),u(),o()}),a.append(_)}u()}const h=()=>{g(),a.hidden=!1,l(),e.classList.add("open"),e.setAttribute("aria-expanded","true"),document.addEventListener("pointerdown",c,!0),addEventListener("resize",l),addEventListener("scroll",l,!0)};return e.addEventListener("click",()=>{a.hidden?h():o()}),e.addEventListener("keydown",m=>{m.key==="Enter"||m.key===" "||m.key==="ArrowDown"?(m.preventDefault(),a.hidden?h():o()):m.key==="Escape"&&!a.hidden&&(m.stopPropagation(),o())}),n._sync=u,g(),u}["chainPick","colorMode","focus"].forEach(n=>Uu(At(n)));Uu(At("bInit")).call;At("bInit").nextElementSibling.classList.add("down");const Or=Zt.meta;At("stats").innerHTML=`<div class="cell"><div class="n">${Or.node_count??Zt.nodes.length}</div><div class="l">insights</div></div><div class="cell"><div class="n">${Or.edge_count??Zt.edges.length}</div><div class="l">links</div></div><div class="cell"><div class="n">${Or.initiative_count??Zt.initiatives.length}</div><div class="l">projects</div></div><div class="cell"><div class="n">${Or.chain_count??Zt.chains.length}</div><div class="l">chains</div></div><div class="cell span">${ua(Us)} — ${ua(rl)}<span class="sub">one agent · ${Math.round((rl-Us)/86400)} days</span></div>`;function Qr(n){return"#"+n.getHexString()}function ox(){const n=At("projects");if(n)for(const t of n.children){const e=(t.dataset.init||null)===Ee;t.classList.toggle("on",e),t.setAttribute("aria-pressed",String(e))}}function Nu(){const n=At("projects"),t=[...Zt.initiatives].sort((e,i)=>(i.node_count||0)-(e.node_count||0));n.innerHTML=`<button type="button" class="pr all${Ee?"":" on"}" data-init=""
       aria-pressed="${Ee?"false":"true"}">
       <span class="nm">all projects</span><span class="ct">${Il.length}</span>
       <span class="sw" style="background:transparent"></span></button>`+t.map(e=>`<button type="button" class="pr${Ee===e.name?" on":""}"
       data-init="${je(e.name)}" aria-pressed="${Ee===e.name?"true":"false"}">
       <span class="nm">${je(e.name)}</span><span class="ct">${e.node_count||0}</span>
       <span class="sw" style="background:${Qr(ex(e.name))}"></span></button>`).join("");for(const e of n.children)e.addEventListener("click",()=>{const i=e.dataset.init||null;zn(Ee===i?null:i)});cx(n)}const lx=12;function cx(n){const t=()=>{const e=n.querySelectorAll(".pr");if(e.length){const i=n.getBoundingClientRect().top-n.scrollTop,s=o=>e[o].getBoundingClientRect().bottom-i,r=innerHeight-n.getBoundingClientRect().top-96;let a=Math.min(lx,e.length);for(;a>4&&s(a-1)>r;)a--;n.style.maxHeight=`${Math.ceil(s(a-1))}px`}n.classList.toggle("more",n.scrollHeight-n.clientHeight-n.scrollTop>4)};t(),n.addEventListener("scroll",t,{passive:!0}),addEventListener("resize",t)}function Xl(){const n=At("legend");Vs==="initiative"?n.innerHTML="":Vs==="tier"?n.innerHTML=`operational (hippocampus) <span class="sw" style="background:${Qr(Fs(al,"operational"))}"></span><br>archival (cortex) <span class="sw" style="background:${Qr(Fs(al,"archival"))}"></span>`:n.innerHTML=["core","hot","warm","cold","frozen"].map(t=>`${t} <span class="sw" style="background:${Qr(Fs(Au,t))}"></span>`).join("<br>")}Xl();Nu();function Fu(n){let t=1/0,e=1/0,i=1/0,s=-1/0,r=-1/0,a=-1/0,o=!1;for(let u=0;u<Ui;u++){const g=ne[u];if(g.isHub||!Ws(g)||n&&!n(g))continue;const h=g.pos;o=!0,h.x<t&&(t=h.x),h.x>s&&(s=h.x),h.y<e&&(e=h.y),h.y>r&&(r=h.y),h.z<i&&(i=h.z),h.z>a&&(a=h.z)}if(!o)return{c:new F,maxR:qr};const c=new F((t+s)/2,(e+r)/2,(i+a)/2),l=Math.max(s-t,r-e,a-i)/2;return{c,maxR:l}}function fs(n=1200){const{c:t,maxR:e}=Fu(null),i=e/Math.tan($e.fov*Math.PI/180/2)*1;Vl({x:t.x,y:t.y,z:t.z+i},t,n)}function hx(n,t=1400){const{c:e,maxR:i}=Fu(r=>r.init===n),s=Math.max(260,i/Math.tan($e.fov*Math.PI/180/2)*1.8);Ie.autoRotate=!1,Vl({x:e.x,y:e.y,z:e.z+s},e,t)}const cs=[{tag:"a knowledge galaxy",title:"One mind. A knowledge galaxy.",narr:"Months of one agent's work across many projects, in one memory. Same rule shapes it as a galaxy: what belongs together pulls together. Each cluster a project, each point a thought.",apply(){os(),Ei(),zn(null),zs(!0),ls("initiative"),fs(1500)}},{tag:"reasoning chains",title:"How — not just what.",narr:"A knowledge chain is the load-bearing path between insights. Watch how one conclusion was reached — node by node, in order.",apply(){Ei(),zn(null);const n=Zt.chains.reduce((t,e)=>(e.members||[]).length>(t?(t.members||[]).length:0)?e:t,null);n&&(dn.value=Zt.chains.indexOf(n),dn._sync&&dn._sync(),Du(n))}},{tag:"one project, up close",title:"Each cluster is a real project.",narr:"Zoom into a single project and the structure appears: keystone facts in Core, standing rules in Hot, working notes in Warm — scoped and prioritized.",apply(){os(),Ei(),zs(!0),ls("layer"),zn((Zt.initiatives[0]||{}).name||null)}},{tag:"memory layers",title:"Important glows first.",narr:"Memory has priority. Core and Hot glow largest and load on every re-entry; Warm is the working set; Cold and Frozen wait until asked.",apply(){os(),Ei(),zn(null),zs(!0),ls("layer"),fs(1200)}},{tag:"two tiers",title:"Hippocampus & cortex.",narr:"Two tiers, like the brain. Operational is fast, messy working thought; archival is settled, durable knowledge. Operational decays and gets revisited; archival is what survives.",apply(){os(),Ei(),zn(null),ls("tier"),fs(1200)}}];let ya=-1;function ux(n){At("scriptDots").innerHTML=cs.map((t,e)=>`<span class="dot${e===n?" on":""}"></span>`).join("")}function $l(n){if(n<0||n>=cs.length)return;ya=n;const t=cs[n];At("scriptTag").textContent=t.tag,At("scriptNum").textContent=`${n+1} / ${cs.length}`,At("scriptTitle").textContent=t.title,At("scriptNarr").textContent=t.narr,At("scriptPrev").disabled=n===0,At("scriptNext").disabled=n===cs.length-1,ux(n),t.apply()}function dx(){At("panel").hidden=!0,At("script").hidden=!1,Ie.autoRotate=!1,$l(0)}function Ou(){At("script").hidden=!0,At("panel").hidden=!1,ya=-1,os(),Ei(),zn(null),zs(!0),ls("initiative")}const Bu=()=>$l(Math.min(ya+1,cs.length-1)),zu=()=>$l(Math.max(ya-1,0));At("talkBtn").addEventListener("click",dx);At("scriptNext").addEventListener("click",Bu);At("scriptPrev").addEventListener("click",zu);At("scriptExit").addEventListener("click",Ou);addEventListener("keydown",n=>{At("script").hidden||/^(INPUT|TEXTAREA)$/.test(n.target.tagName)||(n.key==="ArrowRight"||n.key===" "?(n.preventDefault(),Bu()):n.key==="ArrowLeft"?(n.preventDefault(),zu()):n.key==="Escape"&&Ou())});function ku(){xn.setClearColor(Ot.bg,1),_n.fog.color.set(Ot.bg),_n.fog.density=Ot.fog,_u.opacity=Ot.edgeOpacity,Ol.color.set(Ot.bridge),kl.material.color.set(Ot.particle),sl.color.set(Ot.star),sl.opacity=Ot.starOp,ai.material.color.set(Ot.ring),Xl(),Nu(),qn(),wi&&wi.redraw()}function Hu(n){Gs=n;try{localStorage.setItem("kaeru-viz-theme",n)}catch{}n==="auto"?document.documentElement.removeAttribute("data-theme"):document.documentElement.setAttribute("data-theme",n),Ot=Ll[Dl()],ku();const t=At("themeSeg");if(t)for(const e of t.children)e.classList.toggle("on",e.dataset.t===n)}for(const n of At("themeSeg").children)n.addEventListener("click",()=>Hu(n.dataset.t));uu.addEventListener("change",()=>{Gs==="auto"&&(Ot=Ll[Dl()],ku())});const De=At("findInput"),bi=At("findHits");let ln=[],fn=-1;const Br=ne.filter(n=>!n.isHub);function Gu(n){const t=ne[n];t&&(Ws(t)||(Ns=1/0,Xs.value=100,dl("— full graph —")),Ee&&t.init!==Ee&&zn(null),Ni&&Fi(!1),Oi&&pi(!1),We.clear(),ne.forEach(e=>{e.__visited=!1,e.__cur=!1}),Pu(n),Lu(t.pos,hu.matches?0:1100),pl(`Showing ${t.name}`))}function qs(){bi.hidden=!0,ln=[],fn=-1,De.setAttribute("aria-expanded","false"),De.removeAttribute("aria-activedescendant")}function pl(n){At("findStatus").textContent=n}function Vu(){if(De.setAttribute("aria-expanded","true"),!ln.length){bi.innerHTML='<div class="none">nothing found</div>',bi.hidden=!1,De.removeAttribute("aria-activedescendant"),pl("No nodes found");return}bi.innerHTML=ln.map((n,t)=>{const e=ne[n];return`<div class="hit${t===fn?" on":""}" id="hit-${n}" role="option"
      aria-selected="${t===fn}" data-i="${n}">
      <span class="hn">${je(e.name)}</span>
      <span class="hm">${je(e.type)} · ${je(e.init)}</span></div>`}).join(""),bi.hidden=!1,fn>=0&&De.setAttribute("aria-activedescendant",`hit-${ln[fn]}`),pl(`${ln.length} node${ln.length===1?"":"s"} found`);for(const n of bi.children)n.dataset.i&&n.addEventListener("mousedown",t=>{t.preventDefault(),Gu(+n.dataset.i),qs(),De.blur()})}function Wu(){const n=De.value.trim().toLowerCase();if(n.length<2){qs();return}const t=[],e=[];for(let i=0;i<Br.length&&t.length+e.length<200;i++){const r=Br[i].name.toLowerCase().indexOf(n);r===0?t.push(tn.get(Br[i].id)):r>0&&e.push(tn.get(Br[i].id))}ln=[...t,...e].slice(0,40),fn=ln.length?0:-1,Vu()}De.addEventListener("input",Wu);De.addEventListener("focus",()=>{De.value.trim().length>=2&&Wu()});De.addEventListener("blur",()=>setTimeout(qs,120));De.addEventListener("keydown",n=>{if(n.key==="Escape"){De.value="",qs(),De.blur();return}ln.length&&(n.key==="ArrowDown"||n.key==="ArrowUp"?(n.preventDefault(),fn=(fn+(n.key==="ArrowDown"?1:ln.length-1))%ln.length,Vu(),bi.children[fn]?.scrollIntoView({block:"nearest"})):n.key==="Enter"&&fn>=0&&(n.preventDefault(),Gu(ln[fn]),qs(),De.blur()))});addEventListener("keydown",n=>{n.key!=="/"||n.target.tagName==="INPUT"||!At("script").hidden||Ni||Oi||(n.preventDefault(),De.focus(),De.select())});addEventListener("resize",()=>{$e.aspect=innerWidth/innerHeight,$e.updateProjectionMatrix(),xn.setSize(innerWidth,innerHeight)});let _h=0;function Xu(){requestAnimationFrame(Xu),_h+=1,performance.now()-cl>80&&(hl=0);const n=Ci&&hl<sx?Ru():-1;n!==ph&&(ph=n,mh=performance.now());let t=Xe;n<0?t=-1:performance.now()-mh>=rx&&(t=n),t!==Xe?Gl(t):Xe>=0&&(Rn.copy(ne[Xe].pos).project($e),Tn.style.left=(Rn.x*.5+.5)*innerWidth+"px",Tn.style.top=(-Rn.y*.5+.5)*innerHeight+"px");let e=0;for(const i of Zs){const s=ne[i.a].pos,r=ne[i.b].pos;for(let a=0;a<3;a++){const o=(_h*.0016+a/3)%1;Kr[e*3]=s.x+(r.x-s.x)*o,Kr[e*3+1]=s.y+(r.y-s.y)*o+Math.sin(o*Math.PI)*40,Kr[e*3+2]=s.z+(r.z-s.z)*o,e++}}zl.attributes.position.needsUpdate=!0,Os&&Os(),Ie.autoRotate=!hu.matches&&!(window.__rec&&window.__rec.active)&&Xe<0&&!We.size&&!Ee&&At("script").hidden&&!Os,Ie.update(),!Ni&&!Oi&&xn.render(_n,$e)}if(new URLSearchParams(location.search).has("rec")){let n=null;window.__rec={active:!1,orbit(t){this.active=!0;const e=Ie.target;if(!n){const s=$e.position.clone().sub(e);n={r:Math.hypot(s.x,s.z),y:s.y}}const i=t*Math.PI*2;$e.position.set(e.x+Math.sin(i)*n.r,e.y+n.y,e.z+Math.cos(i)*n.r),$e.lookAt(e),xn.render(_n,$e)}}}Hu(Gs);fs(0);Xu();
