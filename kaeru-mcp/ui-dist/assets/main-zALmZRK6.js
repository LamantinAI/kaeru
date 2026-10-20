import{m as pl,r as Zu,b as Ju}from"./api-D2prxmKE.js";/**
 * @license
 * Copyright 2010-2026 Three.js Authors
 * SPDX-License-Identifier: MIT
 */const ml="184",cs={ROTATE:0,DOLLY:1,PAN:2},ss={ROTATE:0,PAN:1,DOLLY_PAN:2,DOLLY_ROTATE:3},Qu=0,ec=1,ed=2,zr=1,td=2,Ps=3,ci=0,Kt=1,Bn=2,Hn=0,hs=1,ea=2,tc=3,nc=4,nd=5,Si=100,id=101,sd=102,rd=103,ad=104,od=200,ld=201,cd=202,hd=203,lo=204,co=205,ud=206,dd=207,fd=208,pd=209,md=210,gd=211,_d=212,xd=213,vd=214,ho=0,uo=1,fo=2,ps=3,po=4,mo=5,go=6,_o=7,mh=0,Md=1,Sd=2,Cn=0,gh=1,_h=2,xh=3,vh=4,Mh=5,Sh=6,yh=7,Eh=300,Ci=301,ms=302,Ta=303,Aa=304,ga=306,xo=1e3,kn=1001,vo=1002,Lt=1003,yd=1004,tr=1005,Bt=1006,wa=1007,Ti=1008,an=1009,bh=1010,Th=1011,ks=1012,gl=1013,Ln=1014,An=1015,Vn=1016,_l=1017,xl=1018,Hs=1020,Ah=35902,wh=35899,Rh=1021,Ch=1022,pn=1023,Wn=1026,Ai=1027,Ph=1028,vl=1029,Pi=1030,Ml=1031,Sl=1033,kr=33776,Hr=33777,Gr=33778,Vr=33779,Mo=35840,So=35841,yo=35842,Eo=35843,bo=36196,To=37492,Ao=37496,wo=37488,Ro=37489,ta=37490,Co=37491,Po=37808,Lo=37809,Do=37810,Io=37811,Uo=37812,No=37813,Fo=37814,Oo=37815,Bo=37816,zo=37817,ko=37818,Ho=37819,Go=37820,Vo=37821,Wo=36492,Xo=36494,$o=36495,Yo=36283,qo=36284,na=36285,jo=36286,Ed=3200,ic=0,bd=1,si="",sn="srgb",ia="srgb-linear",sa="linear",at="srgb",zi=7680,sc=519,Td=512,Ad=513,wd=514,yl=515,Rd=516,Cd=517,El=518,Pd=519,Ko=35044,rc="300 es",wn=2e3,ra=2001;function Ld(n){for(let e=n.length-1;e>=0;--e)if(n[e]>=65535)return!0;return!1}function aa(n){return document.createElementNS("http://www.w3.org/1999/xhtml",n)}function Dd(){const n=aa("canvas");return n.style.display="block",n}const ac={};function oa(...n){const e="THREE."+n.shift();console.log(e,...n)}function Lh(n){const e=n[0];if(typeof e=="string"&&e.startsWith("TSL:")){const t=n[1];t&&t.isStackTrace?n[0]+=" "+t.getLocation():n[1]='Stack trace not available. Enable "THREE.Node.captureStackTrace" to capture stack traces.'}return n}function Ne(...n){n=Lh(n);const e="THREE."+n.shift();{const t=n[0];t&&t.isStackTrace?console.warn(t.getError(e)):console.warn(e,...n)}}function Qe(...n){n=Lh(n);const e="THREE."+n.shift();{const t=n[0];t&&t.isStackTrace?console.error(t.getError(e)):console.error(e,...n)}}function Zo(...n){const e=n.join(" ");e in ac||(ac[e]=!0,Ne(...n))}function Id(n,e,t){return new Promise(function(i,s){function r(){switch(n.clientWaitSync(e,n.SYNC_FLUSH_COMMANDS_BIT,0)){case n.WAIT_FAILED:s();break;case n.TIMEOUT_EXPIRED:setTimeout(r,t);break;default:i()}}setTimeout(r,t)})}const Ud={[ho]:uo,[fo]:go,[po]:_o,[ps]:mo,[uo]:ho,[go]:fo,[_o]:po,[mo]:ps};class fi{addEventListener(e,t){this._listeners===void 0&&(this._listeners={});const i=this._listeners;i[e]===void 0&&(i[e]=[]),i[e].indexOf(t)===-1&&i[e].push(t)}hasEventListener(e,t){const i=this._listeners;return i===void 0?!1:i[e]!==void 0&&i[e].indexOf(t)!==-1}removeEventListener(e,t){const i=this._listeners;if(i===void 0)return;const s=i[e];if(s!==void 0){const r=s.indexOf(t);r!==-1&&s.splice(r,1)}}dispatchEvent(e){const t=this._listeners;if(t===void 0)return;const i=t[e.type];if(i!==void 0){e.target=this;const s=i.slice(0);for(let r=0,a=s.length;r<a;r++)s[r].call(this,e);e.target=null}}}const Ft=["00","01","02","03","04","05","06","07","08","09","0a","0b","0c","0d","0e","0f","10","11","12","13","14","15","16","17","18","19","1a","1b","1c","1d","1e","1f","20","21","22","23","24","25","26","27","28","29","2a","2b","2c","2d","2e","2f","30","31","32","33","34","35","36","37","38","39","3a","3b","3c","3d","3e","3f","40","41","42","43","44","45","46","47","48","49","4a","4b","4c","4d","4e","4f","50","51","52","53","54","55","56","57","58","59","5a","5b","5c","5d","5e","5f","60","61","62","63","64","65","66","67","68","69","6a","6b","6c","6d","6e","6f","70","71","72","73","74","75","76","77","78","79","7a","7b","7c","7d","7e","7f","80","81","82","83","84","85","86","87","88","89","8a","8b","8c","8d","8e","8f","90","91","92","93","94","95","96","97","98","99","9a","9b","9c","9d","9e","9f","a0","a1","a2","a3","a4","a5","a6","a7","a8","a9","aa","ab","ac","ad","ae","af","b0","b1","b2","b3","b4","b5","b6","b7","b8","b9","ba","bb","bc","bd","be","bf","c0","c1","c2","c3","c4","c5","c6","c7","c8","c9","ca","cb","cc","cd","ce","cf","d0","d1","d2","d3","d4","d5","d6","d7","d8","d9","da","db","dc","dd","de","df","e0","e1","e2","e3","e4","e5","e6","e7","e8","e9","ea","eb","ec","ed","ee","ef","f0","f1","f2","f3","f4","f5","f6","f7","f8","f9","fa","fb","fc","fd","fe","ff"],Wr=Math.PI/180,Jo=180/Math.PI;function oi(){const n=Math.random()*4294967295|0,e=Math.random()*4294967295|0,t=Math.random()*4294967295|0,i=Math.random()*4294967295|0;return(Ft[n&255]+Ft[n>>8&255]+Ft[n>>16&255]+Ft[n>>24&255]+"-"+Ft[e&255]+Ft[e>>8&255]+"-"+Ft[e>>16&15|64]+Ft[e>>24&255]+"-"+Ft[t&63|128]+Ft[t>>8&255]+"-"+Ft[t>>16&255]+Ft[t>>24&255]+Ft[i&255]+Ft[i>>8&255]+Ft[i>>16&255]+Ft[i>>24&255]).toLowerCase()}function je(n,e,t){return Math.max(e,Math.min(t,n))}function Nd(n,e){return(n%e+e)%e}function Ra(n,e,t){return(1-t)*n+t*e}function bn(n,e){switch(e.constructor){case Float32Array:return n;case Uint32Array:return n/4294967295;case Uint16Array:return n/65535;case Uint8Array:return n/255;case Int32Array:return Math.max(n/2147483647,-1);case Int16Array:return Math.max(n/32767,-1);case Int8Array:return Math.max(n/127,-1);default:throw new Error("Invalid component type.")}}function ct(n,e){switch(e.constructor){case Float32Array:return n;case Uint32Array:return Math.round(n*4294967295);case Uint16Array:return Math.round(n*65535);case Uint8Array:return Math.round(n*255);case Int32Array:return Math.round(n*2147483647);case Int16Array:return Math.round(n*32767);case Int8Array:return Math.round(n*127);default:throw new Error("Invalid component type.")}}const Fd={DEG2RAD:Wr};class Oe{static{Oe.prototype.isVector2=!0}constructor(e=0,t=0){this.x=e,this.y=t}get width(){return this.x}set width(e){this.x=e}get height(){return this.y}set height(e){this.y=e}set(e,t){return this.x=e,this.y=t,this}setScalar(e){return this.x=e,this.y=e,this}setX(e){return this.x=e,this}setY(e){return this.y=e,this}setComponent(e,t){switch(e){case 0:this.x=t;break;case 1:this.y=t;break;default:throw new Error("index is out of range: "+e)}return this}getComponent(e){switch(e){case 0:return this.x;case 1:return this.y;default:throw new Error("index is out of range: "+e)}}clone(){return new this.constructor(this.x,this.y)}copy(e){return this.x=e.x,this.y=e.y,this}add(e){return this.x+=e.x,this.y+=e.y,this}addScalar(e){return this.x+=e,this.y+=e,this}addVectors(e,t){return this.x=e.x+t.x,this.y=e.y+t.y,this}addScaledVector(e,t){return this.x+=e.x*t,this.y+=e.y*t,this}sub(e){return this.x-=e.x,this.y-=e.y,this}subScalar(e){return this.x-=e,this.y-=e,this}subVectors(e,t){return this.x=e.x-t.x,this.y=e.y-t.y,this}multiply(e){return this.x*=e.x,this.y*=e.y,this}multiplyScalar(e){return this.x*=e,this.y*=e,this}divide(e){return this.x/=e.x,this.y/=e.y,this}divideScalar(e){return this.multiplyScalar(1/e)}applyMatrix3(e){const t=this.x,i=this.y,s=e.elements;return this.x=s[0]*t+s[3]*i+s[6],this.y=s[1]*t+s[4]*i+s[7],this}min(e){return this.x=Math.min(this.x,e.x),this.y=Math.min(this.y,e.y),this}max(e){return this.x=Math.max(this.x,e.x),this.y=Math.max(this.y,e.y),this}clamp(e,t){return this.x=je(this.x,e.x,t.x),this.y=je(this.y,e.y,t.y),this}clampScalar(e,t){return this.x=je(this.x,e,t),this.y=je(this.y,e,t),this}clampLength(e,t){const i=this.length();return this.divideScalar(i||1).multiplyScalar(je(i,e,t))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this}negate(){return this.x=-this.x,this.y=-this.y,this}dot(e){return this.x*e.x+this.y*e.y}cross(e){return this.x*e.y-this.y*e.x}lengthSq(){return this.x*this.x+this.y*this.y}length(){return Math.sqrt(this.x*this.x+this.y*this.y)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)}normalize(){return this.divideScalar(this.length()||1)}angle(){return Math.atan2(-this.y,-this.x)+Math.PI}angleTo(e){const t=Math.sqrt(this.lengthSq()*e.lengthSq());if(t===0)return Math.PI/2;const i=this.dot(e)/t;return Math.acos(je(i,-1,1))}distanceTo(e){return Math.sqrt(this.distanceToSquared(e))}distanceToSquared(e){const t=this.x-e.x,i=this.y-e.y;return t*t+i*i}manhattanDistanceTo(e){return Math.abs(this.x-e.x)+Math.abs(this.y-e.y)}setLength(e){return this.normalize().multiplyScalar(e)}lerp(e,t){return this.x+=(e.x-this.x)*t,this.y+=(e.y-this.y)*t,this}lerpVectors(e,t,i){return this.x=e.x+(t.x-e.x)*i,this.y=e.y+(t.y-e.y)*i,this}equals(e){return e.x===this.x&&e.y===this.y}fromArray(e,t=0){return this.x=e[t],this.y=e[t+1],this}toArray(e=[],t=0){return e[t]=this.x,e[t+1]=this.y,e}fromBufferAttribute(e,t){return this.x=e.getX(t),this.y=e.getY(t),this}rotateAround(e,t){const i=Math.cos(t),s=Math.sin(t),r=this.x-e.x,a=this.y-e.y;return this.x=r*i-a*s+e.x,this.y=r*s+a*i+e.y,this}random(){return this.x=Math.random(),this.y=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y}}class hi{constructor(e=0,t=0,i=0,s=1){this.isQuaternion=!0,this._x=e,this._y=t,this._z=i,this._w=s}static slerpFlat(e,t,i,s,r,a,o){let c=i[s+0],l=i[s+1],u=i[s+2],g=i[s+3],h=r[a+0],m=r[a+1],_=r[a+2],S=r[a+3];if(g!==S||c!==h||l!==m||u!==_){let p=c*h+l*m+u*_+g*S;p<0&&(h=-h,m=-m,_=-_,S=-S,p=-p);let d=1-o;if(p<.9995){const y=Math.acos(p),A=Math.sin(y);d=Math.sin(d*y)/A,o=Math.sin(o*y)/A,c=c*d+h*o,l=l*d+m*o,u=u*d+_*o,g=g*d+S*o}else{c=c*d+h*o,l=l*d+m*o,u=u*d+_*o,g=g*d+S*o;const y=1/Math.sqrt(c*c+l*l+u*u+g*g);c*=y,l*=y,u*=y,g*=y}}e[t]=c,e[t+1]=l,e[t+2]=u,e[t+3]=g}static multiplyQuaternionsFlat(e,t,i,s,r,a){const o=i[s],c=i[s+1],l=i[s+2],u=i[s+3],g=r[a],h=r[a+1],m=r[a+2],_=r[a+3];return e[t]=o*_+u*g+c*m-l*h,e[t+1]=c*_+u*h+l*g-o*m,e[t+2]=l*_+u*m+o*h-c*g,e[t+3]=u*_-o*g-c*h-l*m,e}get x(){return this._x}set x(e){this._x=e,this._onChangeCallback()}get y(){return this._y}set y(e){this._y=e,this._onChangeCallback()}get z(){return this._z}set z(e){this._z=e,this._onChangeCallback()}get w(){return this._w}set w(e){this._w=e,this._onChangeCallback()}set(e,t,i,s){return this._x=e,this._y=t,this._z=i,this._w=s,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._w)}copy(e){return this._x=e.x,this._y=e.y,this._z=e.z,this._w=e.w,this._onChangeCallback(),this}setFromEuler(e,t=!0){const i=e._x,s=e._y,r=e._z,a=e._order,o=Math.cos,c=Math.sin,l=o(i/2),u=o(s/2),g=o(r/2),h=c(i/2),m=c(s/2),_=c(r/2);switch(a){case"XYZ":this._x=h*u*g+l*m*_,this._y=l*m*g-h*u*_,this._z=l*u*_+h*m*g,this._w=l*u*g-h*m*_;break;case"YXZ":this._x=h*u*g+l*m*_,this._y=l*m*g-h*u*_,this._z=l*u*_-h*m*g,this._w=l*u*g+h*m*_;break;case"ZXY":this._x=h*u*g-l*m*_,this._y=l*m*g+h*u*_,this._z=l*u*_+h*m*g,this._w=l*u*g-h*m*_;break;case"ZYX":this._x=h*u*g-l*m*_,this._y=l*m*g+h*u*_,this._z=l*u*_-h*m*g,this._w=l*u*g+h*m*_;break;case"YZX":this._x=h*u*g+l*m*_,this._y=l*m*g+h*u*_,this._z=l*u*_-h*m*g,this._w=l*u*g-h*m*_;break;case"XZY":this._x=h*u*g-l*m*_,this._y=l*m*g-h*u*_,this._z=l*u*_+h*m*g,this._w=l*u*g+h*m*_;break;default:Ne("Quaternion: .setFromEuler() encountered an unknown order: "+a)}return t===!0&&this._onChangeCallback(),this}setFromAxisAngle(e,t){const i=t/2,s=Math.sin(i);return this._x=e.x*s,this._y=e.y*s,this._z=e.z*s,this._w=Math.cos(i),this._onChangeCallback(),this}setFromRotationMatrix(e){const t=e.elements,i=t[0],s=t[4],r=t[8],a=t[1],o=t[5],c=t[9],l=t[2],u=t[6],g=t[10],h=i+o+g;if(h>0){const m=.5/Math.sqrt(h+1);this._w=.25/m,this._x=(u-c)*m,this._y=(r-l)*m,this._z=(a-s)*m}else if(i>o&&i>g){const m=2*Math.sqrt(1+i-o-g);this._w=(u-c)/m,this._x=.25*m,this._y=(s+a)/m,this._z=(r+l)/m}else if(o>g){const m=2*Math.sqrt(1+o-i-g);this._w=(r-l)/m,this._x=(s+a)/m,this._y=.25*m,this._z=(c+u)/m}else{const m=2*Math.sqrt(1+g-i-o);this._w=(a-s)/m,this._x=(r+l)/m,this._y=(c+u)/m,this._z=.25*m}return this._onChangeCallback(),this}setFromUnitVectors(e,t){let i=e.dot(t)+1;return i<1e-8?(i=0,Math.abs(e.x)>Math.abs(e.z)?(this._x=-e.y,this._y=e.x,this._z=0,this._w=i):(this._x=0,this._y=-e.z,this._z=e.y,this._w=i)):(this._x=e.y*t.z-e.z*t.y,this._y=e.z*t.x-e.x*t.z,this._z=e.x*t.y-e.y*t.x,this._w=i),this.normalize()}angleTo(e){return 2*Math.acos(Math.abs(je(this.dot(e),-1,1)))}rotateTowards(e,t){const i=this.angleTo(e);if(i===0)return this;const s=Math.min(1,t/i);return this.slerp(e,s),this}identity(){return this.set(0,0,0,1)}invert(){return this.conjugate()}conjugate(){return this._x*=-1,this._y*=-1,this._z*=-1,this._onChangeCallback(),this}dot(e){return this._x*e._x+this._y*e._y+this._z*e._z+this._w*e._w}lengthSq(){return this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w}length(){return Math.sqrt(this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w)}normalize(){let e=this.length();return e===0?(this._x=0,this._y=0,this._z=0,this._w=1):(e=1/e,this._x=this._x*e,this._y=this._y*e,this._z=this._z*e,this._w=this._w*e),this._onChangeCallback(),this}multiply(e){return this.multiplyQuaternions(this,e)}premultiply(e){return this.multiplyQuaternions(e,this)}multiplyQuaternions(e,t){const i=e._x,s=e._y,r=e._z,a=e._w,o=t._x,c=t._y,l=t._z,u=t._w;return this._x=i*u+a*o+s*l-r*c,this._y=s*u+a*c+r*o-i*l,this._z=r*u+a*l+i*c-s*o,this._w=a*u-i*o-s*c-r*l,this._onChangeCallback(),this}slerp(e,t){let i=e._x,s=e._y,r=e._z,a=e._w,o=this.dot(e);o<0&&(i=-i,s=-s,r=-r,a=-a,o=-o);let c=1-t;if(o<.9995){const l=Math.acos(o),u=Math.sin(l);c=Math.sin(c*l)/u,t=Math.sin(t*l)/u,this._x=this._x*c+i*t,this._y=this._y*c+s*t,this._z=this._z*c+r*t,this._w=this._w*c+a*t,this._onChangeCallback()}else this._x=this._x*c+i*t,this._y=this._y*c+s*t,this._z=this._z*c+r*t,this._w=this._w*c+a*t,this.normalize();return this}slerpQuaternions(e,t,i){return this.copy(e).slerp(t,i)}random(){const e=2*Math.PI*Math.random(),t=2*Math.PI*Math.random(),i=Math.random(),s=Math.sqrt(1-i),r=Math.sqrt(i);return this.set(s*Math.sin(e),s*Math.cos(e),r*Math.sin(t),r*Math.cos(t))}equals(e){return e._x===this._x&&e._y===this._y&&e._z===this._z&&e._w===this._w}fromArray(e,t=0){return this._x=e[t],this._y=e[t+1],this._z=e[t+2],this._w=e[t+3],this._onChangeCallback(),this}toArray(e=[],t=0){return e[t]=this._x,e[t+1]=this._y,e[t+2]=this._z,e[t+3]=this._w,e}fromBufferAttribute(e,t){return this._x=e.getX(t),this._y=e.getY(t),this._z=e.getZ(t),this._w=e.getW(t),this._onChangeCallback(),this}toJSON(){return this.toArray()}_onChange(e){return this._onChangeCallback=e,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._w}}class N{static{N.prototype.isVector3=!0}constructor(e=0,t=0,i=0){this.x=e,this.y=t,this.z=i}set(e,t,i){return i===void 0&&(i=this.z),this.x=e,this.y=t,this.z=i,this}setScalar(e){return this.x=e,this.y=e,this.z=e,this}setX(e){return this.x=e,this}setY(e){return this.y=e,this}setZ(e){return this.z=e,this}setComponent(e,t){switch(e){case 0:this.x=t;break;case 1:this.y=t;break;case 2:this.z=t;break;default:throw new Error("index is out of range: "+e)}return this}getComponent(e){switch(e){case 0:return this.x;case 1:return this.y;case 2:return this.z;default:throw new Error("index is out of range: "+e)}}clone(){return new this.constructor(this.x,this.y,this.z)}copy(e){return this.x=e.x,this.y=e.y,this.z=e.z,this}add(e){return this.x+=e.x,this.y+=e.y,this.z+=e.z,this}addScalar(e){return this.x+=e,this.y+=e,this.z+=e,this}addVectors(e,t){return this.x=e.x+t.x,this.y=e.y+t.y,this.z=e.z+t.z,this}addScaledVector(e,t){return this.x+=e.x*t,this.y+=e.y*t,this.z+=e.z*t,this}sub(e){return this.x-=e.x,this.y-=e.y,this.z-=e.z,this}subScalar(e){return this.x-=e,this.y-=e,this.z-=e,this}subVectors(e,t){return this.x=e.x-t.x,this.y=e.y-t.y,this.z=e.z-t.z,this}multiply(e){return this.x*=e.x,this.y*=e.y,this.z*=e.z,this}multiplyScalar(e){return this.x*=e,this.y*=e,this.z*=e,this}multiplyVectors(e,t){return this.x=e.x*t.x,this.y=e.y*t.y,this.z=e.z*t.z,this}applyEuler(e){return this.applyQuaternion(oc.setFromEuler(e))}applyAxisAngle(e,t){return this.applyQuaternion(oc.setFromAxisAngle(e,t))}applyMatrix3(e){const t=this.x,i=this.y,s=this.z,r=e.elements;return this.x=r[0]*t+r[3]*i+r[6]*s,this.y=r[1]*t+r[4]*i+r[7]*s,this.z=r[2]*t+r[5]*i+r[8]*s,this}applyNormalMatrix(e){return this.applyMatrix3(e).normalize()}applyMatrix4(e){const t=this.x,i=this.y,s=this.z,r=e.elements,a=1/(r[3]*t+r[7]*i+r[11]*s+r[15]);return this.x=(r[0]*t+r[4]*i+r[8]*s+r[12])*a,this.y=(r[1]*t+r[5]*i+r[9]*s+r[13])*a,this.z=(r[2]*t+r[6]*i+r[10]*s+r[14])*a,this}applyQuaternion(e){const t=this.x,i=this.y,s=this.z,r=e.x,a=e.y,o=e.z,c=e.w,l=2*(a*s-o*i),u=2*(o*t-r*s),g=2*(r*i-a*t);return this.x=t+c*l+a*g-o*u,this.y=i+c*u+o*l-r*g,this.z=s+c*g+r*u-a*l,this}project(e){return this.applyMatrix4(e.matrixWorldInverse).applyMatrix4(e.projectionMatrix)}unproject(e){return this.applyMatrix4(e.projectionMatrixInverse).applyMatrix4(e.matrixWorld)}transformDirection(e){const t=this.x,i=this.y,s=this.z,r=e.elements;return this.x=r[0]*t+r[4]*i+r[8]*s,this.y=r[1]*t+r[5]*i+r[9]*s,this.z=r[2]*t+r[6]*i+r[10]*s,this.normalize()}divide(e){return this.x/=e.x,this.y/=e.y,this.z/=e.z,this}divideScalar(e){return this.multiplyScalar(1/e)}min(e){return this.x=Math.min(this.x,e.x),this.y=Math.min(this.y,e.y),this.z=Math.min(this.z,e.z),this}max(e){return this.x=Math.max(this.x,e.x),this.y=Math.max(this.y,e.y),this.z=Math.max(this.z,e.z),this}clamp(e,t){return this.x=je(this.x,e.x,t.x),this.y=je(this.y,e.y,t.y),this.z=je(this.z,e.z,t.z),this}clampScalar(e,t){return this.x=je(this.x,e,t),this.y=je(this.y,e,t),this.z=je(this.z,e,t),this}clampLength(e,t){const i=this.length();return this.divideScalar(i||1).multiplyScalar(je(i,e,t))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this}dot(e){return this.x*e.x+this.y*e.y+this.z*e.z}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)}normalize(){return this.divideScalar(this.length()||1)}setLength(e){return this.normalize().multiplyScalar(e)}lerp(e,t){return this.x+=(e.x-this.x)*t,this.y+=(e.y-this.y)*t,this.z+=(e.z-this.z)*t,this}lerpVectors(e,t,i){return this.x=e.x+(t.x-e.x)*i,this.y=e.y+(t.y-e.y)*i,this.z=e.z+(t.z-e.z)*i,this}cross(e){return this.crossVectors(this,e)}crossVectors(e,t){const i=e.x,s=e.y,r=e.z,a=t.x,o=t.y,c=t.z;return this.x=s*c-r*o,this.y=r*a-i*c,this.z=i*o-s*a,this}projectOnVector(e){const t=e.lengthSq();if(t===0)return this.set(0,0,0);const i=e.dot(this)/t;return this.copy(e).multiplyScalar(i)}projectOnPlane(e){return Ca.copy(this).projectOnVector(e),this.sub(Ca)}reflect(e){return this.sub(Ca.copy(e).multiplyScalar(2*this.dot(e)))}angleTo(e){const t=Math.sqrt(this.lengthSq()*e.lengthSq());if(t===0)return Math.PI/2;const i=this.dot(e)/t;return Math.acos(je(i,-1,1))}distanceTo(e){return Math.sqrt(this.distanceToSquared(e))}distanceToSquared(e){const t=this.x-e.x,i=this.y-e.y,s=this.z-e.z;return t*t+i*i+s*s}manhattanDistanceTo(e){return Math.abs(this.x-e.x)+Math.abs(this.y-e.y)+Math.abs(this.z-e.z)}setFromSpherical(e){return this.setFromSphericalCoords(e.radius,e.phi,e.theta)}setFromSphericalCoords(e,t,i){const s=Math.sin(t)*e;return this.x=s*Math.sin(i),this.y=Math.cos(t)*e,this.z=s*Math.cos(i),this}setFromCylindrical(e){return this.setFromCylindricalCoords(e.radius,e.theta,e.y)}setFromCylindricalCoords(e,t,i){return this.x=e*Math.sin(t),this.y=i,this.z=e*Math.cos(t),this}setFromMatrixPosition(e){const t=e.elements;return this.x=t[12],this.y=t[13],this.z=t[14],this}setFromMatrixScale(e){const t=this.setFromMatrixColumn(e,0).length(),i=this.setFromMatrixColumn(e,1).length(),s=this.setFromMatrixColumn(e,2).length();return this.x=t,this.y=i,this.z=s,this}setFromMatrixColumn(e,t){return this.fromArray(e.elements,t*4)}setFromMatrix3Column(e,t){return this.fromArray(e.elements,t*3)}setFromEuler(e){return this.x=e._x,this.y=e._y,this.z=e._z,this}setFromColor(e){return this.x=e.r,this.y=e.g,this.z=e.b,this}equals(e){return e.x===this.x&&e.y===this.y&&e.z===this.z}fromArray(e,t=0){return this.x=e[t],this.y=e[t+1],this.z=e[t+2],this}toArray(e=[],t=0){return e[t]=this.x,e[t+1]=this.y,e[t+2]=this.z,e}fromBufferAttribute(e,t){return this.x=e.getX(t),this.y=e.getY(t),this.z=e.getZ(t),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this}randomDirection(){const e=Math.random()*Math.PI*2,t=Math.random()*2-1,i=Math.sqrt(1-t*t);return this.x=i*Math.cos(e),this.y=t,this.z=i*Math.sin(e),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z}}const Ca=new N,oc=new hi;class He{static{He.prototype.isMatrix3=!0}constructor(e,t,i,s,r,a,o,c,l){this.elements=[1,0,0,0,1,0,0,0,1],e!==void 0&&this.set(e,t,i,s,r,a,o,c,l)}set(e,t,i,s,r,a,o,c,l){const u=this.elements;return u[0]=e,u[1]=s,u[2]=o,u[3]=t,u[4]=r,u[5]=c,u[6]=i,u[7]=a,u[8]=l,this}identity(){return this.set(1,0,0,0,1,0,0,0,1),this}copy(e){const t=this.elements,i=e.elements;return t[0]=i[0],t[1]=i[1],t[2]=i[2],t[3]=i[3],t[4]=i[4],t[5]=i[5],t[6]=i[6],t[7]=i[7],t[8]=i[8],this}extractBasis(e,t,i){return e.setFromMatrix3Column(this,0),t.setFromMatrix3Column(this,1),i.setFromMatrix3Column(this,2),this}setFromMatrix4(e){const t=e.elements;return this.set(t[0],t[4],t[8],t[1],t[5],t[9],t[2],t[6],t[10]),this}multiply(e){return this.multiplyMatrices(this,e)}premultiply(e){return this.multiplyMatrices(e,this)}multiplyMatrices(e,t){const i=e.elements,s=t.elements,r=this.elements,a=i[0],o=i[3],c=i[6],l=i[1],u=i[4],g=i[7],h=i[2],m=i[5],_=i[8],S=s[0],p=s[3],d=s[6],y=s[1],A=s[4],T=s[7],L=s[2],b=s[5],I=s[8];return r[0]=a*S+o*y+c*L,r[3]=a*p+o*A+c*b,r[6]=a*d+o*T+c*I,r[1]=l*S+u*y+g*L,r[4]=l*p+u*A+g*b,r[7]=l*d+u*T+g*I,r[2]=h*S+m*y+_*L,r[5]=h*p+m*A+_*b,r[8]=h*d+m*T+_*I,this}multiplyScalar(e){const t=this.elements;return t[0]*=e,t[3]*=e,t[6]*=e,t[1]*=e,t[4]*=e,t[7]*=e,t[2]*=e,t[5]*=e,t[8]*=e,this}determinant(){const e=this.elements,t=e[0],i=e[1],s=e[2],r=e[3],a=e[4],o=e[5],c=e[6],l=e[7],u=e[8];return t*a*u-t*o*l-i*r*u+i*o*c+s*r*l-s*a*c}invert(){const e=this.elements,t=e[0],i=e[1],s=e[2],r=e[3],a=e[4],o=e[5],c=e[6],l=e[7],u=e[8],g=u*a-o*l,h=o*c-u*r,m=l*r-a*c,_=t*g+i*h+s*m;if(_===0)return this.set(0,0,0,0,0,0,0,0,0);const S=1/_;return e[0]=g*S,e[1]=(s*l-u*i)*S,e[2]=(o*i-s*a)*S,e[3]=h*S,e[4]=(u*t-s*c)*S,e[5]=(s*r-o*t)*S,e[6]=m*S,e[7]=(i*c-l*t)*S,e[8]=(a*t-i*r)*S,this}transpose(){let e;const t=this.elements;return e=t[1],t[1]=t[3],t[3]=e,e=t[2],t[2]=t[6],t[6]=e,e=t[5],t[5]=t[7],t[7]=e,this}getNormalMatrix(e){return this.setFromMatrix4(e).invert().transpose()}transposeIntoArray(e){const t=this.elements;return e[0]=t[0],e[1]=t[3],e[2]=t[6],e[3]=t[1],e[4]=t[4],e[5]=t[7],e[6]=t[2],e[7]=t[5],e[8]=t[8],this}setUvTransform(e,t,i,s,r,a,o){const c=Math.cos(r),l=Math.sin(r);return this.set(i*c,i*l,-i*(c*a+l*o)+a+e,-s*l,s*c,-s*(-l*a+c*o)+o+t,0,0,1),this}scale(e,t){return this.premultiply(Pa.makeScale(e,t)),this}rotate(e){return this.premultiply(Pa.makeRotation(-e)),this}translate(e,t){return this.premultiply(Pa.makeTranslation(e,t)),this}makeTranslation(e,t){return e.isVector2?this.set(1,0,e.x,0,1,e.y,0,0,1):this.set(1,0,e,0,1,t,0,0,1),this}makeRotation(e){const t=Math.cos(e),i=Math.sin(e);return this.set(t,-i,0,i,t,0,0,0,1),this}makeScale(e,t){return this.set(e,0,0,0,t,0,0,0,1),this}equals(e){const t=this.elements,i=e.elements;for(let s=0;s<9;s++)if(t[s]!==i[s])return!1;return!0}fromArray(e,t=0){for(let i=0;i<9;i++)this.elements[i]=e[i+t];return this}toArray(e=[],t=0){const i=this.elements;return e[t]=i[0],e[t+1]=i[1],e[t+2]=i[2],e[t+3]=i[3],e[t+4]=i[4],e[t+5]=i[5],e[t+6]=i[6],e[t+7]=i[7],e[t+8]=i[8],e}clone(){return new this.constructor().fromArray(this.elements)}}const Pa=new He,lc=new He().set(.4123908,.3575843,.1804808,.212639,.7151687,.0721923,.0193308,.1191948,.9505322),cc=new He().set(3.2409699,-1.5373832,-.4986108,-.9692436,1.8759675,.0415551,.0556301,-.203977,1.0569715);function Od(){const n={enabled:!0,workingColorSpace:ia,spaces:{},convert:function(s,r,a){return this.enabled===!1||r===a||!r||!a||(this.spaces[r].transfer===at&&(s.r=Gn(s.r),s.g=Gn(s.g),s.b=Gn(s.b)),this.spaces[r].primaries!==this.spaces[a].primaries&&(s.applyMatrix3(this.spaces[r].toXYZ),s.applyMatrix3(this.spaces[a].fromXYZ)),this.spaces[a].transfer===at&&(s.r=us(s.r),s.g=us(s.g),s.b=us(s.b))),s},workingToColorSpace:function(s,r){return this.convert(s,this.workingColorSpace,r)},colorSpaceToWorking:function(s,r){return this.convert(s,r,this.workingColorSpace)},getPrimaries:function(s){return this.spaces[s].primaries},getTransfer:function(s){return s===si?sa:this.spaces[s].transfer},getToneMappingMode:function(s){return this.spaces[s].outputColorSpaceConfig.toneMappingMode||"standard"},getLuminanceCoefficients:function(s,r=this.workingColorSpace){return s.fromArray(this.spaces[r].luminanceCoefficients)},define:function(s){Object.assign(this.spaces,s)},_getMatrix:function(s,r,a){return s.copy(this.spaces[r].toXYZ).multiply(this.spaces[a].fromXYZ)},_getDrawingBufferColorSpace:function(s){return this.spaces[s].outputColorSpaceConfig.drawingBufferColorSpace},_getUnpackColorSpace:function(s=this.workingColorSpace){return this.spaces[s].workingColorSpaceConfig.unpackColorSpace},fromWorkingColorSpace:function(s,r){return Zo("ColorManagement: .fromWorkingColorSpace() has been renamed to .workingToColorSpace()."),n.workingToColorSpace(s,r)},toWorkingColorSpace:function(s,r){return Zo("ColorManagement: .toWorkingColorSpace() has been renamed to .colorSpaceToWorking()."),n.colorSpaceToWorking(s,r)}},e=[.64,.33,.3,.6,.15,.06],t=[.2126,.7152,.0722],i=[.3127,.329];return n.define({[ia]:{primaries:e,whitePoint:i,transfer:sa,toXYZ:lc,fromXYZ:cc,luminanceCoefficients:t,workingColorSpaceConfig:{unpackColorSpace:sn},outputColorSpaceConfig:{drawingBufferColorSpace:sn}},[sn]:{primaries:e,whitePoint:i,transfer:at,toXYZ:lc,fromXYZ:cc,luminanceCoefficients:t,outputColorSpaceConfig:{drawingBufferColorSpace:sn}}}),n}const Ze=Od();function Gn(n){return n<.04045?n*.0773993808:Math.pow(n*.9478672986+.0521327014,2.4)}function us(n){return n<.0031308?n*12.92:1.055*Math.pow(n,.41666)-.055}let ki;class Bd{static getDataURL(e,t="image/png"){if(/^data:/i.test(e.src)||typeof HTMLCanvasElement>"u")return e.src;let i;if(e instanceof HTMLCanvasElement)i=e;else{ki===void 0&&(ki=aa("canvas")),ki.width=e.width,ki.height=e.height;const s=ki.getContext("2d");e instanceof ImageData?s.putImageData(e,0,0):s.drawImage(e,0,0,e.width,e.height),i=ki}return i.toDataURL(t)}static sRGBToLinear(e){if(typeof HTMLImageElement<"u"&&e instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&e instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&e instanceof ImageBitmap){const t=aa("canvas");t.width=e.width,t.height=e.height;const i=t.getContext("2d");i.drawImage(e,0,0,e.width,e.height);const s=i.getImageData(0,0,e.width,e.height),r=s.data;for(let a=0;a<r.length;a++)r[a]=Gn(r[a]/255)*255;return i.putImageData(s,0,0),t}else if(e.data){const t=e.data.slice(0);for(let i=0;i<t.length;i++)t instanceof Uint8Array||t instanceof Uint8ClampedArray?t[i]=Math.floor(Gn(t[i]/255)*255):t[i]=Gn(t[i]);return{data:t,width:e.width,height:e.height}}else return Ne("ImageUtils.sRGBToLinear(): Unsupported image type. No color space conversion applied."),e}}let zd=0;class bl{constructor(e=null){this.isSource=!0,Object.defineProperty(this,"id",{value:zd++}),this.uuid=oi(),this.data=e,this.dataReady=!0,this.version=0}getSize(e){const t=this.data;return typeof HTMLVideoElement<"u"&&t instanceof HTMLVideoElement?e.set(t.videoWidth,t.videoHeight,0):typeof VideoFrame<"u"&&t instanceof VideoFrame?e.set(t.displayWidth,t.displayHeight,0):t!==null?e.set(t.width,t.height,t.depth||0):e.set(0,0,0),e}set needsUpdate(e){e===!0&&this.version++}toJSON(e){const t=e===void 0||typeof e=="string";if(!t&&e.images[this.uuid]!==void 0)return e.images[this.uuid];const i={uuid:this.uuid,url:""},s=this.data;if(s!==null){let r;if(Array.isArray(s)){r=[];for(let a=0,o=s.length;a<o;a++)s[a].isDataTexture?r.push(La(s[a].image)):r.push(La(s[a]))}else r=La(s);i.url=r}return t||(e.images[this.uuid]=i),i}}function La(n){return typeof HTMLImageElement<"u"&&n instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&n instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&n instanceof ImageBitmap?Bd.getDataURL(n):n.data?{data:Array.from(n.data),width:n.width,height:n.height,type:n.data.constructor.name}:(Ne("Texture: Unable to serialize Texture."),{})}let kd=0;const Da=new N;class zt extends fi{constructor(e=zt.DEFAULT_IMAGE,t=zt.DEFAULT_MAPPING,i=kn,s=kn,r=Bt,a=Ti,o=pn,c=an,l=zt.DEFAULT_ANISOTROPY,u=si){super(),this.isTexture=!0,Object.defineProperty(this,"id",{value:kd++}),this.uuid=oi(),this.name="",this.source=new bl(e),this.mipmaps=[],this.mapping=t,this.channel=0,this.wrapS=i,this.wrapT=s,this.magFilter=r,this.minFilter=a,this.anisotropy=l,this.format=o,this.internalFormat=null,this.type=c,this.offset=new Oe(0,0),this.repeat=new Oe(1,1),this.center=new Oe(0,0),this.rotation=0,this.matrixAutoUpdate=!0,this.matrix=new He,this.generateMipmaps=!0,this.premultiplyAlpha=!1,this.flipY=!0,this.unpackAlignment=4,this.colorSpace=u,this.userData={},this.updateRanges=[],this.version=0,this.onUpdate=null,this.renderTarget=null,this.isRenderTargetTexture=!1,this.isArrayTexture=!!(e&&e.depth&&e.depth>1),this.pmremVersion=0,this.normalized=!1}get width(){return this.source.getSize(Da).x}get height(){return this.source.getSize(Da).y}get depth(){return this.source.getSize(Da).z}get image(){return this.source.data}set image(e){this.source.data=e}updateMatrix(){this.matrix.setUvTransform(this.offset.x,this.offset.y,this.repeat.x,this.repeat.y,this.rotation,this.center.x,this.center.y)}addUpdateRange(e,t){this.updateRanges.push({start:e,count:t})}clearUpdateRanges(){this.updateRanges.length=0}clone(){return new this.constructor().copy(this)}copy(e){return this.name=e.name,this.source=e.source,this.mipmaps=e.mipmaps.slice(0),this.mapping=e.mapping,this.channel=e.channel,this.wrapS=e.wrapS,this.wrapT=e.wrapT,this.magFilter=e.magFilter,this.minFilter=e.minFilter,this.anisotropy=e.anisotropy,this.format=e.format,this.internalFormat=e.internalFormat,this.type=e.type,this.normalized=e.normalized,this.offset.copy(e.offset),this.repeat.copy(e.repeat),this.center.copy(e.center),this.rotation=e.rotation,this.matrixAutoUpdate=e.matrixAutoUpdate,this.matrix.copy(e.matrix),this.generateMipmaps=e.generateMipmaps,this.premultiplyAlpha=e.premultiplyAlpha,this.flipY=e.flipY,this.unpackAlignment=e.unpackAlignment,this.colorSpace=e.colorSpace,this.renderTarget=e.renderTarget,this.isRenderTargetTexture=e.isRenderTargetTexture,this.isArrayTexture=e.isArrayTexture,this.userData=JSON.parse(JSON.stringify(e.userData)),this.needsUpdate=!0,this}setValues(e){for(const t in e){const i=e[t];if(i===void 0){Ne(`Texture.setValues(): parameter '${t}' has value of undefined.`);continue}const s=this[t];if(s===void 0){Ne(`Texture.setValues(): property '${t}' does not exist.`);continue}s&&i&&s.isVector2&&i.isVector2||s&&i&&s.isVector3&&i.isVector3||s&&i&&s.isMatrix3&&i.isMatrix3?s.copy(i):this[t]=i}}toJSON(e){const t=e===void 0||typeof e=="string";if(!t&&e.textures[this.uuid]!==void 0)return e.textures[this.uuid];const i={metadata:{version:4.7,type:"Texture",generator:"Texture.toJSON"},uuid:this.uuid,name:this.name,image:this.source.toJSON(e).uuid,mapping:this.mapping,channel:this.channel,repeat:[this.repeat.x,this.repeat.y],offset:[this.offset.x,this.offset.y],center:[this.center.x,this.center.y],rotation:this.rotation,wrap:[this.wrapS,this.wrapT],format:this.format,internalFormat:this.internalFormat,type:this.type,normalized:this.normalized,colorSpace:this.colorSpace,minFilter:this.minFilter,magFilter:this.magFilter,anisotropy:this.anisotropy,flipY:this.flipY,generateMipmaps:this.generateMipmaps,premultiplyAlpha:this.premultiplyAlpha,unpackAlignment:this.unpackAlignment};return Object.keys(this.userData).length>0&&(i.userData=this.userData),t||(e.textures[this.uuid]=i),i}dispose(){this.dispatchEvent({type:"dispose"})}transformUv(e){if(this.mapping!==Eh)return e;if(e.applyMatrix3(this.matrix),e.x<0||e.x>1)switch(this.wrapS){case xo:e.x=e.x-Math.floor(e.x);break;case kn:e.x=e.x<0?0:1;break;case vo:Math.abs(Math.floor(e.x)%2)===1?e.x=Math.ceil(e.x)-e.x:e.x=e.x-Math.floor(e.x);break}if(e.y<0||e.y>1)switch(this.wrapT){case xo:e.y=e.y-Math.floor(e.y);break;case kn:e.y=e.y<0?0:1;break;case vo:Math.abs(Math.floor(e.y)%2)===1?e.y=Math.ceil(e.y)-e.y:e.y=e.y-Math.floor(e.y);break}return this.flipY&&(e.y=1-e.y),e}set needsUpdate(e){e===!0&&(this.version++,this.source.needsUpdate=!0)}set needsPMREMUpdate(e){e===!0&&this.pmremVersion++}}zt.DEFAULT_IMAGE=null;zt.DEFAULT_MAPPING=Eh;zt.DEFAULT_ANISOTROPY=1;class Et{static{Et.prototype.isVector4=!0}constructor(e=0,t=0,i=0,s=1){this.x=e,this.y=t,this.z=i,this.w=s}get width(){return this.z}set width(e){this.z=e}get height(){return this.w}set height(e){this.w=e}set(e,t,i,s){return this.x=e,this.y=t,this.z=i,this.w=s,this}setScalar(e){return this.x=e,this.y=e,this.z=e,this.w=e,this}setX(e){return this.x=e,this}setY(e){return this.y=e,this}setZ(e){return this.z=e,this}setW(e){return this.w=e,this}setComponent(e,t){switch(e){case 0:this.x=t;break;case 1:this.y=t;break;case 2:this.z=t;break;case 3:this.w=t;break;default:throw new Error("index is out of range: "+e)}return this}getComponent(e){switch(e){case 0:return this.x;case 1:return this.y;case 2:return this.z;case 3:return this.w;default:throw new Error("index is out of range: "+e)}}clone(){return new this.constructor(this.x,this.y,this.z,this.w)}copy(e){return this.x=e.x,this.y=e.y,this.z=e.z,this.w=e.w!==void 0?e.w:1,this}add(e){return this.x+=e.x,this.y+=e.y,this.z+=e.z,this.w+=e.w,this}addScalar(e){return this.x+=e,this.y+=e,this.z+=e,this.w+=e,this}addVectors(e,t){return this.x=e.x+t.x,this.y=e.y+t.y,this.z=e.z+t.z,this.w=e.w+t.w,this}addScaledVector(e,t){return this.x+=e.x*t,this.y+=e.y*t,this.z+=e.z*t,this.w+=e.w*t,this}sub(e){return this.x-=e.x,this.y-=e.y,this.z-=e.z,this.w-=e.w,this}subScalar(e){return this.x-=e,this.y-=e,this.z-=e,this.w-=e,this}subVectors(e,t){return this.x=e.x-t.x,this.y=e.y-t.y,this.z=e.z-t.z,this.w=e.w-t.w,this}multiply(e){return this.x*=e.x,this.y*=e.y,this.z*=e.z,this.w*=e.w,this}multiplyScalar(e){return this.x*=e,this.y*=e,this.z*=e,this.w*=e,this}applyMatrix4(e){const t=this.x,i=this.y,s=this.z,r=this.w,a=e.elements;return this.x=a[0]*t+a[4]*i+a[8]*s+a[12]*r,this.y=a[1]*t+a[5]*i+a[9]*s+a[13]*r,this.z=a[2]*t+a[6]*i+a[10]*s+a[14]*r,this.w=a[3]*t+a[7]*i+a[11]*s+a[15]*r,this}divide(e){return this.x/=e.x,this.y/=e.y,this.z/=e.z,this.w/=e.w,this}divideScalar(e){return this.multiplyScalar(1/e)}setAxisAngleFromQuaternion(e){this.w=2*Math.acos(e.w);const t=Math.sqrt(1-e.w*e.w);return t<1e-4?(this.x=1,this.y=0,this.z=0):(this.x=e.x/t,this.y=e.y/t,this.z=e.z/t),this}setAxisAngleFromRotationMatrix(e){let t,i,s,r;const c=e.elements,l=c[0],u=c[4],g=c[8],h=c[1],m=c[5],_=c[9],S=c[2],p=c[6],d=c[10];if(Math.abs(u-h)<.01&&Math.abs(g-S)<.01&&Math.abs(_-p)<.01){if(Math.abs(u+h)<.1&&Math.abs(g+S)<.1&&Math.abs(_+p)<.1&&Math.abs(l+m+d-3)<.1)return this.set(1,0,0,0),this;t=Math.PI;const A=(l+1)/2,T=(m+1)/2,L=(d+1)/2,b=(u+h)/4,I=(g+S)/4,M=(_+p)/4;return A>T&&A>L?A<.01?(i=0,s=.707106781,r=.707106781):(i=Math.sqrt(A),s=b/i,r=I/i):T>L?T<.01?(i=.707106781,s=0,r=.707106781):(s=Math.sqrt(T),i=b/s,r=M/s):L<.01?(i=.707106781,s=.707106781,r=0):(r=Math.sqrt(L),i=I/r,s=M/r),this.set(i,s,r,t),this}let y=Math.sqrt((p-_)*(p-_)+(g-S)*(g-S)+(h-u)*(h-u));return Math.abs(y)<.001&&(y=1),this.x=(p-_)/y,this.y=(g-S)/y,this.z=(h-u)/y,this.w=Math.acos((l+m+d-1)/2),this}setFromMatrixPosition(e){const t=e.elements;return this.x=t[12],this.y=t[13],this.z=t[14],this.w=t[15],this}min(e){return this.x=Math.min(this.x,e.x),this.y=Math.min(this.y,e.y),this.z=Math.min(this.z,e.z),this.w=Math.min(this.w,e.w),this}max(e){return this.x=Math.max(this.x,e.x),this.y=Math.max(this.y,e.y),this.z=Math.max(this.z,e.z),this.w=Math.max(this.w,e.w),this}clamp(e,t){return this.x=je(this.x,e.x,t.x),this.y=je(this.y,e.y,t.y),this.z=je(this.z,e.z,t.z),this.w=je(this.w,e.w,t.w),this}clampScalar(e,t){return this.x=je(this.x,e,t),this.y=je(this.y,e,t),this.z=je(this.z,e,t),this.w=je(this.w,e,t),this}clampLength(e,t){const i=this.length();return this.divideScalar(i||1).multiplyScalar(je(i,e,t))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this.w=Math.floor(this.w),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this.w=Math.ceil(this.w),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this.w=Math.round(this.w),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this.w=Math.trunc(this.w),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this.w=-this.w,this}dot(e){return this.x*e.x+this.y*e.y+this.z*e.z+this.w*e.w}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)+Math.abs(this.w)}normalize(){return this.divideScalar(this.length()||1)}setLength(e){return this.normalize().multiplyScalar(e)}lerp(e,t){return this.x+=(e.x-this.x)*t,this.y+=(e.y-this.y)*t,this.z+=(e.z-this.z)*t,this.w+=(e.w-this.w)*t,this}lerpVectors(e,t,i){return this.x=e.x+(t.x-e.x)*i,this.y=e.y+(t.y-e.y)*i,this.z=e.z+(t.z-e.z)*i,this.w=e.w+(t.w-e.w)*i,this}equals(e){return e.x===this.x&&e.y===this.y&&e.z===this.z&&e.w===this.w}fromArray(e,t=0){return this.x=e[t],this.y=e[t+1],this.z=e[t+2],this.w=e[t+3],this}toArray(e=[],t=0){return e[t]=this.x,e[t+1]=this.y,e[t+2]=this.z,e[t+3]=this.w,e}fromBufferAttribute(e,t){return this.x=e.getX(t),this.y=e.getY(t),this.z=e.getZ(t),this.w=e.getW(t),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this.w=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z,yield this.w}}class Hd extends fi{constructor(e=1,t=1,i={}){super(),i=Object.assign({generateMipmaps:!1,internalFormat:null,minFilter:Bt,depthBuffer:!0,stencilBuffer:!1,resolveDepthBuffer:!0,resolveStencilBuffer:!0,depthTexture:null,samples:0,count:1,depth:1,multiview:!1},i),this.isRenderTarget=!0,this.width=e,this.height=t,this.depth=i.depth,this.scissor=new Et(0,0,e,t),this.scissorTest=!1,this.viewport=new Et(0,0,e,t),this.textures=[];const s={width:e,height:t,depth:i.depth},r=new zt(s),a=i.count;for(let o=0;o<a;o++)this.textures[o]=r.clone(),this.textures[o].isRenderTargetTexture=!0,this.textures[o].renderTarget=this;this._setTextureOptions(i),this.depthBuffer=i.depthBuffer,this.stencilBuffer=i.stencilBuffer,this.resolveDepthBuffer=i.resolveDepthBuffer,this.resolveStencilBuffer=i.resolveStencilBuffer,this._depthTexture=null,this.depthTexture=i.depthTexture,this.samples=i.samples,this.multiview=i.multiview}_setTextureOptions(e={}){const t={minFilter:Bt,generateMipmaps:!1,flipY:!1,internalFormat:null};e.mapping!==void 0&&(t.mapping=e.mapping),e.wrapS!==void 0&&(t.wrapS=e.wrapS),e.wrapT!==void 0&&(t.wrapT=e.wrapT),e.wrapR!==void 0&&(t.wrapR=e.wrapR),e.magFilter!==void 0&&(t.magFilter=e.magFilter),e.minFilter!==void 0&&(t.minFilter=e.minFilter),e.format!==void 0&&(t.format=e.format),e.type!==void 0&&(t.type=e.type),e.anisotropy!==void 0&&(t.anisotropy=e.anisotropy),e.colorSpace!==void 0&&(t.colorSpace=e.colorSpace),e.flipY!==void 0&&(t.flipY=e.flipY),e.generateMipmaps!==void 0&&(t.generateMipmaps=e.generateMipmaps),e.internalFormat!==void 0&&(t.internalFormat=e.internalFormat);for(let i=0;i<this.textures.length;i++)this.textures[i].setValues(t)}get texture(){return this.textures[0]}set texture(e){this.textures[0]=e}set depthTexture(e){this._depthTexture!==null&&(this._depthTexture.renderTarget=null),e!==null&&(e.renderTarget=this),this._depthTexture=e}get depthTexture(){return this._depthTexture}setSize(e,t,i=1){if(this.width!==e||this.height!==t||this.depth!==i){this.width=e,this.height=t,this.depth=i;for(let s=0,r=this.textures.length;s<r;s++)this.textures[s].image.width=e,this.textures[s].image.height=t,this.textures[s].image.depth=i,this.textures[s].isData3DTexture!==!0&&(this.textures[s].isArrayTexture=this.textures[s].image.depth>1);this.dispose()}this.viewport.set(0,0,e,t),this.scissor.set(0,0,e,t)}clone(){return new this.constructor().copy(this)}copy(e){this.width=e.width,this.height=e.height,this.depth=e.depth,this.scissor.copy(e.scissor),this.scissorTest=e.scissorTest,this.viewport.copy(e.viewport),this.textures.length=0;for(let t=0,i=e.textures.length;t<i;t++){this.textures[t]=e.textures[t].clone(),this.textures[t].isRenderTargetTexture=!0,this.textures[t].renderTarget=this;const s=Object.assign({},e.textures[t].image);this.textures[t].source=new bl(s)}return this.depthBuffer=e.depthBuffer,this.stencilBuffer=e.stencilBuffer,this.resolveDepthBuffer=e.resolveDepthBuffer,this.resolveStencilBuffer=e.resolveStencilBuffer,e.depthTexture!==null&&(this.depthTexture=e.depthTexture.clone()),this.samples=e.samples,this.multiview=e.multiview,this}dispose(){this.dispatchEvent({type:"dispose"})}}class Pn extends Hd{constructor(e=1,t=1,i={}){super(e,t,i),this.isWebGLRenderTarget=!0}}class Dh extends zt{constructor(e=null,t=1,i=1,s=1){super(null),this.isDataArrayTexture=!0,this.image={data:e,width:t,height:i,depth:s},this.magFilter=Lt,this.minFilter=Lt,this.wrapR=kn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1,this.layerUpdates=new Set}addLayerUpdate(e){this.layerUpdates.add(e)}clearLayerUpdates(){this.layerUpdates.clear()}}class Gd extends zt{constructor(e=null,t=1,i=1,s=1){super(null),this.isData3DTexture=!0,this.image={data:e,width:t,height:i,depth:s},this.magFilter=Lt,this.minFilter=Lt,this.wrapR=kn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}}class gt{static{gt.prototype.isMatrix4=!0}constructor(e,t,i,s,r,a,o,c,l,u,g,h,m,_,S,p){this.elements=[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],e!==void 0&&this.set(e,t,i,s,r,a,o,c,l,u,g,h,m,_,S,p)}set(e,t,i,s,r,a,o,c,l,u,g,h,m,_,S,p){const d=this.elements;return d[0]=e,d[4]=t,d[8]=i,d[12]=s,d[1]=r,d[5]=a,d[9]=o,d[13]=c,d[2]=l,d[6]=u,d[10]=g,d[14]=h,d[3]=m,d[7]=_,d[11]=S,d[15]=p,this}identity(){return this.set(1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1),this}clone(){return new gt().fromArray(this.elements)}copy(e){const t=this.elements,i=e.elements;return t[0]=i[0],t[1]=i[1],t[2]=i[2],t[3]=i[3],t[4]=i[4],t[5]=i[5],t[6]=i[6],t[7]=i[7],t[8]=i[8],t[9]=i[9],t[10]=i[10],t[11]=i[11],t[12]=i[12],t[13]=i[13],t[14]=i[14],t[15]=i[15],this}copyPosition(e){const t=this.elements,i=e.elements;return t[12]=i[12],t[13]=i[13],t[14]=i[14],this}setFromMatrix3(e){const t=e.elements;return this.set(t[0],t[3],t[6],0,t[1],t[4],t[7],0,t[2],t[5],t[8],0,0,0,0,1),this}extractBasis(e,t,i){return this.determinant()===0?(e.set(1,0,0),t.set(0,1,0),i.set(0,0,1),this):(e.setFromMatrixColumn(this,0),t.setFromMatrixColumn(this,1),i.setFromMatrixColumn(this,2),this)}makeBasis(e,t,i){return this.set(e.x,t.x,i.x,0,e.y,t.y,i.y,0,e.z,t.z,i.z,0,0,0,0,1),this}extractRotation(e){if(e.determinant()===0)return this.identity();const t=this.elements,i=e.elements,s=1/Hi.setFromMatrixColumn(e,0).length(),r=1/Hi.setFromMatrixColumn(e,1).length(),a=1/Hi.setFromMatrixColumn(e,2).length();return t[0]=i[0]*s,t[1]=i[1]*s,t[2]=i[2]*s,t[3]=0,t[4]=i[4]*r,t[5]=i[5]*r,t[6]=i[6]*r,t[7]=0,t[8]=i[8]*a,t[9]=i[9]*a,t[10]=i[10]*a,t[11]=0,t[12]=0,t[13]=0,t[14]=0,t[15]=1,this}makeRotationFromEuler(e){const t=this.elements,i=e.x,s=e.y,r=e.z,a=Math.cos(i),o=Math.sin(i),c=Math.cos(s),l=Math.sin(s),u=Math.cos(r),g=Math.sin(r);if(e.order==="XYZ"){const h=a*u,m=a*g,_=o*u,S=o*g;t[0]=c*u,t[4]=-c*g,t[8]=l,t[1]=m+_*l,t[5]=h-S*l,t[9]=-o*c,t[2]=S-h*l,t[6]=_+m*l,t[10]=a*c}else if(e.order==="YXZ"){const h=c*u,m=c*g,_=l*u,S=l*g;t[0]=h+S*o,t[4]=_*o-m,t[8]=a*l,t[1]=a*g,t[5]=a*u,t[9]=-o,t[2]=m*o-_,t[6]=S+h*o,t[10]=a*c}else if(e.order==="ZXY"){const h=c*u,m=c*g,_=l*u,S=l*g;t[0]=h-S*o,t[4]=-a*g,t[8]=_+m*o,t[1]=m+_*o,t[5]=a*u,t[9]=S-h*o,t[2]=-a*l,t[6]=o,t[10]=a*c}else if(e.order==="ZYX"){const h=a*u,m=a*g,_=o*u,S=o*g;t[0]=c*u,t[4]=_*l-m,t[8]=h*l+S,t[1]=c*g,t[5]=S*l+h,t[9]=m*l-_,t[2]=-l,t[6]=o*c,t[10]=a*c}else if(e.order==="YZX"){const h=a*c,m=a*l,_=o*c,S=o*l;t[0]=c*u,t[4]=S-h*g,t[8]=_*g+m,t[1]=g,t[5]=a*u,t[9]=-o*u,t[2]=-l*u,t[6]=m*g+_,t[10]=h-S*g}else if(e.order==="XZY"){const h=a*c,m=a*l,_=o*c,S=o*l;t[0]=c*u,t[4]=-g,t[8]=l*u,t[1]=h*g+S,t[5]=a*u,t[9]=m*g-_,t[2]=_*g-m,t[6]=o*u,t[10]=S*g+h}return t[3]=0,t[7]=0,t[11]=0,t[12]=0,t[13]=0,t[14]=0,t[15]=1,this}makeRotationFromQuaternion(e){return this.compose(Vd,e,Wd)}lookAt(e,t,i){const s=this.elements;return Jt.subVectors(e,t),Jt.lengthSq()===0&&(Jt.z=1),Jt.normalize(),Zn.crossVectors(i,Jt),Zn.lengthSq()===0&&(Math.abs(i.z)===1?Jt.x+=1e-4:Jt.z+=1e-4,Jt.normalize(),Zn.crossVectors(i,Jt)),Zn.normalize(),nr.crossVectors(Jt,Zn),s[0]=Zn.x,s[4]=nr.x,s[8]=Jt.x,s[1]=Zn.y,s[5]=nr.y,s[9]=Jt.y,s[2]=Zn.z,s[6]=nr.z,s[10]=Jt.z,this}multiply(e){return this.multiplyMatrices(this,e)}premultiply(e){return this.multiplyMatrices(e,this)}multiplyMatrices(e,t){const i=e.elements,s=t.elements,r=this.elements,a=i[0],o=i[4],c=i[8],l=i[12],u=i[1],g=i[5],h=i[9],m=i[13],_=i[2],S=i[6],p=i[10],d=i[14],y=i[3],A=i[7],T=i[11],L=i[15],b=s[0],I=s[4],M=s[8],w=s[12],B=s[1],C=s[5],z=s[9],Z=s[13],J=s[2],O=s[6],W=s[10],$=s[14],se=s[3],oe=s[7],he=s[11],Ee=s[15];return r[0]=a*b+o*B+c*J+l*se,r[4]=a*I+o*C+c*O+l*oe,r[8]=a*M+o*z+c*W+l*he,r[12]=a*w+o*Z+c*$+l*Ee,r[1]=u*b+g*B+h*J+m*se,r[5]=u*I+g*C+h*O+m*oe,r[9]=u*M+g*z+h*W+m*he,r[13]=u*w+g*Z+h*$+m*Ee,r[2]=_*b+S*B+p*J+d*se,r[6]=_*I+S*C+p*O+d*oe,r[10]=_*M+S*z+p*W+d*he,r[14]=_*w+S*Z+p*$+d*Ee,r[3]=y*b+A*B+T*J+L*se,r[7]=y*I+A*C+T*O+L*oe,r[11]=y*M+A*z+T*W+L*he,r[15]=y*w+A*Z+T*$+L*Ee,this}multiplyScalar(e){const t=this.elements;return t[0]*=e,t[4]*=e,t[8]*=e,t[12]*=e,t[1]*=e,t[5]*=e,t[9]*=e,t[13]*=e,t[2]*=e,t[6]*=e,t[10]*=e,t[14]*=e,t[3]*=e,t[7]*=e,t[11]*=e,t[15]*=e,this}determinant(){const e=this.elements,t=e[0],i=e[4],s=e[8],r=e[12],a=e[1],o=e[5],c=e[9],l=e[13],u=e[2],g=e[6],h=e[10],m=e[14],_=e[3],S=e[7],p=e[11],d=e[15],y=c*m-l*h,A=o*m-l*g,T=o*h-c*g,L=a*m-l*u,b=a*h-c*u,I=a*g-o*u;return t*(S*y-p*A+d*T)-i*(_*y-p*L+d*b)+s*(_*A-S*L+d*I)-r*(_*T-S*b+p*I)}transpose(){const e=this.elements;let t;return t=e[1],e[1]=e[4],e[4]=t,t=e[2],e[2]=e[8],e[8]=t,t=e[6],e[6]=e[9],e[9]=t,t=e[3],e[3]=e[12],e[12]=t,t=e[7],e[7]=e[13],e[13]=t,t=e[11],e[11]=e[14],e[14]=t,this}setPosition(e,t,i){const s=this.elements;return e.isVector3?(s[12]=e.x,s[13]=e.y,s[14]=e.z):(s[12]=e,s[13]=t,s[14]=i),this}invert(){const e=this.elements,t=e[0],i=e[1],s=e[2],r=e[3],a=e[4],o=e[5],c=e[6],l=e[7],u=e[8],g=e[9],h=e[10],m=e[11],_=e[12],S=e[13],p=e[14],d=e[15],y=t*o-i*a,A=t*c-s*a,T=t*l-r*a,L=i*c-s*o,b=i*l-r*o,I=s*l-r*c,M=u*S-g*_,w=u*p-h*_,B=u*d-m*_,C=g*p-h*S,z=g*d-m*S,Z=h*d-m*p,J=y*Z-A*z+T*C+L*B-b*w+I*M;if(J===0)return this.set(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);const O=1/J;return e[0]=(o*Z-c*z+l*C)*O,e[1]=(s*z-i*Z-r*C)*O,e[2]=(S*I-p*b+d*L)*O,e[3]=(h*b-g*I-m*L)*O,e[4]=(c*B-a*Z-l*w)*O,e[5]=(t*Z-s*B+r*w)*O,e[6]=(p*T-_*I-d*A)*O,e[7]=(u*I-h*T+m*A)*O,e[8]=(a*z-o*B+l*M)*O,e[9]=(i*B-t*z-r*M)*O,e[10]=(_*b-S*T+d*y)*O,e[11]=(g*T-u*b-m*y)*O,e[12]=(o*w-a*C-c*M)*O,e[13]=(t*C-i*w+s*M)*O,e[14]=(S*A-_*L-p*y)*O,e[15]=(u*L-g*A+h*y)*O,this}scale(e){const t=this.elements,i=e.x,s=e.y,r=e.z;return t[0]*=i,t[4]*=s,t[8]*=r,t[1]*=i,t[5]*=s,t[9]*=r,t[2]*=i,t[6]*=s,t[10]*=r,t[3]*=i,t[7]*=s,t[11]*=r,this}getMaxScaleOnAxis(){const e=this.elements,t=e[0]*e[0]+e[1]*e[1]+e[2]*e[2],i=e[4]*e[4]+e[5]*e[5]+e[6]*e[6],s=e[8]*e[8]+e[9]*e[9]+e[10]*e[10];return Math.sqrt(Math.max(t,i,s))}makeTranslation(e,t,i){return e.isVector3?this.set(1,0,0,e.x,0,1,0,e.y,0,0,1,e.z,0,0,0,1):this.set(1,0,0,e,0,1,0,t,0,0,1,i,0,0,0,1),this}makeRotationX(e){const t=Math.cos(e),i=Math.sin(e);return this.set(1,0,0,0,0,t,-i,0,0,i,t,0,0,0,0,1),this}makeRotationY(e){const t=Math.cos(e),i=Math.sin(e);return this.set(t,0,i,0,0,1,0,0,-i,0,t,0,0,0,0,1),this}makeRotationZ(e){const t=Math.cos(e),i=Math.sin(e);return this.set(t,-i,0,0,i,t,0,0,0,0,1,0,0,0,0,1),this}makeRotationAxis(e,t){const i=Math.cos(t),s=Math.sin(t),r=1-i,a=e.x,o=e.y,c=e.z,l=r*a,u=r*o;return this.set(l*a+i,l*o-s*c,l*c+s*o,0,l*o+s*c,u*o+i,u*c-s*a,0,l*c-s*o,u*c+s*a,r*c*c+i,0,0,0,0,1),this}makeScale(e,t,i){return this.set(e,0,0,0,0,t,0,0,0,0,i,0,0,0,0,1),this}makeShear(e,t,i,s,r,a){return this.set(1,i,r,0,e,1,a,0,t,s,1,0,0,0,0,1),this}compose(e,t,i){const s=this.elements,r=t._x,a=t._y,o=t._z,c=t._w,l=r+r,u=a+a,g=o+o,h=r*l,m=r*u,_=r*g,S=a*u,p=a*g,d=o*g,y=c*l,A=c*u,T=c*g,L=i.x,b=i.y,I=i.z;return s[0]=(1-(S+d))*L,s[1]=(m+T)*L,s[2]=(_-A)*L,s[3]=0,s[4]=(m-T)*b,s[5]=(1-(h+d))*b,s[6]=(p+y)*b,s[7]=0,s[8]=(_+A)*I,s[9]=(p-y)*I,s[10]=(1-(h+S))*I,s[11]=0,s[12]=e.x,s[13]=e.y,s[14]=e.z,s[15]=1,this}decompose(e,t,i){const s=this.elements;e.x=s[12],e.y=s[13],e.z=s[14];const r=this.determinant();if(r===0)return i.set(1,1,1),t.identity(),this;let a=Hi.set(s[0],s[1],s[2]).length();const o=Hi.set(s[4],s[5],s[6]).length(),c=Hi.set(s[8],s[9],s[10]).length();r<0&&(a=-a),cn.copy(this);const l=1/a,u=1/o,g=1/c;return cn.elements[0]*=l,cn.elements[1]*=l,cn.elements[2]*=l,cn.elements[4]*=u,cn.elements[5]*=u,cn.elements[6]*=u,cn.elements[8]*=g,cn.elements[9]*=g,cn.elements[10]*=g,t.setFromRotationMatrix(cn),i.x=a,i.y=o,i.z=c,this}makePerspective(e,t,i,s,r,a,o=wn,c=!1){const l=this.elements,u=2*r/(t-e),g=2*r/(i-s),h=(t+e)/(t-e),m=(i+s)/(i-s);let _,S;if(c)_=r/(a-r),S=a*r/(a-r);else if(o===wn)_=-(a+r)/(a-r),S=-2*a*r/(a-r);else if(o===ra)_=-a/(a-r),S=-a*r/(a-r);else throw new Error("THREE.Matrix4.makePerspective(): Invalid coordinate system: "+o);return l[0]=u,l[4]=0,l[8]=h,l[12]=0,l[1]=0,l[5]=g,l[9]=m,l[13]=0,l[2]=0,l[6]=0,l[10]=_,l[14]=S,l[3]=0,l[7]=0,l[11]=-1,l[15]=0,this}makeOrthographic(e,t,i,s,r,a,o=wn,c=!1){const l=this.elements,u=2/(t-e),g=2/(i-s),h=-(t+e)/(t-e),m=-(i+s)/(i-s);let _,S;if(c)_=1/(a-r),S=a/(a-r);else if(o===wn)_=-2/(a-r),S=-(a+r)/(a-r);else if(o===ra)_=-1/(a-r),S=-r/(a-r);else throw new Error("THREE.Matrix4.makeOrthographic(): Invalid coordinate system: "+o);return l[0]=u,l[4]=0,l[8]=0,l[12]=h,l[1]=0,l[5]=g,l[9]=0,l[13]=m,l[2]=0,l[6]=0,l[10]=_,l[14]=S,l[3]=0,l[7]=0,l[11]=0,l[15]=1,this}equals(e){const t=this.elements,i=e.elements;for(let s=0;s<16;s++)if(t[s]!==i[s])return!1;return!0}fromArray(e,t=0){for(let i=0;i<16;i++)this.elements[i]=e[i+t];return this}toArray(e=[],t=0){const i=this.elements;return e[t]=i[0],e[t+1]=i[1],e[t+2]=i[2],e[t+3]=i[3],e[t+4]=i[4],e[t+5]=i[5],e[t+6]=i[6],e[t+7]=i[7],e[t+8]=i[8],e[t+9]=i[9],e[t+10]=i[10],e[t+11]=i[11],e[t+12]=i[12],e[t+13]=i[13],e[t+14]=i[14],e[t+15]=i[15],e}}const Hi=new N,cn=new gt,Vd=new N(0,0,0),Wd=new N(1,1,1),Zn=new N,nr=new N,Jt=new N,hc=new gt,uc=new hi;class Li{constructor(e=0,t=0,i=0,s=Li.DEFAULT_ORDER){this.isEuler=!0,this._x=e,this._y=t,this._z=i,this._order=s}get x(){return this._x}set x(e){this._x=e,this._onChangeCallback()}get y(){return this._y}set y(e){this._y=e,this._onChangeCallback()}get z(){return this._z}set z(e){this._z=e,this._onChangeCallback()}get order(){return this._order}set order(e){this._order=e,this._onChangeCallback()}set(e,t,i,s=this._order){return this._x=e,this._y=t,this._z=i,this._order=s,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._order)}copy(e){return this._x=e._x,this._y=e._y,this._z=e._z,this._order=e._order,this._onChangeCallback(),this}setFromRotationMatrix(e,t=this._order,i=!0){const s=e.elements,r=s[0],a=s[4],o=s[8],c=s[1],l=s[5],u=s[9],g=s[2],h=s[6],m=s[10];switch(t){case"XYZ":this._y=Math.asin(je(o,-1,1)),Math.abs(o)<.9999999?(this._x=Math.atan2(-u,m),this._z=Math.atan2(-a,r)):(this._x=Math.atan2(h,l),this._z=0);break;case"YXZ":this._x=Math.asin(-je(u,-1,1)),Math.abs(u)<.9999999?(this._y=Math.atan2(o,m),this._z=Math.atan2(c,l)):(this._y=Math.atan2(-g,r),this._z=0);break;case"ZXY":this._x=Math.asin(je(h,-1,1)),Math.abs(h)<.9999999?(this._y=Math.atan2(-g,m),this._z=Math.atan2(-a,l)):(this._y=0,this._z=Math.atan2(c,r));break;case"ZYX":this._y=Math.asin(-je(g,-1,1)),Math.abs(g)<.9999999?(this._x=Math.atan2(h,m),this._z=Math.atan2(c,r)):(this._x=0,this._z=Math.atan2(-a,l));break;case"YZX":this._z=Math.asin(je(c,-1,1)),Math.abs(c)<.9999999?(this._x=Math.atan2(-u,l),this._y=Math.atan2(-g,r)):(this._x=0,this._y=Math.atan2(o,m));break;case"XZY":this._z=Math.asin(-je(a,-1,1)),Math.abs(a)<.9999999?(this._x=Math.atan2(h,l),this._y=Math.atan2(o,r)):(this._x=Math.atan2(-u,m),this._y=0);break;default:Ne("Euler: .setFromRotationMatrix() encountered an unknown order: "+t)}return this._order=t,i===!0&&this._onChangeCallback(),this}setFromQuaternion(e,t,i){return hc.makeRotationFromQuaternion(e),this.setFromRotationMatrix(hc,t,i)}setFromVector3(e,t=this._order){return this.set(e.x,e.y,e.z,t)}reorder(e){return uc.setFromEuler(this),this.setFromQuaternion(uc,e)}equals(e){return e._x===this._x&&e._y===this._y&&e._z===this._z&&e._order===this._order}fromArray(e){return this._x=e[0],this._y=e[1],this._z=e[2],e[3]!==void 0&&(this._order=e[3]),this._onChangeCallback(),this}toArray(e=[],t=0){return e[t]=this._x,e[t+1]=this._y,e[t+2]=this._z,e[t+3]=this._order,e}_onChange(e){return this._onChangeCallback=e,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._order}}Li.DEFAULT_ORDER="XYZ";class Ih{constructor(){this.mask=1}set(e){this.mask=(1<<e|0)>>>0}enable(e){this.mask|=1<<e|0}enableAll(){this.mask=-1}toggle(e){this.mask^=1<<e|0}disable(e){this.mask&=~(1<<e|0)}disableAll(){this.mask=0}test(e){return(this.mask&e.mask)!==0}isEnabled(e){return(this.mask&(1<<e|0))!==0}}let Xd=0;const dc=new N,Gi=new hi,In=new gt,ir=new N,Ms=new N,$d=new N,Yd=new hi,fc=new N(1,0,0),pc=new N(0,1,0),mc=new N(0,0,1),gc={type:"added"},qd={type:"removed"},Vi={type:"childadded",child:null},Ia={type:"childremoved",child:null};class kt extends fi{constructor(){super(),this.isObject3D=!0,Object.defineProperty(this,"id",{value:Xd++}),this.uuid=oi(),this.name="",this.type="Object3D",this.parent=null,this.children=[],this.up=kt.DEFAULT_UP.clone();const e=new N,t=new Li,i=new hi,s=new N(1,1,1);function r(){i.setFromEuler(t,!1)}function a(){t.setFromQuaternion(i,void 0,!1)}t._onChange(r),i._onChange(a),Object.defineProperties(this,{position:{configurable:!0,enumerable:!0,value:e},rotation:{configurable:!0,enumerable:!0,value:t},quaternion:{configurable:!0,enumerable:!0,value:i},scale:{configurable:!0,enumerable:!0,value:s},modelViewMatrix:{value:new gt},normalMatrix:{value:new He}}),this.matrix=new gt,this.matrixWorld=new gt,this.matrixAutoUpdate=kt.DEFAULT_MATRIX_AUTO_UPDATE,this.matrixWorldAutoUpdate=kt.DEFAULT_MATRIX_WORLD_AUTO_UPDATE,this.matrixWorldNeedsUpdate=!1,this.layers=new Ih,this.visible=!0,this.castShadow=!1,this.receiveShadow=!1,this.frustumCulled=!0,this.renderOrder=0,this.animations=[],this.customDepthMaterial=void 0,this.customDistanceMaterial=void 0,this.static=!1,this.userData={},this.pivot=null}onBeforeShadow(){}onAfterShadow(){}onBeforeRender(){}onAfterRender(){}applyMatrix4(e){this.matrixAutoUpdate&&this.updateMatrix(),this.matrix.premultiply(e),this.matrix.decompose(this.position,this.quaternion,this.scale)}applyQuaternion(e){return this.quaternion.premultiply(e),this}setRotationFromAxisAngle(e,t){this.quaternion.setFromAxisAngle(e,t)}setRotationFromEuler(e){this.quaternion.setFromEuler(e,!0)}setRotationFromMatrix(e){this.quaternion.setFromRotationMatrix(e)}setRotationFromQuaternion(e){this.quaternion.copy(e)}rotateOnAxis(e,t){return Gi.setFromAxisAngle(e,t),this.quaternion.multiply(Gi),this}rotateOnWorldAxis(e,t){return Gi.setFromAxisAngle(e,t),this.quaternion.premultiply(Gi),this}rotateX(e){return this.rotateOnAxis(fc,e)}rotateY(e){return this.rotateOnAxis(pc,e)}rotateZ(e){return this.rotateOnAxis(mc,e)}translateOnAxis(e,t){return dc.copy(e).applyQuaternion(this.quaternion),this.position.add(dc.multiplyScalar(t)),this}translateX(e){return this.translateOnAxis(fc,e)}translateY(e){return this.translateOnAxis(pc,e)}translateZ(e){return this.translateOnAxis(mc,e)}localToWorld(e){return this.updateWorldMatrix(!0,!1),e.applyMatrix4(this.matrixWorld)}worldToLocal(e){return this.updateWorldMatrix(!0,!1),e.applyMatrix4(In.copy(this.matrixWorld).invert())}lookAt(e,t,i){e.isVector3?ir.copy(e):ir.set(e,t,i);const s=this.parent;this.updateWorldMatrix(!0,!1),Ms.setFromMatrixPosition(this.matrixWorld),this.isCamera||this.isLight?In.lookAt(Ms,ir,this.up):In.lookAt(ir,Ms,this.up),this.quaternion.setFromRotationMatrix(In),s&&(In.extractRotation(s.matrixWorld),Gi.setFromRotationMatrix(In),this.quaternion.premultiply(Gi.invert()))}add(e){if(arguments.length>1){for(let t=0;t<arguments.length;t++)this.add(arguments[t]);return this}return e===this?(Qe("Object3D.add: object can't be added as a child of itself.",e),this):(e&&e.isObject3D?(e.removeFromParent(),e.parent=this,this.children.push(e),e.dispatchEvent(gc),Vi.child=e,this.dispatchEvent(Vi),Vi.child=null):Qe("Object3D.add: object not an instance of THREE.Object3D.",e),this)}remove(e){if(arguments.length>1){for(let i=0;i<arguments.length;i++)this.remove(arguments[i]);return this}const t=this.children.indexOf(e);return t!==-1&&(e.parent=null,this.children.splice(t,1),e.dispatchEvent(qd),Ia.child=e,this.dispatchEvent(Ia),Ia.child=null),this}removeFromParent(){const e=this.parent;return e!==null&&e.remove(this),this}clear(){return this.remove(...this.children)}attach(e){return this.updateWorldMatrix(!0,!1),In.copy(this.matrixWorld).invert(),e.parent!==null&&(e.parent.updateWorldMatrix(!0,!1),In.multiply(e.parent.matrixWorld)),e.applyMatrix4(In),e.removeFromParent(),e.parent=this,this.children.push(e),e.updateWorldMatrix(!1,!0),e.dispatchEvent(gc),Vi.child=e,this.dispatchEvent(Vi),Vi.child=null,this}getObjectById(e){return this.getObjectByProperty("id",e)}getObjectByName(e){return this.getObjectByProperty("name",e)}getObjectByProperty(e,t){if(this[e]===t)return this;for(let i=0,s=this.children.length;i<s;i++){const a=this.children[i].getObjectByProperty(e,t);if(a!==void 0)return a}}getObjectsByProperty(e,t,i=[]){this[e]===t&&i.push(this);const s=this.children;for(let r=0,a=s.length;r<a;r++)s[r].getObjectsByProperty(e,t,i);return i}getWorldPosition(e){return this.updateWorldMatrix(!0,!1),e.setFromMatrixPosition(this.matrixWorld)}getWorldQuaternion(e){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Ms,e,$d),e}getWorldScale(e){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Ms,Yd,e),e}getWorldDirection(e){this.updateWorldMatrix(!0,!1);const t=this.matrixWorld.elements;return e.set(t[8],t[9],t[10]).normalize()}raycast(){}traverse(e){e(this);const t=this.children;for(let i=0,s=t.length;i<s;i++)t[i].traverse(e)}traverseVisible(e){if(this.visible===!1)return;e(this);const t=this.children;for(let i=0,s=t.length;i<s;i++)t[i].traverseVisible(e)}traverseAncestors(e){const t=this.parent;t!==null&&(e(t),t.traverseAncestors(e))}updateMatrix(){this.matrix.compose(this.position,this.quaternion,this.scale);const e=this.pivot;if(e!==null){const t=e.x,i=e.y,s=e.z,r=this.matrix.elements;r[12]+=t-r[0]*t-r[4]*i-r[8]*s,r[13]+=i-r[1]*t-r[5]*i-r[9]*s,r[14]+=s-r[2]*t-r[6]*i-r[10]*s}this.matrixWorldNeedsUpdate=!0}updateMatrixWorld(e){this.matrixAutoUpdate&&this.updateMatrix(),(this.matrixWorldNeedsUpdate||e)&&(this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),this.matrixWorldNeedsUpdate=!1,e=!0);const t=this.children;for(let i=0,s=t.length;i<s;i++)t[i].updateMatrixWorld(e)}updateWorldMatrix(e,t){const i=this.parent;if(e===!0&&i!==null&&i.updateWorldMatrix(!0,!1),this.matrixAutoUpdate&&this.updateMatrix(),this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),t===!0){const s=this.children;for(let r=0,a=s.length;r<a;r++)s[r].updateWorldMatrix(!1,!0)}}toJSON(e){const t=e===void 0||typeof e=="string",i={};t&&(e={geometries:{},materials:{},textures:{},images:{},shapes:{},skeletons:{},animations:{},nodes:{}},i.metadata={version:4.7,type:"Object",generator:"Object3D.toJSON"});const s={};s.uuid=this.uuid,s.type=this.type,this.name!==""&&(s.name=this.name),this.castShadow===!0&&(s.castShadow=!0),this.receiveShadow===!0&&(s.receiveShadow=!0),this.visible===!1&&(s.visible=!1),this.frustumCulled===!1&&(s.frustumCulled=!1),this.renderOrder!==0&&(s.renderOrder=this.renderOrder),this.static!==!1&&(s.static=this.static),Object.keys(this.userData).length>0&&(s.userData=this.userData),s.layers=this.layers.mask,s.matrix=this.matrix.toArray(),s.up=this.up.toArray(),this.pivot!==null&&(s.pivot=this.pivot.toArray()),this.matrixAutoUpdate===!1&&(s.matrixAutoUpdate=!1),this.morphTargetDictionary!==void 0&&(s.morphTargetDictionary=Object.assign({},this.morphTargetDictionary)),this.morphTargetInfluences!==void 0&&(s.morphTargetInfluences=this.morphTargetInfluences.slice()),this.isInstancedMesh&&(s.type="InstancedMesh",s.count=this.count,s.instanceMatrix=this.instanceMatrix.toJSON(),this.instanceColor!==null&&(s.instanceColor=this.instanceColor.toJSON())),this.isBatchedMesh&&(s.type="BatchedMesh",s.perObjectFrustumCulled=this.perObjectFrustumCulled,s.sortObjects=this.sortObjects,s.drawRanges=this._drawRanges,s.reservedRanges=this._reservedRanges,s.geometryInfo=this._geometryInfo.map(o=>({...o,boundingBox:o.boundingBox?o.boundingBox.toJSON():void 0,boundingSphere:o.boundingSphere?o.boundingSphere.toJSON():void 0})),s.instanceInfo=this._instanceInfo.map(o=>({...o})),s.availableInstanceIds=this._availableInstanceIds.slice(),s.availableGeometryIds=this._availableGeometryIds.slice(),s.nextIndexStart=this._nextIndexStart,s.nextVertexStart=this._nextVertexStart,s.geometryCount=this._geometryCount,s.maxInstanceCount=this._maxInstanceCount,s.maxVertexCount=this._maxVertexCount,s.maxIndexCount=this._maxIndexCount,s.geometryInitialized=this._geometryInitialized,s.matricesTexture=this._matricesTexture.toJSON(e),s.indirectTexture=this._indirectTexture.toJSON(e),this._colorsTexture!==null&&(s.colorsTexture=this._colorsTexture.toJSON(e)),this.boundingSphere!==null&&(s.boundingSphere=this.boundingSphere.toJSON()),this.boundingBox!==null&&(s.boundingBox=this.boundingBox.toJSON()));function r(o,c){return o[c.uuid]===void 0&&(o[c.uuid]=c.toJSON(e)),c.uuid}if(this.isScene)this.background&&(this.background.isColor?s.background=this.background.toJSON():this.background.isTexture&&(s.background=this.background.toJSON(e).uuid)),this.environment&&this.environment.isTexture&&this.environment.isRenderTargetTexture!==!0&&(s.environment=this.environment.toJSON(e).uuid);else if(this.isMesh||this.isLine||this.isPoints){s.geometry=r(e.geometries,this.geometry);const o=this.geometry.parameters;if(o!==void 0&&o.shapes!==void 0){const c=o.shapes;if(Array.isArray(c))for(let l=0,u=c.length;l<u;l++){const g=c[l];r(e.shapes,g)}else r(e.shapes,c)}}if(this.isSkinnedMesh&&(s.bindMode=this.bindMode,s.bindMatrix=this.bindMatrix.toArray(),this.skeleton!==void 0&&(r(e.skeletons,this.skeleton),s.skeleton=this.skeleton.uuid)),this.material!==void 0)if(Array.isArray(this.material)){const o=[];for(let c=0,l=this.material.length;c<l;c++)o.push(r(e.materials,this.material[c]));s.material=o}else s.material=r(e.materials,this.material);if(this.children.length>0){s.children=[];for(let o=0;o<this.children.length;o++)s.children.push(this.children[o].toJSON(e).object)}if(this.animations.length>0){s.animations=[];for(let o=0;o<this.animations.length;o++){const c=this.animations[o];s.animations.push(r(e.animations,c))}}if(t){const o=a(e.geometries),c=a(e.materials),l=a(e.textures),u=a(e.images),g=a(e.shapes),h=a(e.skeletons),m=a(e.animations),_=a(e.nodes);o.length>0&&(i.geometries=o),c.length>0&&(i.materials=c),l.length>0&&(i.textures=l),u.length>0&&(i.images=u),g.length>0&&(i.shapes=g),h.length>0&&(i.skeletons=h),m.length>0&&(i.animations=m),_.length>0&&(i.nodes=_)}return i.object=s,i;function a(o){const c=[];for(const l in o){const u=o[l];delete u.metadata,c.push(u)}return c}}clone(e){return new this.constructor().copy(this,e)}copy(e,t=!0){if(this.name=e.name,this.up.copy(e.up),this.position.copy(e.position),this.rotation.order=e.rotation.order,this.quaternion.copy(e.quaternion),this.scale.copy(e.scale),this.pivot=e.pivot!==null?e.pivot.clone():null,this.matrix.copy(e.matrix),this.matrixWorld.copy(e.matrixWorld),this.matrixAutoUpdate=e.matrixAutoUpdate,this.matrixWorldAutoUpdate=e.matrixWorldAutoUpdate,this.matrixWorldNeedsUpdate=e.matrixWorldNeedsUpdate,this.layers.mask=e.layers.mask,this.visible=e.visible,this.castShadow=e.castShadow,this.receiveShadow=e.receiveShadow,this.frustumCulled=e.frustumCulled,this.renderOrder=e.renderOrder,this.static=e.static,this.animations=e.animations.slice(),this.userData=JSON.parse(JSON.stringify(e.userData)),t===!0)for(let i=0;i<e.children.length;i++){const s=e.children[i];this.add(s.clone())}return this}}kt.DEFAULT_UP=new N(0,1,0);kt.DEFAULT_MATRIX_AUTO_UPDATE=!0;kt.DEFAULT_MATRIX_WORLD_AUTO_UPDATE=!0;class sr extends kt{constructor(){super(),this.isGroup=!0,this.type="Group"}}const jd={type:"move"};class Ua{constructor(){this._targetRay=null,this._grip=null,this._hand=null}getHandSpace(){return this._hand===null&&(this._hand=new sr,this._hand.matrixAutoUpdate=!1,this._hand.visible=!1,this._hand.joints={},this._hand.inputState={pinching:!1}),this._hand}getTargetRaySpace(){return this._targetRay===null&&(this._targetRay=new sr,this._targetRay.matrixAutoUpdate=!1,this._targetRay.visible=!1,this._targetRay.hasLinearVelocity=!1,this._targetRay.linearVelocity=new N,this._targetRay.hasAngularVelocity=!1,this._targetRay.angularVelocity=new N),this._targetRay}getGripSpace(){return this._grip===null&&(this._grip=new sr,this._grip.matrixAutoUpdate=!1,this._grip.visible=!1,this._grip.hasLinearVelocity=!1,this._grip.linearVelocity=new N,this._grip.hasAngularVelocity=!1,this._grip.angularVelocity=new N,this._grip.eventsEnabled=!1),this._grip}dispatchEvent(e){return this._targetRay!==null&&this._targetRay.dispatchEvent(e),this._grip!==null&&this._grip.dispatchEvent(e),this._hand!==null&&this._hand.dispatchEvent(e),this}connect(e){if(e&&e.hand){const t=this._hand;if(t)for(const i of e.hand.values())this._getHandJoint(t,i)}return this.dispatchEvent({type:"connected",data:e}),this}disconnect(e){return this.dispatchEvent({type:"disconnected",data:e}),this._targetRay!==null&&(this._targetRay.visible=!1),this._grip!==null&&(this._grip.visible=!1),this._hand!==null&&(this._hand.visible=!1),this}update(e,t,i){let s=null,r=null,a=null;const o=this._targetRay,c=this._grip,l=this._hand;if(e&&t.session.visibilityState!=="visible-blurred"){if(l&&e.hand){a=!0;for(const S of e.hand.values()){const p=t.getJointPose(S,i),d=this._getHandJoint(l,S);p!==null&&(d.matrix.fromArray(p.transform.matrix),d.matrix.decompose(d.position,d.rotation,d.scale),d.matrixWorldNeedsUpdate=!0,d.jointRadius=p.radius),d.visible=p!==null}const u=l.joints["index-finger-tip"],g=l.joints["thumb-tip"],h=u.position.distanceTo(g.position),m=.02,_=.005;l.inputState.pinching&&h>m+_?(l.inputState.pinching=!1,this.dispatchEvent({type:"pinchend",handedness:e.handedness,target:this})):!l.inputState.pinching&&h<=m-_&&(l.inputState.pinching=!0,this.dispatchEvent({type:"pinchstart",handedness:e.handedness,target:this}))}else c!==null&&e.gripSpace&&(r=t.getPose(e.gripSpace,i),r!==null&&(c.matrix.fromArray(r.transform.matrix),c.matrix.decompose(c.position,c.rotation,c.scale),c.matrixWorldNeedsUpdate=!0,r.linearVelocity?(c.hasLinearVelocity=!0,c.linearVelocity.copy(r.linearVelocity)):c.hasLinearVelocity=!1,r.angularVelocity?(c.hasAngularVelocity=!0,c.angularVelocity.copy(r.angularVelocity)):c.hasAngularVelocity=!1,c.eventsEnabled&&c.dispatchEvent({type:"gripUpdated",data:e,target:this})));o!==null&&(s=t.getPose(e.targetRaySpace,i),s===null&&r!==null&&(s=r),s!==null&&(o.matrix.fromArray(s.transform.matrix),o.matrix.decompose(o.position,o.rotation,o.scale),o.matrixWorldNeedsUpdate=!0,s.linearVelocity?(o.hasLinearVelocity=!0,o.linearVelocity.copy(s.linearVelocity)):o.hasLinearVelocity=!1,s.angularVelocity?(o.hasAngularVelocity=!0,o.angularVelocity.copy(s.angularVelocity)):o.hasAngularVelocity=!1,this.dispatchEvent(jd)))}return o!==null&&(o.visible=s!==null),c!==null&&(c.visible=r!==null),l!==null&&(l.visible=a!==null),this}_getHandJoint(e,t){if(e.joints[t.jointName]===void 0){const i=new sr;i.matrixAutoUpdate=!1,i.visible=!1,e.joints[t.jointName]=i,e.add(i)}return e.joints[t.jointName]}}const Uh={aliceblue:15792383,antiquewhite:16444375,aqua:65535,aquamarine:8388564,azure:15794175,beige:16119260,bisque:16770244,black:0,blanchedalmond:16772045,blue:255,blueviolet:9055202,brown:10824234,burlywood:14596231,cadetblue:6266528,chartreuse:8388352,chocolate:13789470,coral:16744272,cornflowerblue:6591981,cornsilk:16775388,crimson:14423100,cyan:65535,darkblue:139,darkcyan:35723,darkgoldenrod:12092939,darkgray:11119017,darkgreen:25600,darkgrey:11119017,darkkhaki:12433259,darkmagenta:9109643,darkolivegreen:5597999,darkorange:16747520,darkorchid:10040012,darkred:9109504,darksalmon:15308410,darkseagreen:9419919,darkslateblue:4734347,darkslategray:3100495,darkslategrey:3100495,darkturquoise:52945,darkviolet:9699539,deeppink:16716947,deepskyblue:49151,dimgray:6908265,dimgrey:6908265,dodgerblue:2003199,firebrick:11674146,floralwhite:16775920,forestgreen:2263842,fuchsia:16711935,gainsboro:14474460,ghostwhite:16316671,gold:16766720,goldenrod:14329120,gray:8421504,green:32768,greenyellow:11403055,grey:8421504,honeydew:15794160,hotpink:16738740,indianred:13458524,indigo:4915330,ivory:16777200,khaki:15787660,lavender:15132410,lavenderblush:16773365,lawngreen:8190976,lemonchiffon:16775885,lightblue:11393254,lightcoral:15761536,lightcyan:14745599,lightgoldenrodyellow:16448210,lightgray:13882323,lightgreen:9498256,lightgrey:13882323,lightpink:16758465,lightsalmon:16752762,lightseagreen:2142890,lightskyblue:8900346,lightslategray:7833753,lightslategrey:7833753,lightsteelblue:11584734,lightyellow:16777184,lime:65280,limegreen:3329330,linen:16445670,magenta:16711935,maroon:8388608,mediumaquamarine:6737322,mediumblue:205,mediumorchid:12211667,mediumpurple:9662683,mediumseagreen:3978097,mediumslateblue:8087790,mediumspringgreen:64154,mediumturquoise:4772300,mediumvioletred:13047173,midnightblue:1644912,mintcream:16121850,mistyrose:16770273,moccasin:16770229,navajowhite:16768685,navy:128,oldlace:16643558,olive:8421376,olivedrab:7048739,orange:16753920,orangered:16729344,orchid:14315734,palegoldenrod:15657130,palegreen:10025880,paleturquoise:11529966,palevioletred:14381203,papayawhip:16773077,peachpuff:16767673,peru:13468991,pink:16761035,plum:14524637,powderblue:11591910,purple:8388736,rebeccapurple:6697881,red:16711680,rosybrown:12357519,royalblue:4286945,saddlebrown:9127187,salmon:16416882,sandybrown:16032864,seagreen:3050327,seashell:16774638,sienna:10506797,silver:12632256,skyblue:8900331,slateblue:6970061,slategray:7372944,slategrey:7372944,snow:16775930,springgreen:65407,steelblue:4620980,tan:13808780,teal:32896,thistle:14204888,tomato:16737095,turquoise:4251856,violet:15631086,wheat:16113331,white:16777215,whitesmoke:16119285,yellow:16776960,yellowgreen:10145074},Jn={h:0,s:0,l:0},rr={h:0,s:0,l:0};function Na(n,e,t){return t<0&&(t+=1),t>1&&(t-=1),t<1/6?n+(e-n)*6*t:t<1/2?e:t<2/3?n+(e-n)*6*(2/3-t):n}class ke{constructor(e,t,i){return this.isColor=!0,this.r=1,this.g=1,this.b=1,this.set(e,t,i)}set(e,t,i){if(t===void 0&&i===void 0){const s=e;s&&s.isColor?this.copy(s):typeof s=="number"?this.setHex(s):typeof s=="string"&&this.setStyle(s)}else this.setRGB(e,t,i);return this}setScalar(e){return this.r=e,this.g=e,this.b=e,this}setHex(e,t=sn){return e=Math.floor(e),this.r=(e>>16&255)/255,this.g=(e>>8&255)/255,this.b=(e&255)/255,Ze.colorSpaceToWorking(this,t),this}setRGB(e,t,i,s=Ze.workingColorSpace){return this.r=e,this.g=t,this.b=i,Ze.colorSpaceToWorking(this,s),this}setHSL(e,t,i,s=Ze.workingColorSpace){if(e=Nd(e,1),t=je(t,0,1),i=je(i,0,1),t===0)this.r=this.g=this.b=i;else{const r=i<=.5?i*(1+t):i+t-i*t,a=2*i-r;this.r=Na(a,r,e+1/3),this.g=Na(a,r,e),this.b=Na(a,r,e-1/3)}return Ze.colorSpaceToWorking(this,s),this}setStyle(e,t=sn){function i(r){r!==void 0&&parseFloat(r)<1&&Ne("Color: Alpha component of "+e+" will be ignored.")}let s;if(s=/^(\w+)\(([^\)]*)\)/.exec(e)){let r;const a=s[1],o=s[2];switch(a){case"rgb":case"rgba":if(r=/^\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return i(r[4]),this.setRGB(Math.min(255,parseInt(r[1],10))/255,Math.min(255,parseInt(r[2],10))/255,Math.min(255,parseInt(r[3],10))/255,t);if(r=/^\s*(\d+)\%\s*,\s*(\d+)\%\s*,\s*(\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return i(r[4]),this.setRGB(Math.min(100,parseInt(r[1],10))/100,Math.min(100,parseInt(r[2],10))/100,Math.min(100,parseInt(r[3],10))/100,t);break;case"hsl":case"hsla":if(r=/^\s*(\d*\.?\d+)\s*,\s*(\d*\.?\d+)\%\s*,\s*(\d*\.?\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return i(r[4]),this.setHSL(parseFloat(r[1])/360,parseFloat(r[2])/100,parseFloat(r[3])/100,t);break;default:Ne("Color: Unknown color model "+e)}}else if(s=/^\#([A-Fa-f\d]+)$/.exec(e)){const r=s[1],a=r.length;if(a===3)return this.setRGB(parseInt(r.charAt(0),16)/15,parseInt(r.charAt(1),16)/15,parseInt(r.charAt(2),16)/15,t);if(a===6)return this.setHex(parseInt(r,16),t);Ne("Color: Invalid hex color "+e)}else if(e&&e.length>0)return this.setColorName(e,t);return this}setColorName(e,t=sn){const i=Uh[e.toLowerCase()];return i!==void 0?this.setHex(i,t):Ne("Color: Unknown color "+e),this}clone(){return new this.constructor(this.r,this.g,this.b)}copy(e){return this.r=e.r,this.g=e.g,this.b=e.b,this}copySRGBToLinear(e){return this.r=Gn(e.r),this.g=Gn(e.g),this.b=Gn(e.b),this}copyLinearToSRGB(e){return this.r=us(e.r),this.g=us(e.g),this.b=us(e.b),this}convertSRGBToLinear(){return this.copySRGBToLinear(this),this}convertLinearToSRGB(){return this.copyLinearToSRGB(this),this}getHex(e=sn){return Ze.workingToColorSpace(Ot.copy(this),e),Math.round(je(Ot.r*255,0,255))*65536+Math.round(je(Ot.g*255,0,255))*256+Math.round(je(Ot.b*255,0,255))}getHexString(e=sn){return("000000"+this.getHex(e).toString(16)).slice(-6)}getHSL(e,t=Ze.workingColorSpace){Ze.workingToColorSpace(Ot.copy(this),t);const i=Ot.r,s=Ot.g,r=Ot.b,a=Math.max(i,s,r),o=Math.min(i,s,r);let c,l;const u=(o+a)/2;if(o===a)c=0,l=0;else{const g=a-o;switch(l=u<=.5?g/(a+o):g/(2-a-o),a){case i:c=(s-r)/g+(s<r?6:0);break;case s:c=(r-i)/g+2;break;case r:c=(i-s)/g+4;break}c/=6}return e.h=c,e.s=l,e.l=u,e}getRGB(e,t=Ze.workingColorSpace){return Ze.workingToColorSpace(Ot.copy(this),t),e.r=Ot.r,e.g=Ot.g,e.b=Ot.b,e}getStyle(e=sn){Ze.workingToColorSpace(Ot.copy(this),e);const t=Ot.r,i=Ot.g,s=Ot.b;return e!==sn?`color(${e} ${t.toFixed(3)} ${i.toFixed(3)} ${s.toFixed(3)})`:`rgb(${Math.round(t*255)},${Math.round(i*255)},${Math.round(s*255)})`}offsetHSL(e,t,i){return this.getHSL(Jn),this.setHSL(Jn.h+e,Jn.s+t,Jn.l+i)}add(e){return this.r+=e.r,this.g+=e.g,this.b+=e.b,this}addColors(e,t){return this.r=e.r+t.r,this.g=e.g+t.g,this.b=e.b+t.b,this}addScalar(e){return this.r+=e,this.g+=e,this.b+=e,this}sub(e){return this.r=Math.max(0,this.r-e.r),this.g=Math.max(0,this.g-e.g),this.b=Math.max(0,this.b-e.b),this}multiply(e){return this.r*=e.r,this.g*=e.g,this.b*=e.b,this}multiplyScalar(e){return this.r*=e,this.g*=e,this.b*=e,this}lerp(e,t){return this.r+=(e.r-this.r)*t,this.g+=(e.g-this.g)*t,this.b+=(e.b-this.b)*t,this}lerpColors(e,t,i){return this.r=e.r+(t.r-e.r)*i,this.g=e.g+(t.g-e.g)*i,this.b=e.b+(t.b-e.b)*i,this}lerpHSL(e,t){this.getHSL(Jn),e.getHSL(rr);const i=Ra(Jn.h,rr.h,t),s=Ra(Jn.s,rr.s,t),r=Ra(Jn.l,rr.l,t);return this.setHSL(i,s,r),this}setFromVector3(e){return this.r=e.x,this.g=e.y,this.b=e.z,this}applyMatrix3(e){const t=this.r,i=this.g,s=this.b,r=e.elements;return this.r=r[0]*t+r[3]*i+r[6]*s,this.g=r[1]*t+r[4]*i+r[7]*s,this.b=r[2]*t+r[5]*i+r[8]*s,this}equals(e){return e.r===this.r&&e.g===this.g&&e.b===this.b}fromArray(e,t=0){return this.r=e[t],this.g=e[t+1],this.b=e[t+2],this}toArray(e=[],t=0){return e[t]=this.r,e[t+1]=this.g,e[t+2]=this.b,e}fromBufferAttribute(e,t){return this.r=e.getX(t),this.g=e.getY(t),this.b=e.getZ(t),this}toJSON(){return this.getHex()}*[Symbol.iterator](){yield this.r,yield this.g,yield this.b}}const Ot=new ke;ke.NAMES=Uh;class Tl{constructor(e,t=25e-5){this.isFogExp2=!0,this.name="",this.color=new ke(e),this.density=t}clone(){return new Tl(this.color,this.density)}toJSON(){return{type:"FogExp2",name:this.name,color:this.color.getHex(),density:this.density}}}class Kd extends kt{constructor(){super(),this.isScene=!0,this.type="Scene",this.background=null,this.environment=null,this.fog=null,this.backgroundBlurriness=0,this.backgroundIntensity=1,this.backgroundRotation=new Li,this.environmentIntensity=1,this.environmentRotation=new Li,this.overrideMaterial=null,typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}copy(e,t){return super.copy(e,t),e.background!==null&&(this.background=e.background.clone()),e.environment!==null&&(this.environment=e.environment.clone()),e.fog!==null&&(this.fog=e.fog.clone()),this.backgroundBlurriness=e.backgroundBlurriness,this.backgroundIntensity=e.backgroundIntensity,this.backgroundRotation.copy(e.backgroundRotation),this.environmentIntensity=e.environmentIntensity,this.environmentRotation.copy(e.environmentRotation),e.overrideMaterial!==null&&(this.overrideMaterial=e.overrideMaterial.clone()),this.matrixAutoUpdate=e.matrixAutoUpdate,this}toJSON(e){const t=super.toJSON(e);return this.fog!==null&&(t.object.fog=this.fog.toJSON()),this.backgroundBlurriness>0&&(t.object.backgroundBlurriness=this.backgroundBlurriness),this.backgroundIntensity!==1&&(t.object.backgroundIntensity=this.backgroundIntensity),t.object.backgroundRotation=this.backgroundRotation.toArray(),this.environmentIntensity!==1&&(t.object.environmentIntensity=this.environmentIntensity),t.object.environmentRotation=this.environmentRotation.toArray(),t}}const hn=new N,Un=new N,Fa=new N,Nn=new N,Wi=new N,Xi=new N,_c=new N,Oa=new N,Ba=new N,za=new N,ka=new Et,Ha=new Et,Ga=new Et;class on{constructor(e=new N,t=new N,i=new N){this.a=e,this.b=t,this.c=i}static getNormal(e,t,i,s){s.subVectors(i,t),hn.subVectors(e,t),s.cross(hn);const r=s.lengthSq();return r>0?s.multiplyScalar(1/Math.sqrt(r)):s.set(0,0,0)}static getBarycoord(e,t,i,s,r){hn.subVectors(s,t),Un.subVectors(i,t),Fa.subVectors(e,t);const a=hn.dot(hn),o=hn.dot(Un),c=hn.dot(Fa),l=Un.dot(Un),u=Un.dot(Fa),g=a*l-o*o;if(g===0)return r.set(0,0,0),null;const h=1/g,m=(l*c-o*u)*h,_=(a*u-o*c)*h;return r.set(1-m-_,_,m)}static containsPoint(e,t,i,s){return this.getBarycoord(e,t,i,s,Nn)===null?!1:Nn.x>=0&&Nn.y>=0&&Nn.x+Nn.y<=1}static getInterpolation(e,t,i,s,r,a,o,c){return this.getBarycoord(e,t,i,s,Nn)===null?(c.x=0,c.y=0,"z"in c&&(c.z=0),"w"in c&&(c.w=0),null):(c.setScalar(0),c.addScaledVector(r,Nn.x),c.addScaledVector(a,Nn.y),c.addScaledVector(o,Nn.z),c)}static getInterpolatedAttribute(e,t,i,s,r,a){return ka.setScalar(0),Ha.setScalar(0),Ga.setScalar(0),ka.fromBufferAttribute(e,t),Ha.fromBufferAttribute(e,i),Ga.fromBufferAttribute(e,s),a.setScalar(0),a.addScaledVector(ka,r.x),a.addScaledVector(Ha,r.y),a.addScaledVector(Ga,r.z),a}static isFrontFacing(e,t,i,s){return hn.subVectors(i,t),Un.subVectors(e,t),hn.cross(Un).dot(s)<0}set(e,t,i){return this.a.copy(e),this.b.copy(t),this.c.copy(i),this}setFromPointsAndIndices(e,t,i,s){return this.a.copy(e[t]),this.b.copy(e[i]),this.c.copy(e[s]),this}setFromAttributeAndIndices(e,t,i,s){return this.a.fromBufferAttribute(e,t),this.b.fromBufferAttribute(e,i),this.c.fromBufferAttribute(e,s),this}clone(){return new this.constructor().copy(this)}copy(e){return this.a.copy(e.a),this.b.copy(e.b),this.c.copy(e.c),this}getArea(){return hn.subVectors(this.c,this.b),Un.subVectors(this.a,this.b),hn.cross(Un).length()*.5}getMidpoint(e){return e.addVectors(this.a,this.b).add(this.c).multiplyScalar(1/3)}getNormal(e){return on.getNormal(this.a,this.b,this.c,e)}getPlane(e){return e.setFromCoplanarPoints(this.a,this.b,this.c)}getBarycoord(e,t){return on.getBarycoord(e,this.a,this.b,this.c,t)}getInterpolation(e,t,i,s,r){return on.getInterpolation(e,this.a,this.b,this.c,t,i,s,r)}containsPoint(e){return on.containsPoint(e,this.a,this.b,this.c)}isFrontFacing(e){return on.isFrontFacing(this.a,this.b,this.c,e)}intersectsBox(e){return e.intersectsTriangle(this)}closestPointToPoint(e,t){const i=this.a,s=this.b,r=this.c;let a,o;Wi.subVectors(s,i),Xi.subVectors(r,i),Oa.subVectors(e,i);const c=Wi.dot(Oa),l=Xi.dot(Oa);if(c<=0&&l<=0)return t.copy(i);Ba.subVectors(e,s);const u=Wi.dot(Ba),g=Xi.dot(Ba);if(u>=0&&g<=u)return t.copy(s);const h=c*g-u*l;if(h<=0&&c>=0&&u<=0)return a=c/(c-u),t.copy(i).addScaledVector(Wi,a);za.subVectors(e,r);const m=Wi.dot(za),_=Xi.dot(za);if(_>=0&&m<=_)return t.copy(r);const S=m*l-c*_;if(S<=0&&l>=0&&_<=0)return o=l/(l-_),t.copy(i).addScaledVector(Xi,o);const p=u*_-m*g;if(p<=0&&g-u>=0&&m-_>=0)return _c.subVectors(r,s),o=(g-u)/(g-u+(m-_)),t.copy(s).addScaledVector(_c,o);const d=1/(p+S+h);return a=S*d,o=h*d,t.copy(i).addScaledVector(Wi,a).addScaledVector(Xi,o)}equals(e){return e.a.equals(this.a)&&e.b.equals(this.b)&&e.c.equals(this.c)}}class qs{constructor(e=new N(1/0,1/0,1/0),t=new N(-1/0,-1/0,-1/0)){this.isBox3=!0,this.min=e,this.max=t}set(e,t){return this.min.copy(e),this.max.copy(t),this}setFromArray(e){this.makeEmpty();for(let t=0,i=e.length;t<i;t+=3)this.expandByPoint(un.fromArray(e,t));return this}setFromBufferAttribute(e){this.makeEmpty();for(let t=0,i=e.count;t<i;t++)this.expandByPoint(un.fromBufferAttribute(e,t));return this}setFromPoints(e){this.makeEmpty();for(let t=0,i=e.length;t<i;t++)this.expandByPoint(e[t]);return this}setFromCenterAndSize(e,t){const i=un.copy(t).multiplyScalar(.5);return this.min.copy(e).sub(i),this.max.copy(e).add(i),this}setFromObject(e,t=!1){return this.makeEmpty(),this.expandByObject(e,t)}clone(){return new this.constructor().copy(this)}copy(e){return this.min.copy(e.min),this.max.copy(e.max),this}makeEmpty(){return this.min.x=this.min.y=this.min.z=1/0,this.max.x=this.max.y=this.max.z=-1/0,this}isEmpty(){return this.max.x<this.min.x||this.max.y<this.min.y||this.max.z<this.min.z}getCenter(e){return this.isEmpty()?e.set(0,0,0):e.addVectors(this.min,this.max).multiplyScalar(.5)}getSize(e){return this.isEmpty()?e.set(0,0,0):e.subVectors(this.max,this.min)}expandByPoint(e){return this.min.min(e),this.max.max(e),this}expandByVector(e){return this.min.sub(e),this.max.add(e),this}expandByScalar(e){return this.min.addScalar(-e),this.max.addScalar(e),this}expandByObject(e,t=!1){e.updateWorldMatrix(!1,!1);const i=e.geometry;if(i!==void 0){const r=i.getAttribute("position");if(t===!0&&r!==void 0&&e.isInstancedMesh!==!0)for(let a=0,o=r.count;a<o;a++)e.isMesh===!0?e.getVertexPosition(a,un):un.fromBufferAttribute(r,a),un.applyMatrix4(e.matrixWorld),this.expandByPoint(un);else e.boundingBox!==void 0?(e.boundingBox===null&&e.computeBoundingBox(),ar.copy(e.boundingBox)):(i.boundingBox===null&&i.computeBoundingBox(),ar.copy(i.boundingBox)),ar.applyMatrix4(e.matrixWorld),this.union(ar)}const s=e.children;for(let r=0,a=s.length;r<a;r++)this.expandByObject(s[r],t);return this}containsPoint(e){return e.x>=this.min.x&&e.x<=this.max.x&&e.y>=this.min.y&&e.y<=this.max.y&&e.z>=this.min.z&&e.z<=this.max.z}containsBox(e){return this.min.x<=e.min.x&&e.max.x<=this.max.x&&this.min.y<=e.min.y&&e.max.y<=this.max.y&&this.min.z<=e.min.z&&e.max.z<=this.max.z}getParameter(e,t){return t.set((e.x-this.min.x)/(this.max.x-this.min.x),(e.y-this.min.y)/(this.max.y-this.min.y),(e.z-this.min.z)/(this.max.z-this.min.z))}intersectsBox(e){return e.max.x>=this.min.x&&e.min.x<=this.max.x&&e.max.y>=this.min.y&&e.min.y<=this.max.y&&e.max.z>=this.min.z&&e.min.z<=this.max.z}intersectsSphere(e){return this.clampPoint(e.center,un),un.distanceToSquared(e.center)<=e.radius*e.radius}intersectsPlane(e){let t,i;return e.normal.x>0?(t=e.normal.x*this.min.x,i=e.normal.x*this.max.x):(t=e.normal.x*this.max.x,i=e.normal.x*this.min.x),e.normal.y>0?(t+=e.normal.y*this.min.y,i+=e.normal.y*this.max.y):(t+=e.normal.y*this.max.y,i+=e.normal.y*this.min.y),e.normal.z>0?(t+=e.normal.z*this.min.z,i+=e.normal.z*this.max.z):(t+=e.normal.z*this.max.z,i+=e.normal.z*this.min.z),t<=-e.constant&&i>=-e.constant}intersectsTriangle(e){if(this.isEmpty())return!1;this.getCenter(Ss),or.subVectors(this.max,Ss),$i.subVectors(e.a,Ss),Yi.subVectors(e.b,Ss),qi.subVectors(e.c,Ss),Qn.subVectors(Yi,$i),ei.subVectors(qi,Yi),gi.subVectors($i,qi);let t=[0,-Qn.z,Qn.y,0,-ei.z,ei.y,0,-gi.z,gi.y,Qn.z,0,-Qn.x,ei.z,0,-ei.x,gi.z,0,-gi.x,-Qn.y,Qn.x,0,-ei.y,ei.x,0,-gi.y,gi.x,0];return!Va(t,$i,Yi,qi,or)||(t=[1,0,0,0,1,0,0,0,1],!Va(t,$i,Yi,qi,or))?!1:(lr.crossVectors(Qn,ei),t=[lr.x,lr.y,lr.z],Va(t,$i,Yi,qi,or))}clampPoint(e,t){return t.copy(e).clamp(this.min,this.max)}distanceToPoint(e){return this.clampPoint(e,un).distanceTo(e)}getBoundingSphere(e){return this.isEmpty()?e.makeEmpty():(this.getCenter(e.center),e.radius=this.getSize(un).length()*.5),e}intersect(e){return this.min.max(e.min),this.max.min(e.max),this.isEmpty()&&this.makeEmpty(),this}union(e){return this.min.min(e.min),this.max.max(e.max),this}applyMatrix4(e){return this.isEmpty()?this:(Fn[0].set(this.min.x,this.min.y,this.min.z).applyMatrix4(e),Fn[1].set(this.min.x,this.min.y,this.max.z).applyMatrix4(e),Fn[2].set(this.min.x,this.max.y,this.min.z).applyMatrix4(e),Fn[3].set(this.min.x,this.max.y,this.max.z).applyMatrix4(e),Fn[4].set(this.max.x,this.min.y,this.min.z).applyMatrix4(e),Fn[5].set(this.max.x,this.min.y,this.max.z).applyMatrix4(e),Fn[6].set(this.max.x,this.max.y,this.min.z).applyMatrix4(e),Fn[7].set(this.max.x,this.max.y,this.max.z).applyMatrix4(e),this.setFromPoints(Fn),this)}translate(e){return this.min.add(e),this.max.add(e),this}equals(e){return e.min.equals(this.min)&&e.max.equals(this.max)}toJSON(){return{min:this.min.toArray(),max:this.max.toArray()}}fromJSON(e){return this.min.fromArray(e.min),this.max.fromArray(e.max),this}}const Fn=[new N,new N,new N,new N,new N,new N,new N,new N],un=new N,ar=new qs,$i=new N,Yi=new N,qi=new N,Qn=new N,ei=new N,gi=new N,Ss=new N,or=new N,lr=new N,_i=new N;function Va(n,e,t,i,s){for(let r=0,a=n.length-3;r<=a;r+=3){_i.fromArray(n,r);const o=s.x*Math.abs(_i.x)+s.y*Math.abs(_i.y)+s.z*Math.abs(_i.z),c=e.dot(_i),l=t.dot(_i),u=i.dot(_i);if(Math.max(-Math.max(c,l,u),Math.min(c,l,u))>o)return!1}return!0}const Tt=new N,cr=new Oe;let Zd=0;class At extends fi{constructor(e,t,i=!1){if(super(),Array.isArray(e))throw new TypeError("THREE.BufferAttribute: array should be a Typed Array.");this.isBufferAttribute=!0,Object.defineProperty(this,"id",{value:Zd++}),this.name="",this.array=e,this.itemSize=t,this.count=e!==void 0?e.length/t:0,this.normalized=i,this.usage=Ko,this.updateRanges=[],this.gpuType=An,this.version=0}onUploadCallback(){}set needsUpdate(e){e===!0&&this.version++}setUsage(e){return this.usage=e,this}addUpdateRange(e,t){this.updateRanges.push({start:e,count:t})}clearUpdateRanges(){this.updateRanges.length=0}copy(e){return this.name=e.name,this.array=new e.array.constructor(e.array),this.itemSize=e.itemSize,this.count=e.count,this.normalized=e.normalized,this.usage=e.usage,this.gpuType=e.gpuType,this}copyAt(e,t,i){e*=this.itemSize,i*=t.itemSize;for(let s=0,r=this.itemSize;s<r;s++)this.array[e+s]=t.array[i+s];return this}copyArray(e){return this.array.set(e),this}applyMatrix3(e){if(this.itemSize===2)for(let t=0,i=this.count;t<i;t++)cr.fromBufferAttribute(this,t),cr.applyMatrix3(e),this.setXY(t,cr.x,cr.y);else if(this.itemSize===3)for(let t=0,i=this.count;t<i;t++)Tt.fromBufferAttribute(this,t),Tt.applyMatrix3(e),this.setXYZ(t,Tt.x,Tt.y,Tt.z);return this}applyMatrix4(e){for(let t=0,i=this.count;t<i;t++)Tt.fromBufferAttribute(this,t),Tt.applyMatrix4(e),this.setXYZ(t,Tt.x,Tt.y,Tt.z);return this}applyNormalMatrix(e){for(let t=0,i=this.count;t<i;t++)Tt.fromBufferAttribute(this,t),Tt.applyNormalMatrix(e),this.setXYZ(t,Tt.x,Tt.y,Tt.z);return this}transformDirection(e){for(let t=0,i=this.count;t<i;t++)Tt.fromBufferAttribute(this,t),Tt.transformDirection(e),this.setXYZ(t,Tt.x,Tt.y,Tt.z);return this}set(e,t=0){return this.array.set(e,t),this}getComponent(e,t){let i=this.array[e*this.itemSize+t];return this.normalized&&(i=bn(i,this.array)),i}setComponent(e,t,i){return this.normalized&&(i=ct(i,this.array)),this.array[e*this.itemSize+t]=i,this}getX(e){let t=this.array[e*this.itemSize];return this.normalized&&(t=bn(t,this.array)),t}setX(e,t){return this.normalized&&(t=ct(t,this.array)),this.array[e*this.itemSize]=t,this}getY(e){let t=this.array[e*this.itemSize+1];return this.normalized&&(t=bn(t,this.array)),t}setY(e,t){return this.normalized&&(t=ct(t,this.array)),this.array[e*this.itemSize+1]=t,this}getZ(e){let t=this.array[e*this.itemSize+2];return this.normalized&&(t=bn(t,this.array)),t}setZ(e,t){return this.normalized&&(t=ct(t,this.array)),this.array[e*this.itemSize+2]=t,this}getW(e){let t=this.array[e*this.itemSize+3];return this.normalized&&(t=bn(t,this.array)),t}setW(e,t){return this.normalized&&(t=ct(t,this.array)),this.array[e*this.itemSize+3]=t,this}setXY(e,t,i){return e*=this.itemSize,this.normalized&&(t=ct(t,this.array),i=ct(i,this.array)),this.array[e+0]=t,this.array[e+1]=i,this}setXYZ(e,t,i,s){return e*=this.itemSize,this.normalized&&(t=ct(t,this.array),i=ct(i,this.array),s=ct(s,this.array)),this.array[e+0]=t,this.array[e+1]=i,this.array[e+2]=s,this}setXYZW(e,t,i,s,r){return e*=this.itemSize,this.normalized&&(t=ct(t,this.array),i=ct(i,this.array),s=ct(s,this.array),r=ct(r,this.array)),this.array[e+0]=t,this.array[e+1]=i,this.array[e+2]=s,this.array[e+3]=r,this}onUpload(e){return this.onUploadCallback=e,this}clone(){return new this.constructor(this.array,this.itemSize).copy(this)}toJSON(){const e={itemSize:this.itemSize,type:this.array.constructor.name,array:Array.from(this.array),normalized:this.normalized};return this.name!==""&&(e.name=this.name),this.usage!==Ko&&(e.usage=this.usage),e}dispose(){this.dispatchEvent({type:"dispose"})}}class Nh extends At{constructor(e,t,i){super(new Uint16Array(e),t,i)}}class Fh extends At{constructor(e,t,i){super(new Uint32Array(e),t,i)}}class mn extends At{constructor(e,t,i){super(new Float32Array(e),t,i)}}const Jd=new qs,ys=new N,Wa=new N;class js{constructor(e=new N,t=-1){this.isSphere=!0,this.center=e,this.radius=t}set(e,t){return this.center.copy(e),this.radius=t,this}setFromPoints(e,t){const i=this.center;t!==void 0?i.copy(t):Jd.setFromPoints(e).getCenter(i);let s=0;for(let r=0,a=e.length;r<a;r++)s=Math.max(s,i.distanceToSquared(e[r]));return this.radius=Math.sqrt(s),this}copy(e){return this.center.copy(e.center),this.radius=e.radius,this}isEmpty(){return this.radius<0}makeEmpty(){return this.center.set(0,0,0),this.radius=-1,this}containsPoint(e){return e.distanceToSquared(this.center)<=this.radius*this.radius}distanceToPoint(e){return e.distanceTo(this.center)-this.radius}intersectsSphere(e){const t=this.radius+e.radius;return e.center.distanceToSquared(this.center)<=t*t}intersectsBox(e){return e.intersectsSphere(this)}intersectsPlane(e){return Math.abs(e.distanceToPoint(this.center))<=this.radius}clampPoint(e,t){const i=this.center.distanceToSquared(e);return t.copy(e),i>this.radius*this.radius&&(t.sub(this.center).normalize(),t.multiplyScalar(this.radius).add(this.center)),t}getBoundingBox(e){return this.isEmpty()?(e.makeEmpty(),e):(e.set(this.center,this.center),e.expandByScalar(this.radius),e)}applyMatrix4(e){return this.center.applyMatrix4(e),this.radius=this.radius*e.getMaxScaleOnAxis(),this}translate(e){return this.center.add(e),this}expandByPoint(e){if(this.isEmpty())return this.center.copy(e),this.radius=0,this;ys.subVectors(e,this.center);const t=ys.lengthSq();if(t>this.radius*this.radius){const i=Math.sqrt(t),s=(i-this.radius)*.5;this.center.addScaledVector(ys,s/i),this.radius+=s}return this}union(e){return e.isEmpty()?this:this.isEmpty()?(this.copy(e),this):(this.center.equals(e.center)===!0?this.radius=Math.max(this.radius,e.radius):(Wa.subVectors(e.center,this.center).setLength(e.radius),this.expandByPoint(ys.copy(e.center).add(Wa)),this.expandByPoint(ys.copy(e.center).sub(Wa))),this)}equals(e){return e.center.equals(this.center)&&e.radius===this.radius}clone(){return new this.constructor().copy(this)}toJSON(){return{radius:this.radius,center:this.center.toArray()}}fromJSON(e){return this.radius=e.radius,this.center.fromArray(e.center),this}}let Qd=0;const nn=new gt,Xa=new kt,ji=new N,Qt=new qs,Es=new qs,Pt=new N;class Ut extends fi{constructor(){super(),this.isBufferGeometry=!0,Object.defineProperty(this,"id",{value:Qd++}),this.uuid=oi(),this.name="",this.type="BufferGeometry",this.index=null,this.indirect=null,this.indirectOffset=0,this.attributes={},this.morphAttributes={},this.morphTargetsRelative=!1,this.groups=[],this.boundingBox=null,this.boundingSphere=null,this.drawRange={start:0,count:1/0},this.userData={}}getIndex(){return this.index}setIndex(e){return Array.isArray(e)?this.index=new(Ld(e)?Fh:Nh)(e,1):this.index=e,this}setIndirect(e,t=0){return this.indirect=e,this.indirectOffset=t,this}getIndirect(){return this.indirect}getAttribute(e){return this.attributes[e]}setAttribute(e,t){return this.attributes[e]=t,this}deleteAttribute(e){return delete this.attributes[e],this}hasAttribute(e){return this.attributes[e]!==void 0}addGroup(e,t,i=0){this.groups.push({start:e,count:t,materialIndex:i})}clearGroups(){this.groups=[]}setDrawRange(e,t){this.drawRange.start=e,this.drawRange.count=t}applyMatrix4(e){const t=this.attributes.position;t!==void 0&&(t.applyMatrix4(e),t.needsUpdate=!0);const i=this.attributes.normal;if(i!==void 0){const r=new He().getNormalMatrix(e);i.applyNormalMatrix(r),i.needsUpdate=!0}const s=this.attributes.tangent;return s!==void 0&&(s.transformDirection(e),s.needsUpdate=!0),this.boundingBox!==null&&this.computeBoundingBox(),this.boundingSphere!==null&&this.computeBoundingSphere(),this}applyQuaternion(e){return nn.makeRotationFromQuaternion(e),this.applyMatrix4(nn),this}rotateX(e){return nn.makeRotationX(e),this.applyMatrix4(nn),this}rotateY(e){return nn.makeRotationY(e),this.applyMatrix4(nn),this}rotateZ(e){return nn.makeRotationZ(e),this.applyMatrix4(nn),this}translate(e,t,i){return nn.makeTranslation(e,t,i),this.applyMatrix4(nn),this}scale(e,t,i){return nn.makeScale(e,t,i),this.applyMatrix4(nn),this}lookAt(e){return Xa.lookAt(e),Xa.updateMatrix(),this.applyMatrix4(Xa.matrix),this}center(){return this.computeBoundingBox(),this.boundingBox.getCenter(ji).negate(),this.translate(ji.x,ji.y,ji.z),this}setFromPoints(e){const t=this.getAttribute("position");if(t===void 0){const i=[];for(let s=0,r=e.length;s<r;s++){const a=e[s];i.push(a.x,a.y,a.z||0)}this.setAttribute("position",new mn(i,3))}else{const i=Math.min(e.length,t.count);for(let s=0;s<i;s++){const r=e[s];t.setXYZ(s,r.x,r.y,r.z||0)}e.length>t.count&&Ne("BufferGeometry: Buffer size too small for points data. Use .dispose() and create a new geometry."),t.needsUpdate=!0}return this}computeBoundingBox(){this.boundingBox===null&&(this.boundingBox=new qs);const e=this.attributes.position,t=this.morphAttributes.position;if(e&&e.isGLBufferAttribute){Qe("BufferGeometry.computeBoundingBox(): GLBufferAttribute requires a manual bounding box.",this),this.boundingBox.set(new N(-1/0,-1/0,-1/0),new N(1/0,1/0,1/0));return}if(e!==void 0){if(this.boundingBox.setFromBufferAttribute(e),t)for(let i=0,s=t.length;i<s;i++){const r=t[i];Qt.setFromBufferAttribute(r),this.morphTargetsRelative?(Pt.addVectors(this.boundingBox.min,Qt.min),this.boundingBox.expandByPoint(Pt),Pt.addVectors(this.boundingBox.max,Qt.max),this.boundingBox.expandByPoint(Pt)):(this.boundingBox.expandByPoint(Qt.min),this.boundingBox.expandByPoint(Qt.max))}}else this.boundingBox.makeEmpty();(isNaN(this.boundingBox.min.x)||isNaN(this.boundingBox.min.y)||isNaN(this.boundingBox.min.z))&&Qe('BufferGeometry.computeBoundingBox(): Computed min/max have NaN values. The "position" attribute is likely to have NaN values.',this)}computeBoundingSphere(){this.boundingSphere===null&&(this.boundingSphere=new js);const e=this.attributes.position,t=this.morphAttributes.position;if(e&&e.isGLBufferAttribute){Qe("BufferGeometry.computeBoundingSphere(): GLBufferAttribute requires a manual bounding sphere.",this),this.boundingSphere.set(new N,1/0);return}if(e){const i=this.boundingSphere.center;if(Qt.setFromBufferAttribute(e),t)for(let r=0,a=t.length;r<a;r++){const o=t[r];Es.setFromBufferAttribute(o),this.morphTargetsRelative?(Pt.addVectors(Qt.min,Es.min),Qt.expandByPoint(Pt),Pt.addVectors(Qt.max,Es.max),Qt.expandByPoint(Pt)):(Qt.expandByPoint(Es.min),Qt.expandByPoint(Es.max))}Qt.getCenter(i);let s=0;for(let r=0,a=e.count;r<a;r++)Pt.fromBufferAttribute(e,r),s=Math.max(s,i.distanceToSquared(Pt));if(t)for(let r=0,a=t.length;r<a;r++){const o=t[r],c=this.morphTargetsRelative;for(let l=0,u=o.count;l<u;l++)Pt.fromBufferAttribute(o,l),c&&(ji.fromBufferAttribute(e,l),Pt.add(ji)),s=Math.max(s,i.distanceToSquared(Pt))}this.boundingSphere.radius=Math.sqrt(s),isNaN(this.boundingSphere.radius)&&Qe('BufferGeometry.computeBoundingSphere(): Computed radius is NaN. The "position" attribute is likely to have NaN values.',this)}}computeTangents(){const e=this.index,t=this.attributes;if(e===null||t.position===void 0||t.normal===void 0||t.uv===void 0){Qe("BufferGeometry: .computeTangents() failed. Missing required attributes (index, position, normal or uv)");return}const i=t.position,s=t.normal,r=t.uv;this.hasAttribute("tangent")===!1&&this.setAttribute("tangent",new At(new Float32Array(4*i.count),4));const a=this.getAttribute("tangent"),o=[],c=[];for(let M=0;M<i.count;M++)o[M]=new N,c[M]=new N;const l=new N,u=new N,g=new N,h=new Oe,m=new Oe,_=new Oe,S=new N,p=new N;function d(M,w,B){l.fromBufferAttribute(i,M),u.fromBufferAttribute(i,w),g.fromBufferAttribute(i,B),h.fromBufferAttribute(r,M),m.fromBufferAttribute(r,w),_.fromBufferAttribute(r,B),u.sub(l),g.sub(l),m.sub(h),_.sub(h);const C=1/(m.x*_.y-_.x*m.y);isFinite(C)&&(S.copy(u).multiplyScalar(_.y).addScaledVector(g,-m.y).multiplyScalar(C),p.copy(g).multiplyScalar(m.x).addScaledVector(u,-_.x).multiplyScalar(C),o[M].add(S),o[w].add(S),o[B].add(S),c[M].add(p),c[w].add(p),c[B].add(p))}let y=this.groups;y.length===0&&(y=[{start:0,count:e.count}]);for(let M=0,w=y.length;M<w;++M){const B=y[M],C=B.start,z=B.count;for(let Z=C,J=C+z;Z<J;Z+=3)d(e.getX(Z+0),e.getX(Z+1),e.getX(Z+2))}const A=new N,T=new N,L=new N,b=new N;function I(M){L.fromBufferAttribute(s,M),b.copy(L);const w=o[M];A.copy(w),A.sub(L.multiplyScalar(L.dot(w))).normalize(),T.crossVectors(b,w);const C=T.dot(c[M])<0?-1:1;a.setXYZW(M,A.x,A.y,A.z,C)}for(let M=0,w=y.length;M<w;++M){const B=y[M],C=B.start,z=B.count;for(let Z=C,J=C+z;Z<J;Z+=3)I(e.getX(Z+0)),I(e.getX(Z+1)),I(e.getX(Z+2))}}computeVertexNormals(){const e=this.index,t=this.getAttribute("position");if(t!==void 0){let i=this.getAttribute("normal");if(i===void 0)i=new At(new Float32Array(t.count*3),3),this.setAttribute("normal",i);else for(let h=0,m=i.count;h<m;h++)i.setXYZ(h,0,0,0);const s=new N,r=new N,a=new N,o=new N,c=new N,l=new N,u=new N,g=new N;if(e)for(let h=0,m=e.count;h<m;h+=3){const _=e.getX(h+0),S=e.getX(h+1),p=e.getX(h+2);s.fromBufferAttribute(t,_),r.fromBufferAttribute(t,S),a.fromBufferAttribute(t,p),u.subVectors(a,r),g.subVectors(s,r),u.cross(g),o.fromBufferAttribute(i,_),c.fromBufferAttribute(i,S),l.fromBufferAttribute(i,p),o.add(u),c.add(u),l.add(u),i.setXYZ(_,o.x,o.y,o.z),i.setXYZ(S,c.x,c.y,c.z),i.setXYZ(p,l.x,l.y,l.z)}else for(let h=0,m=t.count;h<m;h+=3)s.fromBufferAttribute(t,h+0),r.fromBufferAttribute(t,h+1),a.fromBufferAttribute(t,h+2),u.subVectors(a,r),g.subVectors(s,r),u.cross(g),i.setXYZ(h+0,u.x,u.y,u.z),i.setXYZ(h+1,u.x,u.y,u.z),i.setXYZ(h+2,u.x,u.y,u.z);this.normalizeNormals(),i.needsUpdate=!0}}normalizeNormals(){const e=this.attributes.normal;for(let t=0,i=e.count;t<i;t++)Pt.fromBufferAttribute(e,t),Pt.normalize(),e.setXYZ(t,Pt.x,Pt.y,Pt.z)}toNonIndexed(){function e(o,c){const l=o.array,u=o.itemSize,g=o.normalized,h=new l.constructor(c.length*u);let m=0,_=0;for(let S=0,p=c.length;S<p;S++){o.isInterleavedBufferAttribute?m=c[S]*o.data.stride+o.offset:m=c[S]*u;for(let d=0;d<u;d++)h[_++]=l[m++]}return new At(h,u,g)}if(this.index===null)return Ne("BufferGeometry.toNonIndexed(): BufferGeometry is already non-indexed."),this;const t=new Ut,i=this.index.array,s=this.attributes;for(const o in s){const c=s[o],l=e(c,i);t.setAttribute(o,l)}const r=this.morphAttributes;for(const o in r){const c=[],l=r[o];for(let u=0,g=l.length;u<g;u++){const h=l[u],m=e(h,i);c.push(m)}t.morphAttributes[o]=c}t.morphTargetsRelative=this.morphTargetsRelative;const a=this.groups;for(let o=0,c=a.length;o<c;o++){const l=a[o];t.addGroup(l.start,l.count,l.materialIndex)}return t}toJSON(){const e={metadata:{version:4.7,type:"BufferGeometry",generator:"BufferGeometry.toJSON"}};if(e.uuid=this.uuid,e.type=this.type,this.name!==""&&(e.name=this.name),Object.keys(this.userData).length>0&&(e.userData=this.userData),this.parameters!==void 0){const c=this.parameters;for(const l in c)c[l]!==void 0&&(e[l]=c[l]);return e}e.data={attributes:{}};const t=this.index;t!==null&&(e.data.index={type:t.array.constructor.name,array:Array.prototype.slice.call(t.array)});const i=this.attributes;for(const c in i){const l=i[c];e.data.attributes[c]=l.toJSON(e.data)}const s={};let r=!1;for(const c in this.morphAttributes){const l=this.morphAttributes[c],u=[];for(let g=0,h=l.length;g<h;g++){const m=l[g];u.push(m.toJSON(e.data))}u.length>0&&(s[c]=u,r=!0)}r&&(e.data.morphAttributes=s,e.data.morphTargetsRelative=this.morphTargetsRelative);const a=this.groups;a.length>0&&(e.data.groups=JSON.parse(JSON.stringify(a)));const o=this.boundingSphere;return o!==null&&(e.data.boundingSphere=o.toJSON()),e}clone(){return new this.constructor().copy(this)}copy(e){this.index=null,this.attributes={},this.morphAttributes={},this.groups=[],this.boundingBox=null,this.boundingSphere=null;const t={};this.name=e.name;const i=e.index;i!==null&&this.setIndex(i.clone());const s=e.attributes;for(const l in s){const u=s[l];this.setAttribute(l,u.clone(t))}const r=e.morphAttributes;for(const l in r){const u=[],g=r[l];for(let h=0,m=g.length;h<m;h++)u.push(g[h].clone(t));this.morphAttributes[l]=u}this.morphTargetsRelative=e.morphTargetsRelative;const a=e.groups;for(let l=0,u=a.length;l<u;l++){const g=a[l];this.addGroup(g.start,g.count,g.materialIndex)}const o=e.boundingBox;o!==null&&(this.boundingBox=o.clone());const c=e.boundingSphere;return c!==null&&(this.boundingSphere=c.clone()),this.drawRange.start=e.drawRange.start,this.drawRange.count=e.drawRange.count,this.userData=e.userData,this}dispose(){this.dispatchEvent({type:"dispose"})}}class ef{constructor(e,t){this.isInterleavedBuffer=!0,this.array=e,this.stride=t,this.count=e!==void 0?e.length/t:0,this.usage=Ko,this.updateRanges=[],this.version=0,this.uuid=oi()}onUploadCallback(){}set needsUpdate(e){e===!0&&this.version++}setUsage(e){return this.usage=e,this}addUpdateRange(e,t){this.updateRanges.push({start:e,count:t})}clearUpdateRanges(){this.updateRanges.length=0}copy(e){return this.array=new e.array.constructor(e.array),this.count=e.count,this.stride=e.stride,this.usage=e.usage,this}copyAt(e,t,i){e*=this.stride,i*=t.stride;for(let s=0,r=this.stride;s<r;s++)this.array[e+s]=t.array[i+s];return this}set(e,t=0){return this.array.set(e,t),this}clone(e){e.arrayBuffers===void 0&&(e.arrayBuffers={}),this.array.buffer._uuid===void 0&&(this.array.buffer._uuid=oi()),e.arrayBuffers[this.array.buffer._uuid]===void 0&&(e.arrayBuffers[this.array.buffer._uuid]=this.array.slice(0).buffer);const t=new this.array.constructor(e.arrayBuffers[this.array.buffer._uuid]),i=new this.constructor(t,this.stride);return i.setUsage(this.usage),i}onUpload(e){return this.onUploadCallback=e,this}toJSON(e){return e.arrayBuffers===void 0&&(e.arrayBuffers={}),this.array.buffer._uuid===void 0&&(this.array.buffer._uuid=oi()),e.arrayBuffers[this.array.buffer._uuid]===void 0&&(e.arrayBuffers[this.array.buffer._uuid]=Array.from(new Uint32Array(this.array.buffer))),{uuid:this.uuid,buffer:this.array.buffer._uuid,type:this.array.constructor.name,stride:this.stride}}}const Ht=new N;class la{constructor(e,t,i,s=!1){this.isInterleavedBufferAttribute=!0,this.name="",this.data=e,this.itemSize=t,this.offset=i,this.normalized=s}get count(){return this.data.count}get array(){return this.data.array}set needsUpdate(e){this.data.needsUpdate=e}applyMatrix4(e){for(let t=0,i=this.data.count;t<i;t++)Ht.fromBufferAttribute(this,t),Ht.applyMatrix4(e),this.setXYZ(t,Ht.x,Ht.y,Ht.z);return this}applyNormalMatrix(e){for(let t=0,i=this.count;t<i;t++)Ht.fromBufferAttribute(this,t),Ht.applyNormalMatrix(e),this.setXYZ(t,Ht.x,Ht.y,Ht.z);return this}transformDirection(e){for(let t=0,i=this.count;t<i;t++)Ht.fromBufferAttribute(this,t),Ht.transformDirection(e),this.setXYZ(t,Ht.x,Ht.y,Ht.z);return this}getComponent(e,t){let i=this.array[e*this.data.stride+this.offset+t];return this.normalized&&(i=bn(i,this.array)),i}setComponent(e,t,i){return this.normalized&&(i=ct(i,this.array)),this.data.array[e*this.data.stride+this.offset+t]=i,this}setX(e,t){return this.normalized&&(t=ct(t,this.array)),this.data.array[e*this.data.stride+this.offset]=t,this}setY(e,t){return this.normalized&&(t=ct(t,this.array)),this.data.array[e*this.data.stride+this.offset+1]=t,this}setZ(e,t){return this.normalized&&(t=ct(t,this.array)),this.data.array[e*this.data.stride+this.offset+2]=t,this}setW(e,t){return this.normalized&&(t=ct(t,this.array)),this.data.array[e*this.data.stride+this.offset+3]=t,this}getX(e){let t=this.data.array[e*this.data.stride+this.offset];return this.normalized&&(t=bn(t,this.array)),t}getY(e){let t=this.data.array[e*this.data.stride+this.offset+1];return this.normalized&&(t=bn(t,this.array)),t}getZ(e){let t=this.data.array[e*this.data.stride+this.offset+2];return this.normalized&&(t=bn(t,this.array)),t}getW(e){let t=this.data.array[e*this.data.stride+this.offset+3];return this.normalized&&(t=bn(t,this.array)),t}setXY(e,t,i){return e=e*this.data.stride+this.offset,this.normalized&&(t=ct(t,this.array),i=ct(i,this.array)),this.data.array[e+0]=t,this.data.array[e+1]=i,this}setXYZ(e,t,i,s){return e=e*this.data.stride+this.offset,this.normalized&&(t=ct(t,this.array),i=ct(i,this.array),s=ct(s,this.array)),this.data.array[e+0]=t,this.data.array[e+1]=i,this.data.array[e+2]=s,this}setXYZW(e,t,i,s,r){return e=e*this.data.stride+this.offset,this.normalized&&(t=ct(t,this.array),i=ct(i,this.array),s=ct(s,this.array),r=ct(r,this.array)),this.data.array[e+0]=t,this.data.array[e+1]=i,this.data.array[e+2]=s,this.data.array[e+3]=r,this}clone(e){if(e===void 0){oa("InterleavedBufferAttribute.clone(): Cloning an interleaved buffer attribute will de-interleave buffer data.");const t=[];for(let i=0;i<this.count;i++){const s=i*this.data.stride+this.offset;for(let r=0;r<this.itemSize;r++)t.push(this.data.array[s+r])}return new At(new this.array.constructor(t),this.itemSize,this.normalized)}else return e.interleavedBuffers===void 0&&(e.interleavedBuffers={}),e.interleavedBuffers[this.data.uuid]===void 0&&(e.interleavedBuffers[this.data.uuid]=this.data.clone(e)),new la(e.interleavedBuffers[this.data.uuid],this.itemSize,this.offset,this.normalized)}toJSON(e){if(e===void 0){oa("InterleavedBufferAttribute.toJSON(): Serializing an interleaved buffer attribute will de-interleave buffer data.");const t=[];for(let i=0;i<this.count;i++){const s=i*this.data.stride+this.offset;for(let r=0;r<this.itemSize;r++)t.push(this.data.array[s+r])}return{itemSize:this.itemSize,type:this.array.constructor.name,array:t,normalized:this.normalized}}else return e.interleavedBuffers===void 0&&(e.interleavedBuffers={}),e.interleavedBuffers[this.data.uuid]===void 0&&(e.interleavedBuffers[this.data.uuid]=this.data.toJSON(e)),{isInterleavedBufferAttribute:!0,itemSize:this.itemSize,data:this.data.uuid,offset:this.offset,normalized:this.normalized}}}let tf=0;class Di extends fi{constructor(){super(),this.isMaterial=!0,Object.defineProperty(this,"id",{value:tf++}),this.uuid=oi(),this.name="",this.type="Material",this.blending=hs,this.side=ci,this.vertexColors=!1,this.opacity=1,this.transparent=!1,this.alphaHash=!1,this.blendSrc=lo,this.blendDst=co,this.blendEquation=Si,this.blendSrcAlpha=null,this.blendDstAlpha=null,this.blendEquationAlpha=null,this.blendColor=new ke(0,0,0),this.blendAlpha=0,this.depthFunc=ps,this.depthTest=!0,this.depthWrite=!0,this.stencilWriteMask=255,this.stencilFunc=sc,this.stencilRef=0,this.stencilFuncMask=255,this.stencilFail=zi,this.stencilZFail=zi,this.stencilZPass=zi,this.stencilWrite=!1,this.clippingPlanes=null,this.clipIntersection=!1,this.clipShadows=!1,this.shadowSide=null,this.colorWrite=!0,this.precision=null,this.polygonOffset=!1,this.polygonOffsetFactor=0,this.polygonOffsetUnits=0,this.dithering=!1,this.alphaToCoverage=!1,this.premultipliedAlpha=!1,this.forceSinglePass=!1,this.allowOverride=!0,this.visible=!0,this.toneMapped=!0,this.userData={},this.version=0,this._alphaTest=0}get alphaTest(){return this._alphaTest}set alphaTest(e){this._alphaTest>0!=e>0&&this.version++,this._alphaTest=e}onBeforeRender(){}onBeforeCompile(){}customProgramCacheKey(){return this.onBeforeCompile.toString()}setValues(e){if(e!==void 0)for(const t in e){const i=e[t];if(i===void 0){Ne(`Material: parameter '${t}' has value of undefined.`);continue}const s=this[t];if(s===void 0){Ne(`Material: '${t}' is not a property of THREE.${this.type}.`);continue}s&&s.isColor?s.set(i):s&&s.isVector3&&i&&i.isVector3?s.copy(i):this[t]=i}}toJSON(e){const t=e===void 0||typeof e=="string";t&&(e={textures:{},images:{}});const i={metadata:{version:4.7,type:"Material",generator:"Material.toJSON"}};i.uuid=this.uuid,i.type=this.type,this.name!==""&&(i.name=this.name),this.color&&this.color.isColor&&(i.color=this.color.getHex()),this.roughness!==void 0&&(i.roughness=this.roughness),this.metalness!==void 0&&(i.metalness=this.metalness),this.sheen!==void 0&&(i.sheen=this.sheen),this.sheenColor&&this.sheenColor.isColor&&(i.sheenColor=this.sheenColor.getHex()),this.sheenRoughness!==void 0&&(i.sheenRoughness=this.sheenRoughness),this.emissive&&this.emissive.isColor&&(i.emissive=this.emissive.getHex()),this.emissiveIntensity!==void 0&&this.emissiveIntensity!==1&&(i.emissiveIntensity=this.emissiveIntensity),this.specular&&this.specular.isColor&&(i.specular=this.specular.getHex()),this.specularIntensity!==void 0&&(i.specularIntensity=this.specularIntensity),this.specularColor&&this.specularColor.isColor&&(i.specularColor=this.specularColor.getHex()),this.shininess!==void 0&&(i.shininess=this.shininess),this.clearcoat!==void 0&&(i.clearcoat=this.clearcoat),this.clearcoatRoughness!==void 0&&(i.clearcoatRoughness=this.clearcoatRoughness),this.clearcoatMap&&this.clearcoatMap.isTexture&&(i.clearcoatMap=this.clearcoatMap.toJSON(e).uuid),this.clearcoatRoughnessMap&&this.clearcoatRoughnessMap.isTexture&&(i.clearcoatRoughnessMap=this.clearcoatRoughnessMap.toJSON(e).uuid),this.clearcoatNormalMap&&this.clearcoatNormalMap.isTexture&&(i.clearcoatNormalMap=this.clearcoatNormalMap.toJSON(e).uuid,i.clearcoatNormalScale=this.clearcoatNormalScale.toArray()),this.sheenColorMap&&this.sheenColorMap.isTexture&&(i.sheenColorMap=this.sheenColorMap.toJSON(e).uuid),this.sheenRoughnessMap&&this.sheenRoughnessMap.isTexture&&(i.sheenRoughnessMap=this.sheenRoughnessMap.toJSON(e).uuid),this.dispersion!==void 0&&(i.dispersion=this.dispersion),this.iridescence!==void 0&&(i.iridescence=this.iridescence),this.iridescenceIOR!==void 0&&(i.iridescenceIOR=this.iridescenceIOR),this.iridescenceThicknessRange!==void 0&&(i.iridescenceThicknessRange=this.iridescenceThicknessRange),this.iridescenceMap&&this.iridescenceMap.isTexture&&(i.iridescenceMap=this.iridescenceMap.toJSON(e).uuid),this.iridescenceThicknessMap&&this.iridescenceThicknessMap.isTexture&&(i.iridescenceThicknessMap=this.iridescenceThicknessMap.toJSON(e).uuid),this.anisotropy!==void 0&&(i.anisotropy=this.anisotropy),this.anisotropyRotation!==void 0&&(i.anisotropyRotation=this.anisotropyRotation),this.anisotropyMap&&this.anisotropyMap.isTexture&&(i.anisotropyMap=this.anisotropyMap.toJSON(e).uuid),this.map&&this.map.isTexture&&(i.map=this.map.toJSON(e).uuid),this.matcap&&this.matcap.isTexture&&(i.matcap=this.matcap.toJSON(e).uuid),this.alphaMap&&this.alphaMap.isTexture&&(i.alphaMap=this.alphaMap.toJSON(e).uuid),this.lightMap&&this.lightMap.isTexture&&(i.lightMap=this.lightMap.toJSON(e).uuid,i.lightMapIntensity=this.lightMapIntensity),this.aoMap&&this.aoMap.isTexture&&(i.aoMap=this.aoMap.toJSON(e).uuid,i.aoMapIntensity=this.aoMapIntensity),this.bumpMap&&this.bumpMap.isTexture&&(i.bumpMap=this.bumpMap.toJSON(e).uuid,i.bumpScale=this.bumpScale),this.normalMap&&this.normalMap.isTexture&&(i.normalMap=this.normalMap.toJSON(e).uuid,i.normalMapType=this.normalMapType,i.normalScale=this.normalScale.toArray()),this.displacementMap&&this.displacementMap.isTexture&&(i.displacementMap=this.displacementMap.toJSON(e).uuid,i.displacementScale=this.displacementScale,i.displacementBias=this.displacementBias),this.roughnessMap&&this.roughnessMap.isTexture&&(i.roughnessMap=this.roughnessMap.toJSON(e).uuid),this.metalnessMap&&this.metalnessMap.isTexture&&(i.metalnessMap=this.metalnessMap.toJSON(e).uuid),this.emissiveMap&&this.emissiveMap.isTexture&&(i.emissiveMap=this.emissiveMap.toJSON(e).uuid),this.specularMap&&this.specularMap.isTexture&&(i.specularMap=this.specularMap.toJSON(e).uuid),this.specularIntensityMap&&this.specularIntensityMap.isTexture&&(i.specularIntensityMap=this.specularIntensityMap.toJSON(e).uuid),this.specularColorMap&&this.specularColorMap.isTexture&&(i.specularColorMap=this.specularColorMap.toJSON(e).uuid),this.envMap&&this.envMap.isTexture&&(i.envMap=this.envMap.toJSON(e).uuid,this.combine!==void 0&&(i.combine=this.combine)),this.envMapRotation!==void 0&&(i.envMapRotation=this.envMapRotation.toArray()),this.envMapIntensity!==void 0&&(i.envMapIntensity=this.envMapIntensity),this.reflectivity!==void 0&&(i.reflectivity=this.reflectivity),this.refractionRatio!==void 0&&(i.refractionRatio=this.refractionRatio),this.gradientMap&&this.gradientMap.isTexture&&(i.gradientMap=this.gradientMap.toJSON(e).uuid),this.transmission!==void 0&&(i.transmission=this.transmission),this.transmissionMap&&this.transmissionMap.isTexture&&(i.transmissionMap=this.transmissionMap.toJSON(e).uuid),this.thickness!==void 0&&(i.thickness=this.thickness),this.thicknessMap&&this.thicknessMap.isTexture&&(i.thicknessMap=this.thicknessMap.toJSON(e).uuid),this.attenuationDistance!==void 0&&this.attenuationDistance!==1/0&&(i.attenuationDistance=this.attenuationDistance),this.attenuationColor!==void 0&&(i.attenuationColor=this.attenuationColor.getHex()),this.size!==void 0&&(i.size=this.size),this.shadowSide!==null&&(i.shadowSide=this.shadowSide),this.sizeAttenuation!==void 0&&(i.sizeAttenuation=this.sizeAttenuation),this.blending!==hs&&(i.blending=this.blending),this.side!==ci&&(i.side=this.side),this.vertexColors===!0&&(i.vertexColors=!0),this.opacity<1&&(i.opacity=this.opacity),this.transparent===!0&&(i.transparent=!0),this.blendSrc!==lo&&(i.blendSrc=this.blendSrc),this.blendDst!==co&&(i.blendDst=this.blendDst),this.blendEquation!==Si&&(i.blendEquation=this.blendEquation),this.blendSrcAlpha!==null&&(i.blendSrcAlpha=this.blendSrcAlpha),this.blendDstAlpha!==null&&(i.blendDstAlpha=this.blendDstAlpha),this.blendEquationAlpha!==null&&(i.blendEquationAlpha=this.blendEquationAlpha),this.blendColor&&this.blendColor.isColor&&(i.blendColor=this.blendColor.getHex()),this.blendAlpha!==0&&(i.blendAlpha=this.blendAlpha),this.depthFunc!==ps&&(i.depthFunc=this.depthFunc),this.depthTest===!1&&(i.depthTest=this.depthTest),this.depthWrite===!1&&(i.depthWrite=this.depthWrite),this.colorWrite===!1&&(i.colorWrite=this.colorWrite),this.stencilWriteMask!==255&&(i.stencilWriteMask=this.stencilWriteMask),this.stencilFunc!==sc&&(i.stencilFunc=this.stencilFunc),this.stencilRef!==0&&(i.stencilRef=this.stencilRef),this.stencilFuncMask!==255&&(i.stencilFuncMask=this.stencilFuncMask),this.stencilFail!==zi&&(i.stencilFail=this.stencilFail),this.stencilZFail!==zi&&(i.stencilZFail=this.stencilZFail),this.stencilZPass!==zi&&(i.stencilZPass=this.stencilZPass),this.stencilWrite===!0&&(i.stencilWrite=this.stencilWrite),this.rotation!==void 0&&this.rotation!==0&&(i.rotation=this.rotation),this.polygonOffset===!0&&(i.polygonOffset=!0),this.polygonOffsetFactor!==0&&(i.polygonOffsetFactor=this.polygonOffsetFactor),this.polygonOffsetUnits!==0&&(i.polygonOffsetUnits=this.polygonOffsetUnits),this.linewidth!==void 0&&this.linewidth!==1&&(i.linewidth=this.linewidth),this.dashSize!==void 0&&(i.dashSize=this.dashSize),this.gapSize!==void 0&&(i.gapSize=this.gapSize),this.scale!==void 0&&(i.scale=this.scale),this.dithering===!0&&(i.dithering=!0),this.alphaTest>0&&(i.alphaTest=this.alphaTest),this.alphaHash===!0&&(i.alphaHash=!0),this.alphaToCoverage===!0&&(i.alphaToCoverage=!0),this.premultipliedAlpha===!0&&(i.premultipliedAlpha=!0),this.forceSinglePass===!0&&(i.forceSinglePass=!0),this.allowOverride===!1&&(i.allowOverride=!1),this.wireframe===!0&&(i.wireframe=!0),this.wireframeLinewidth>1&&(i.wireframeLinewidth=this.wireframeLinewidth),this.wireframeLinecap!=="round"&&(i.wireframeLinecap=this.wireframeLinecap),this.wireframeLinejoin!=="round"&&(i.wireframeLinejoin=this.wireframeLinejoin),this.flatShading===!0&&(i.flatShading=!0),this.visible===!1&&(i.visible=!1),this.toneMapped===!1&&(i.toneMapped=!1),this.fog===!1&&(i.fog=!1),Object.keys(this.userData).length>0&&(i.userData=this.userData);function s(r){const a=[];for(const o in r){const c=r[o];delete c.metadata,a.push(c)}return a}if(t){const r=s(e.textures),a=s(e.images);r.length>0&&(i.textures=r),a.length>0&&(i.images=a)}return i}clone(){return new this.constructor().copy(this)}copy(e){this.name=e.name,this.blending=e.blending,this.side=e.side,this.vertexColors=e.vertexColors,this.opacity=e.opacity,this.transparent=e.transparent,this.blendSrc=e.blendSrc,this.blendDst=e.blendDst,this.blendEquation=e.blendEquation,this.blendSrcAlpha=e.blendSrcAlpha,this.blendDstAlpha=e.blendDstAlpha,this.blendEquationAlpha=e.blendEquationAlpha,this.blendColor.copy(e.blendColor),this.blendAlpha=e.blendAlpha,this.depthFunc=e.depthFunc,this.depthTest=e.depthTest,this.depthWrite=e.depthWrite,this.stencilWriteMask=e.stencilWriteMask,this.stencilFunc=e.stencilFunc,this.stencilRef=e.stencilRef,this.stencilFuncMask=e.stencilFuncMask,this.stencilFail=e.stencilFail,this.stencilZFail=e.stencilZFail,this.stencilZPass=e.stencilZPass,this.stencilWrite=e.stencilWrite;const t=e.clippingPlanes;let i=null;if(t!==null){const s=t.length;i=new Array(s);for(let r=0;r!==s;++r)i[r]=t[r].clone()}return this.clippingPlanes=i,this.clipIntersection=e.clipIntersection,this.clipShadows=e.clipShadows,this.shadowSide=e.shadowSide,this.colorWrite=e.colorWrite,this.precision=e.precision,this.polygonOffset=e.polygonOffset,this.polygonOffsetFactor=e.polygonOffsetFactor,this.polygonOffsetUnits=e.polygonOffsetUnits,this.dithering=e.dithering,this.alphaTest=e.alphaTest,this.alphaHash=e.alphaHash,this.alphaToCoverage=e.alphaToCoverage,this.premultipliedAlpha=e.premultipliedAlpha,this.forceSinglePass=e.forceSinglePass,this.allowOverride=e.allowOverride,this.visible=e.visible,this.toneMapped=e.toneMapped,this.userData=JSON.parse(JSON.stringify(e.userData)),this}dispose(){this.dispatchEvent({type:"dispose"})}set needsUpdate(e){e===!0&&this.version++}}class Oh extends Di{constructor(e){super(),this.isSpriteMaterial=!0,this.type="SpriteMaterial",this.color=new ke(16777215),this.map=null,this.alphaMap=null,this.rotation=0,this.sizeAttenuation=!0,this.transparent=!0,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.alphaMap=e.alphaMap,this.rotation=e.rotation,this.sizeAttenuation=e.sizeAttenuation,this.fog=e.fog,this}}let Ki;const bs=new N,Zi=new N,Ji=new N,Qi=new Oe,Ts=new Oe,Bh=new gt,hr=new N,As=new N,ur=new N,xc=new Oe,$a=new Oe,vc=new Oe;class nf extends kt{constructor(e=new Oh){if(super(),this.isSprite=!0,this.type="Sprite",Ki===void 0){Ki=new Ut;const t=new Float32Array([-.5,-.5,0,0,0,.5,-.5,0,1,0,.5,.5,0,1,1,-.5,.5,0,0,1]),i=new ef(t,5);Ki.setIndex([0,1,2,0,2,3]),Ki.setAttribute("position",new la(i,3,0,!1)),Ki.setAttribute("uv",new la(i,2,3,!1))}this.geometry=Ki,this.material=e,this.center=new Oe(.5,.5),this.count=1}raycast(e,t){e.camera===null&&Qe('Sprite: "Raycaster.camera" needs to be set in order to raycast against sprites.'),Zi.setFromMatrixScale(this.matrixWorld),Bh.copy(e.camera.matrixWorld),this.modelViewMatrix.multiplyMatrices(e.camera.matrixWorldInverse,this.matrixWorld),Ji.setFromMatrixPosition(this.modelViewMatrix),e.camera.isPerspectiveCamera&&this.material.sizeAttenuation===!1&&Zi.multiplyScalar(-Ji.z);const i=this.material.rotation;let s,r;i!==0&&(r=Math.cos(i),s=Math.sin(i));const a=this.center;dr(hr.set(-.5,-.5,0),Ji,a,Zi,s,r),dr(As.set(.5,-.5,0),Ji,a,Zi,s,r),dr(ur.set(.5,.5,0),Ji,a,Zi,s,r),xc.set(0,0),$a.set(1,0),vc.set(1,1);let o=e.ray.intersectTriangle(hr,As,ur,!1,bs);if(o===null&&(dr(As.set(-.5,.5,0),Ji,a,Zi,s,r),$a.set(0,1),o=e.ray.intersectTriangle(hr,ur,As,!1,bs),o===null))return;const c=e.ray.origin.distanceTo(bs);c<e.near||c>e.far||t.push({distance:c,point:bs.clone(),uv:on.getInterpolation(bs,hr,As,ur,xc,$a,vc,new Oe),face:null,object:this})}copy(e,t){return super.copy(e,t),e.center!==void 0&&this.center.copy(e.center),this.material=e.material,this}}function dr(n,e,t,i,s,r){Qi.subVectors(n,t).addScalar(.5).multiply(i),s!==void 0?(Ts.x=r*Qi.x-s*Qi.y,Ts.y=s*Qi.x+r*Qi.y):Ts.copy(Qi),n.copy(e),n.x+=Ts.x,n.y+=Ts.y,n.applyMatrix4(Bh)}const On=new N,Ya=new N,fr=new N,ti=new N,qa=new N,pr=new N,ja=new N;class _a{constructor(e=new N,t=new N(0,0,-1)){this.origin=e,this.direction=t}set(e,t){return this.origin.copy(e),this.direction.copy(t),this}copy(e){return this.origin.copy(e.origin),this.direction.copy(e.direction),this}at(e,t){return t.copy(this.origin).addScaledVector(this.direction,e)}lookAt(e){return this.direction.copy(e).sub(this.origin).normalize(),this}recast(e){return this.origin.copy(this.at(e,On)),this}closestPointToPoint(e,t){t.subVectors(e,this.origin);const i=t.dot(this.direction);return i<0?t.copy(this.origin):t.copy(this.origin).addScaledVector(this.direction,i)}distanceToPoint(e){return Math.sqrt(this.distanceSqToPoint(e))}distanceSqToPoint(e){const t=On.subVectors(e,this.origin).dot(this.direction);return t<0?this.origin.distanceToSquared(e):(On.copy(this.origin).addScaledVector(this.direction,t),On.distanceToSquared(e))}distanceSqToSegment(e,t,i,s){Ya.copy(e).add(t).multiplyScalar(.5),fr.copy(t).sub(e).normalize(),ti.copy(this.origin).sub(Ya);const r=e.distanceTo(t)*.5,a=-this.direction.dot(fr),o=ti.dot(this.direction),c=-ti.dot(fr),l=ti.lengthSq(),u=Math.abs(1-a*a);let g,h,m,_;if(u>0)if(g=a*c-o,h=a*o-c,_=r*u,g>=0)if(h>=-_)if(h<=_){const S=1/u;g*=S,h*=S,m=g*(g+a*h+2*o)+h*(a*g+h+2*c)+l}else h=r,g=Math.max(0,-(a*h+o)),m=-g*g+h*(h+2*c)+l;else h=-r,g=Math.max(0,-(a*h+o)),m=-g*g+h*(h+2*c)+l;else h<=-_?(g=Math.max(0,-(-a*r+o)),h=g>0?-r:Math.min(Math.max(-r,-c),r),m=-g*g+h*(h+2*c)+l):h<=_?(g=0,h=Math.min(Math.max(-r,-c),r),m=h*(h+2*c)+l):(g=Math.max(0,-(a*r+o)),h=g>0?r:Math.min(Math.max(-r,-c),r),m=-g*g+h*(h+2*c)+l);else h=a>0?-r:r,g=Math.max(0,-(a*h+o)),m=-g*g+h*(h+2*c)+l;return i&&i.copy(this.origin).addScaledVector(this.direction,g),s&&s.copy(Ya).addScaledVector(fr,h),m}intersectSphere(e,t){On.subVectors(e.center,this.origin);const i=On.dot(this.direction),s=On.dot(On)-i*i,r=e.radius*e.radius;if(s>r)return null;const a=Math.sqrt(r-s),o=i-a,c=i+a;return c<0?null:o<0?this.at(c,t):this.at(o,t)}intersectsSphere(e){return e.radius<0?!1:this.distanceSqToPoint(e.center)<=e.radius*e.radius}distanceToPlane(e){const t=e.normal.dot(this.direction);if(t===0)return e.distanceToPoint(this.origin)===0?0:null;const i=-(this.origin.dot(e.normal)+e.constant)/t;return i>=0?i:null}intersectPlane(e,t){const i=this.distanceToPlane(e);return i===null?null:this.at(i,t)}intersectsPlane(e){const t=e.distanceToPoint(this.origin);return t===0||e.normal.dot(this.direction)*t<0}intersectBox(e,t){let i,s,r,a,o,c;const l=1/this.direction.x,u=1/this.direction.y,g=1/this.direction.z,h=this.origin;return l>=0?(i=(e.min.x-h.x)*l,s=(e.max.x-h.x)*l):(i=(e.max.x-h.x)*l,s=(e.min.x-h.x)*l),u>=0?(r=(e.min.y-h.y)*u,a=(e.max.y-h.y)*u):(r=(e.max.y-h.y)*u,a=(e.min.y-h.y)*u),i>a||r>s||((r>i||isNaN(i))&&(i=r),(a<s||isNaN(s))&&(s=a),g>=0?(o=(e.min.z-h.z)*g,c=(e.max.z-h.z)*g):(o=(e.max.z-h.z)*g,c=(e.min.z-h.z)*g),i>c||o>s)||((o>i||i!==i)&&(i=o),(c<s||s!==s)&&(s=c),s<0)?null:this.at(i>=0?i:s,t)}intersectsBox(e){return this.intersectBox(e,On)!==null}intersectTriangle(e,t,i,s,r){qa.subVectors(t,e),pr.subVectors(i,e),ja.crossVectors(qa,pr);let a=this.direction.dot(ja),o;if(a>0){if(s)return null;o=1}else if(a<0)o=-1,a=-a;else return null;ti.subVectors(this.origin,e);const c=o*this.direction.dot(pr.crossVectors(ti,pr));if(c<0)return null;const l=o*this.direction.dot(qa.cross(ti));if(l<0||c+l>a)return null;const u=-o*ti.dot(ja);return u<0?null:this.at(u/a,r)}applyMatrix4(e){return this.origin.applyMatrix4(e),this.direction.transformDirection(e),this}equals(e){return e.origin.equals(this.origin)&&e.direction.equals(this.direction)}clone(){return new this.constructor().copy(this)}}class zh extends Di{constructor(e){super(),this.isMeshBasicMaterial=!0,this.type="MeshBasicMaterial",this.color=new ke(16777215),this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.specularMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Li,this.combine=mh,this.reflectivity=1,this.refractionRatio=.98,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap="round",this.wireframeLinejoin="round",this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.lightMap=e.lightMap,this.lightMapIntensity=e.lightMapIntensity,this.aoMap=e.aoMap,this.aoMapIntensity=e.aoMapIntensity,this.specularMap=e.specularMap,this.alphaMap=e.alphaMap,this.envMap=e.envMap,this.envMapRotation.copy(e.envMapRotation),this.combine=e.combine,this.reflectivity=e.reflectivity,this.refractionRatio=e.refractionRatio,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.wireframeLinecap=e.wireframeLinecap,this.wireframeLinejoin=e.wireframeLinejoin,this.fog=e.fog,this}}const Mc=new gt,xi=new _a,mr=new js,Sc=new N,gr=new N,_r=new N,xr=new N,Ka=new N,vr=new N,yc=new N,Mr=new N;class Xn extends kt{constructor(e=new Ut,t=new zh){super(),this.isMesh=!0,this.type="Mesh",this.geometry=e,this.material=t,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.count=1,this.updateMorphTargets()}copy(e,t){return super.copy(e,t),e.morphTargetInfluences!==void 0&&(this.morphTargetInfluences=e.morphTargetInfluences.slice()),e.morphTargetDictionary!==void 0&&(this.morphTargetDictionary=Object.assign({},e.morphTargetDictionary)),this.material=Array.isArray(e.material)?e.material.slice():e.material,this.geometry=e.geometry,this}updateMorphTargets(){const t=this.geometry.morphAttributes,i=Object.keys(t);if(i.length>0){const s=t[i[0]];if(s!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let r=0,a=s.length;r<a;r++){const o=s[r].name||String(r);this.morphTargetInfluences.push(0),this.morphTargetDictionary[o]=r}}}}getVertexPosition(e,t){const i=this.geometry,s=i.attributes.position,r=i.morphAttributes.position,a=i.morphTargetsRelative;t.fromBufferAttribute(s,e);const o=this.morphTargetInfluences;if(r&&o){vr.set(0,0,0);for(let c=0,l=r.length;c<l;c++){const u=o[c],g=r[c];u!==0&&(Ka.fromBufferAttribute(g,e),a?vr.addScaledVector(Ka,u):vr.addScaledVector(Ka.sub(t),u))}t.add(vr)}return t}raycast(e,t){const i=this.geometry,s=this.material,r=this.matrixWorld;s!==void 0&&(i.boundingSphere===null&&i.computeBoundingSphere(),mr.copy(i.boundingSphere),mr.applyMatrix4(r),xi.copy(e.ray).recast(e.near),!(mr.containsPoint(xi.origin)===!1&&(xi.intersectSphere(mr,Sc)===null||xi.origin.distanceToSquared(Sc)>(e.far-e.near)**2))&&(Mc.copy(r).invert(),xi.copy(e.ray).applyMatrix4(Mc),!(i.boundingBox!==null&&xi.intersectsBox(i.boundingBox)===!1)&&this._computeIntersections(e,t,xi)))}_computeIntersections(e,t,i){let s;const r=this.geometry,a=this.material,o=r.index,c=r.attributes.position,l=r.attributes.uv,u=r.attributes.uv1,g=r.attributes.normal,h=r.groups,m=r.drawRange;if(o!==null)if(Array.isArray(a))for(let _=0,S=h.length;_<S;_++){const p=h[_],d=a[p.materialIndex],y=Math.max(p.start,m.start),A=Math.min(o.count,Math.min(p.start+p.count,m.start+m.count));for(let T=y,L=A;T<L;T+=3){const b=o.getX(T),I=o.getX(T+1),M=o.getX(T+2);s=Sr(this,d,e,i,l,u,g,b,I,M),s&&(s.faceIndex=Math.floor(T/3),s.face.materialIndex=p.materialIndex,t.push(s))}}else{const _=Math.max(0,m.start),S=Math.min(o.count,m.start+m.count);for(let p=_,d=S;p<d;p+=3){const y=o.getX(p),A=o.getX(p+1),T=o.getX(p+2);s=Sr(this,a,e,i,l,u,g,y,A,T),s&&(s.faceIndex=Math.floor(p/3),t.push(s))}}else if(c!==void 0)if(Array.isArray(a))for(let _=0,S=h.length;_<S;_++){const p=h[_],d=a[p.materialIndex],y=Math.max(p.start,m.start),A=Math.min(c.count,Math.min(p.start+p.count,m.start+m.count));for(let T=y,L=A;T<L;T+=3){const b=T,I=T+1,M=T+2;s=Sr(this,d,e,i,l,u,g,b,I,M),s&&(s.faceIndex=Math.floor(T/3),s.face.materialIndex=p.materialIndex,t.push(s))}}else{const _=Math.max(0,m.start),S=Math.min(c.count,m.start+m.count);for(let p=_,d=S;p<d;p+=3){const y=p,A=p+1,T=p+2;s=Sr(this,a,e,i,l,u,g,y,A,T),s&&(s.faceIndex=Math.floor(p/3),t.push(s))}}}}function sf(n,e,t,i,s,r,a,o){let c;if(e.side===Kt?c=i.intersectTriangle(a,r,s,!0,o):c=i.intersectTriangle(s,r,a,e.side===ci,o),c===null)return null;Mr.copy(o),Mr.applyMatrix4(n.matrixWorld);const l=t.ray.origin.distanceTo(Mr);return l<t.near||l>t.far?null:{distance:l,point:Mr.clone(),object:n}}function Sr(n,e,t,i,s,r,a,o,c,l){n.getVertexPosition(o,gr),n.getVertexPosition(c,_r),n.getVertexPosition(l,xr);const u=sf(n,e,t,i,gr,_r,xr,yc);if(u){const g=new N;on.getBarycoord(yc,gr,_r,xr,g),s&&(u.uv=on.getInterpolatedAttribute(s,o,c,l,g,new Oe)),r&&(u.uv1=on.getInterpolatedAttribute(r,o,c,l,g,new Oe)),a&&(u.normal=on.getInterpolatedAttribute(a,o,c,l,g,new N),u.normal.dot(i.direction)>0&&u.normal.multiplyScalar(-1));const h={a:o,b:c,c:l,normal:new N,materialIndex:0};on.getNormal(gr,_r,xr,h.normal),u.face=h,u.barycoord=g}return u}class rf extends zt{constructor(e=null,t=1,i=1,s,r,a,o,c,l=Lt,u=Lt,g,h){super(null,a,o,c,l,u,s,r,g,h),this.isDataTexture=!0,this.image={data:e,width:t,height:i},this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}}const Za=new N,af=new N,of=new He;class ii{constructor(e=new N(1,0,0),t=0){this.isPlane=!0,this.normal=e,this.constant=t}set(e,t){return this.normal.copy(e),this.constant=t,this}setComponents(e,t,i,s){return this.normal.set(e,t,i),this.constant=s,this}setFromNormalAndCoplanarPoint(e,t){return this.normal.copy(e),this.constant=-t.dot(this.normal),this}setFromCoplanarPoints(e,t,i){const s=Za.subVectors(i,t).cross(af.subVectors(e,t)).normalize();return this.setFromNormalAndCoplanarPoint(s,e),this}copy(e){return this.normal.copy(e.normal),this.constant=e.constant,this}normalize(){const e=1/this.normal.length();return this.normal.multiplyScalar(e),this.constant*=e,this}negate(){return this.constant*=-1,this.normal.negate(),this}distanceToPoint(e){return this.normal.dot(e)+this.constant}distanceToSphere(e){return this.distanceToPoint(e.center)-e.radius}projectPoint(e,t){return t.copy(e).addScaledVector(this.normal,-this.distanceToPoint(e))}intersectLine(e,t,i=!0){const s=e.delta(Za),r=this.normal.dot(s);if(r===0)return this.distanceToPoint(e.start)===0?t.copy(e.start):null;const a=-(e.start.dot(this.normal)+this.constant)/r;return i===!0&&(a<0||a>1)?null:t.copy(e.start).addScaledVector(s,a)}intersectsLine(e){const t=this.distanceToPoint(e.start),i=this.distanceToPoint(e.end);return t<0&&i>0||i<0&&t>0}intersectsBox(e){return e.intersectsPlane(this)}intersectsSphere(e){return e.intersectsPlane(this)}coplanarPoint(e){return e.copy(this.normal).multiplyScalar(-this.constant)}applyMatrix4(e,t){const i=t||of.getNormalMatrix(e),s=this.coplanarPoint(Za).applyMatrix4(e),r=this.normal.applyMatrix3(i).normalize();return this.constant=-s.dot(r),this}translate(e){return this.constant-=e.dot(this.normal),this}equals(e){return e.normal.equals(this.normal)&&e.constant===this.constant}clone(){return new this.constructor().copy(this)}}const vi=new js,lf=new Oe(.5,.5),yr=new N;class kh{constructor(e=new ii,t=new ii,i=new ii,s=new ii,r=new ii,a=new ii){this.planes=[e,t,i,s,r,a]}set(e,t,i,s,r,a){const o=this.planes;return o[0].copy(e),o[1].copy(t),o[2].copy(i),o[3].copy(s),o[4].copy(r),o[5].copy(a),this}copy(e){const t=this.planes;for(let i=0;i<6;i++)t[i].copy(e.planes[i]);return this}setFromProjectionMatrix(e,t=wn,i=!1){const s=this.planes,r=e.elements,a=r[0],o=r[1],c=r[2],l=r[3],u=r[4],g=r[5],h=r[6],m=r[7],_=r[8],S=r[9],p=r[10],d=r[11],y=r[12],A=r[13],T=r[14],L=r[15];if(s[0].setComponents(l-a,m-u,d-_,L-y).normalize(),s[1].setComponents(l+a,m+u,d+_,L+y).normalize(),s[2].setComponents(l+o,m+g,d+S,L+A).normalize(),s[3].setComponents(l-o,m-g,d-S,L-A).normalize(),i)s[4].setComponents(c,h,p,T).normalize(),s[5].setComponents(l-c,m-h,d-p,L-T).normalize();else if(s[4].setComponents(l-c,m-h,d-p,L-T).normalize(),t===wn)s[5].setComponents(l+c,m+h,d+p,L+T).normalize();else if(t===ra)s[5].setComponents(c,h,p,T).normalize();else throw new Error("THREE.Frustum.setFromProjectionMatrix(): Invalid coordinate system: "+t);return this}intersectsObject(e){if(e.boundingSphere!==void 0)e.boundingSphere===null&&e.computeBoundingSphere(),vi.copy(e.boundingSphere).applyMatrix4(e.matrixWorld);else{const t=e.geometry;t.boundingSphere===null&&t.computeBoundingSphere(),vi.copy(t.boundingSphere).applyMatrix4(e.matrixWorld)}return this.intersectsSphere(vi)}intersectsSprite(e){vi.center.set(0,0,0);const t=lf.distanceTo(e.center);return vi.radius=.7071067811865476+t,vi.applyMatrix4(e.matrixWorld),this.intersectsSphere(vi)}intersectsSphere(e){const t=this.planes,i=e.center,s=-e.radius;for(let r=0;r<6;r++)if(t[r].distanceToPoint(i)<s)return!1;return!0}intersectsBox(e){const t=this.planes;for(let i=0;i<6;i++){const s=t[i];if(yr.x=s.normal.x>0?e.max.x:e.min.x,yr.y=s.normal.y>0?e.max.y:e.min.y,yr.z=s.normal.z>0?e.max.z:e.min.z,s.distanceToPoint(yr)<0)return!1}return!0}containsPoint(e){const t=this.planes;for(let i=0;i<6;i++)if(t[i].distanceToPoint(e)<0)return!1;return!0}clone(){return new this.constructor().copy(this)}}class Al extends Di{constructor(e){super(),this.isLineBasicMaterial=!0,this.type="LineBasicMaterial",this.color=new ke(16777215),this.map=null,this.linewidth=1,this.linecap="round",this.linejoin="round",this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.linewidth=e.linewidth,this.linecap=e.linecap,this.linejoin=e.linejoin,this.fog=e.fog,this}}const ca=new N,ha=new N,Ec=new gt,ws=new _a,Er=new js,Ja=new N,bc=new N;class cf extends kt{constructor(e=new Ut,t=new Al){super(),this.isLine=!0,this.type="Line",this.geometry=e,this.material=t,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.updateMorphTargets()}copy(e,t){return super.copy(e,t),this.material=Array.isArray(e.material)?e.material.slice():e.material,this.geometry=e.geometry,this}computeLineDistances(){const e=this.geometry;if(e.index===null){const t=e.attributes.position,i=[0];for(let s=1,r=t.count;s<r;s++)ca.fromBufferAttribute(t,s-1),ha.fromBufferAttribute(t,s),i[s]=i[s-1],i[s]+=ca.distanceTo(ha);e.setAttribute("lineDistance",new mn(i,1))}else Ne("Line.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.");return this}raycast(e,t){const i=this.geometry,s=this.matrixWorld,r=e.params.Line.threshold,a=i.drawRange;if(i.boundingSphere===null&&i.computeBoundingSphere(),Er.copy(i.boundingSphere),Er.applyMatrix4(s),Er.radius+=r,e.ray.intersectsSphere(Er)===!1)return;Ec.copy(s).invert(),ws.copy(e.ray).applyMatrix4(Ec);const o=r/((this.scale.x+this.scale.y+this.scale.z)/3),c=o*o,l=this.isLineSegments?2:1,u=i.index,h=i.attributes.position;if(u!==null){const m=Math.max(0,a.start),_=Math.min(u.count,a.start+a.count);for(let S=m,p=_-1;S<p;S+=l){const d=u.getX(S),y=u.getX(S+1),A=br(this,e,ws,c,d,y,S);A&&t.push(A)}if(this.isLineLoop){const S=u.getX(_-1),p=u.getX(m),d=br(this,e,ws,c,S,p,_-1);d&&t.push(d)}}else{const m=Math.max(0,a.start),_=Math.min(h.count,a.start+a.count);for(let S=m,p=_-1;S<p;S+=l){const d=br(this,e,ws,c,S,S+1,S);d&&t.push(d)}if(this.isLineLoop){const S=br(this,e,ws,c,_-1,m,_-1);S&&t.push(S)}}}updateMorphTargets(){const t=this.geometry.morphAttributes,i=Object.keys(t);if(i.length>0){const s=t[i[0]];if(s!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let r=0,a=s.length;r<a;r++){const o=s[r].name||String(r);this.morphTargetInfluences.push(0),this.morphTargetDictionary[o]=r}}}}}function br(n,e,t,i,s,r,a){const o=n.geometry.attributes.position;if(ca.fromBufferAttribute(o,s),ha.fromBufferAttribute(o,r),t.distanceSqToSegment(ca,ha,Ja,bc)>i)return;Ja.applyMatrix4(n.matrixWorld);const l=e.ray.origin.distanceTo(Ja);if(!(l<e.near||l>e.far))return{distance:l,point:bc.clone().applyMatrix4(n.matrixWorld),index:a,face:null,faceIndex:null,barycoord:null,object:n}}const Tc=new N,Ac=new N;class Hh extends cf{constructor(e,t){super(e,t),this.isLineSegments=!0,this.type="LineSegments"}computeLineDistances(){const e=this.geometry;if(e.index===null){const t=e.attributes.position,i=[];for(let s=0,r=t.count;s<r;s+=2)Tc.fromBufferAttribute(t,s),Ac.fromBufferAttribute(t,s+1),i[s]=s===0?0:i[s-1],i[s+1]=i[s]+Tc.distanceTo(Ac);e.setAttribute("lineDistance",new mn(i,1))}else Ne("LineSegments.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.");return this}}class wl extends Di{constructor(e){super(),this.isPointsMaterial=!0,this.type="PointsMaterial",this.color=new ke(16777215),this.map=null,this.alphaMap=null,this.size=1,this.sizeAttenuation=!0,this.fog=!0,this.setValues(e)}copy(e){return super.copy(e),this.color.copy(e.color),this.map=e.map,this.alphaMap=e.alphaMap,this.size=e.size,this.sizeAttenuation=e.sizeAttenuation,this.fog=e.fog,this}}const wc=new gt,Qo=new _a,Tr=new js,Ar=new N;class Rl extends kt{constructor(e=new Ut,t=new wl){super(),this.isPoints=!0,this.type="Points",this.geometry=e,this.material=t,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.updateMorphTargets()}copy(e,t){return super.copy(e,t),this.material=Array.isArray(e.material)?e.material.slice():e.material,this.geometry=e.geometry,this}raycast(e,t){const i=this.geometry,s=this.matrixWorld,r=e.params.Points.threshold,a=i.drawRange;if(i.boundingSphere===null&&i.computeBoundingSphere(),Tr.copy(i.boundingSphere),Tr.applyMatrix4(s),Tr.radius+=r,e.ray.intersectsSphere(Tr)===!1)return;wc.copy(s).invert(),Qo.copy(e.ray).applyMatrix4(wc);const o=r/((this.scale.x+this.scale.y+this.scale.z)/3),c=o*o,l=i.index,g=i.attributes.position;if(l!==null){const h=Math.max(0,a.start),m=Math.min(l.count,a.start+a.count);for(let _=h,S=m;_<S;_++){const p=l.getX(_);Ar.fromBufferAttribute(g,p),Rc(Ar,p,c,s,e,t,this)}}else{const h=Math.max(0,a.start),m=Math.min(g.count,a.start+a.count);for(let _=h,S=m;_<S;_++)Ar.fromBufferAttribute(g,_),Rc(Ar,_,c,s,e,t,this)}}updateMorphTargets(){const t=this.geometry.morphAttributes,i=Object.keys(t);if(i.length>0){const s=t[i[0]];if(s!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let r=0,a=s.length;r<a;r++){const o=s[r].name||String(r);this.morphTargetInfluences.push(0),this.morphTargetDictionary[o]=r}}}}}function Rc(n,e,t,i,s,r,a){const o=Qo.distanceSqToPoint(n);if(o<t){const c=new N;Qo.closestPointToPoint(n,c),c.applyMatrix4(i);const l=s.ray.origin.distanceTo(c);if(l<s.near||l>s.far)return;r.push({distance:l,distanceToRay:Math.sqrt(o),point:c,index:e,face:null,faceIndex:null,barycoord:null,object:a})}}class Gh extends zt{constructor(e=[],t=Ci,i,s,r,a,o,c,l,u){super(e,t,i,s,r,a,o,c,l,u),this.isCubeTexture=!0,this.flipY=!1}get images(){return this.image}set images(e){this.image=e}}class Vh extends zt{constructor(e,t,i,s,r,a,o,c,l){super(e,t,i,s,r,a,o,c,l),this.isCanvasTexture=!0,this.needsUpdate=!0}}class gs extends zt{constructor(e,t,i=Ln,s,r,a,o=Lt,c=Lt,l,u=Wn,g=1){if(u!==Wn&&u!==Ai)throw new Error("DepthTexture format must be either THREE.DepthFormat or THREE.DepthStencilFormat");const h={width:e,height:t,depth:g};super(h,s,r,a,o,c,u,i,l),this.isDepthTexture=!0,this.flipY=!1,this.generateMipmaps=!1,this.compareFunction=null}copy(e){return super.copy(e),this.source=new bl(Object.assign({},e.image)),this.compareFunction=e.compareFunction,this}toJSON(e){const t=super.toJSON(e);return this.compareFunction!==null&&(t.compareFunction=this.compareFunction),t}}class hf extends gs{constructor(e,t=Ln,i=Ci,s,r,a=Lt,o=Lt,c,l=Wn){const u={width:e,height:e,depth:1},g=[u,u,u,u,u,u];super(e,e,t,i,s,r,a,o,c,l),this.image=g,this.isCubeDepthTexture=!0,this.isCubeTexture=!0}get images(){return this.image}set images(e){this.image=e}}class Wh extends zt{constructor(e=null){super(),this.sourceTexture=e,this.isExternalTexture=!0}copy(e){return super.copy(e),this.sourceTexture=e.sourceTexture,this}}class Ks extends Ut{constructor(e=1,t=1,i=1,s=1,r=1,a=1){super(),this.type="BoxGeometry",this.parameters={width:e,height:t,depth:i,widthSegments:s,heightSegments:r,depthSegments:a};const o=this;s=Math.floor(s),r=Math.floor(r),a=Math.floor(a);const c=[],l=[],u=[],g=[];let h=0,m=0;_("z","y","x",-1,-1,i,t,e,a,r,0),_("z","y","x",1,-1,i,t,-e,a,r,1),_("x","z","y",1,1,e,i,t,s,a,2),_("x","z","y",1,-1,e,i,-t,s,a,3),_("x","y","z",1,-1,e,t,i,s,r,4),_("x","y","z",-1,-1,e,t,-i,s,r,5),this.setIndex(c),this.setAttribute("position",new mn(l,3)),this.setAttribute("normal",new mn(u,3)),this.setAttribute("uv",new mn(g,2));function _(S,p,d,y,A,T,L,b,I,M,w){const B=T/I,C=L/M,z=T/2,Z=L/2,J=b/2,O=I+1,W=M+1;let $=0,se=0;const oe=new N;for(let he=0;he<W;he++){const Ee=he*C-Z;for(let Pe=0;Pe<O;Pe++){const Ge=Pe*B-z;oe[S]=Ge*y,oe[p]=Ee*A,oe[d]=J,l.push(oe.x,oe.y,oe.z),oe[S]=0,oe[p]=0,oe[d]=b>0?1:-1,u.push(oe.x,oe.y,oe.z),g.push(Pe/I),g.push(1-he/M),$+=1}}for(let he=0;he<M;he++)for(let Ee=0;Ee<I;Ee++){const Pe=h+Ee+O*he,Ge=h+Ee+O*(he+1),Je=h+(Ee+1)+O*(he+1),R=h+(Ee+1)+O*he;c.push(Pe,Ge,R),c.push(Ge,Je,R),se+=6}o.addGroup(m,se,w),m+=se,h+=$}}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(e){return new Ks(e.width,e.height,e.depth,e.widthSegments,e.heightSegments,e.depthSegments)}}class xa extends Ut{constructor(e=1,t=1,i=1,s=1){super(),this.type="PlaneGeometry",this.parameters={width:e,height:t,widthSegments:i,heightSegments:s};const r=e/2,a=t/2,o=Math.floor(i),c=Math.floor(s),l=o+1,u=c+1,g=e/o,h=t/c,m=[],_=[],S=[],p=[];for(let d=0;d<u;d++){const y=d*h-a;for(let A=0;A<l;A++){const T=A*g-r;_.push(T,-y,0),S.push(0,0,1),p.push(A/o),p.push(1-d/c)}}for(let d=0;d<c;d++)for(let y=0;y<o;y++){const A=y+l*d,T=y+l*(d+1),L=y+1+l*(d+1),b=y+1+l*d;m.push(A,T,b),m.push(T,L,b)}this.setIndex(m),this.setAttribute("position",new mn(_,3)),this.setAttribute("normal",new mn(S,3)),this.setAttribute("uv",new mn(p,2))}copy(e){return super.copy(e),this.parameters=Object.assign({},e.parameters),this}static fromJSON(e){return new xa(e.width,e.height,e.widthSegments,e.heightSegments)}}function _s(n){const e={};for(const t in n){e[t]={};for(const i in n[t]){const s=n[t][i];if(Cc(s))s.isRenderTargetTexture?(Ne("UniformsUtils: Textures of render targets cannot be cloned via cloneUniforms() or mergeUniforms()."),e[t][i]=null):e[t][i]=s.clone();else if(Array.isArray(s))if(Cc(s[0])){const r=[];for(let a=0,o=s.length;a<o;a++)r[a]=s[a].clone();e[t][i]=r}else e[t][i]=s.slice();else e[t][i]=s}}return e}function Gt(n){const e={};for(let t=0;t<n.length;t++){const i=_s(n[t]);for(const s in i)e[s]=i[s]}return e}function Cc(n){return n&&(n.isColor||n.isMatrix3||n.isMatrix4||n.isVector2||n.isVector3||n.isVector4||n.isTexture||n.isQuaternion)}function uf(n){const e=[];for(let t=0;t<n.length;t++)e.push(n[t].clone());return e}function Xh(n){const e=n.getRenderTarget();return e===null?n.outputColorSpace:e.isXRRenderTarget===!0?e.texture.colorSpace:Ze.workingColorSpace}const df={clone:_s,merge:Gt};var ff=`void main() {
	gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
}`,pf=`void main() {
	gl_FragColor = vec4( 1.0, 0.0, 0.0, 1.0 );
}`;class gn extends Di{constructor(e){super(),this.isShaderMaterial=!0,this.type="ShaderMaterial",this.defines={},this.uniforms={},this.uniformsGroups=[],this.vertexShader=ff,this.fragmentShader=pf,this.linewidth=1,this.wireframe=!1,this.wireframeLinewidth=1,this.fog=!1,this.lights=!1,this.clipping=!1,this.forceSinglePass=!0,this.extensions={clipCullDistance:!1,multiDraw:!1},this.defaultAttributeValues={color:[1,1,1],uv:[0,0],uv1:[0,0]},this.index0AttributeName=void 0,this.uniformsNeedUpdate=!1,this.glslVersion=null,e!==void 0&&this.setValues(e)}copy(e){return super.copy(e),this.fragmentShader=e.fragmentShader,this.vertexShader=e.vertexShader,this.uniforms=_s(e.uniforms),this.uniformsGroups=uf(e.uniformsGroups),this.defines=Object.assign({},e.defines),this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this.fog=e.fog,this.lights=e.lights,this.clipping=e.clipping,this.extensions=Object.assign({},e.extensions),this.glslVersion=e.glslVersion,this.defaultAttributeValues=Object.assign({},e.defaultAttributeValues),this.index0AttributeName=e.index0AttributeName,this.uniformsNeedUpdate=e.uniformsNeedUpdate,this}toJSON(e){const t=super.toJSON(e);t.glslVersion=this.glslVersion,t.uniforms={};for(const s in this.uniforms){const a=this.uniforms[s].value;a&&a.isTexture?t.uniforms[s]={type:"t",value:a.toJSON(e).uuid}:a&&a.isColor?t.uniforms[s]={type:"c",value:a.getHex()}:a&&a.isVector2?t.uniforms[s]={type:"v2",value:a.toArray()}:a&&a.isVector3?t.uniforms[s]={type:"v3",value:a.toArray()}:a&&a.isVector4?t.uniforms[s]={type:"v4",value:a.toArray()}:a&&a.isMatrix3?t.uniforms[s]={type:"m3",value:a.toArray()}:a&&a.isMatrix4?t.uniforms[s]={type:"m4",value:a.toArray()}:t.uniforms[s]={value:a}}Object.keys(this.defines).length>0&&(t.defines=this.defines),t.vertexShader=this.vertexShader,t.fragmentShader=this.fragmentShader,t.lights=this.lights,t.clipping=this.clipping;const i={};for(const s in this.extensions)this.extensions[s]===!0&&(i[s]=!0);return Object.keys(i).length>0&&(t.extensions=i),t}}class mf extends gn{constructor(e){super(e),this.isRawShaderMaterial=!0,this.type="RawShaderMaterial"}}class gf extends Di{constructor(e){super(),this.isMeshDepthMaterial=!0,this.type="MeshDepthMaterial",this.depthPacking=Ed,this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.wireframe=!1,this.wireframeLinewidth=1,this.setValues(e)}copy(e){return super.copy(e),this.depthPacking=e.depthPacking,this.map=e.map,this.alphaMap=e.alphaMap,this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this.wireframe=e.wireframe,this.wireframeLinewidth=e.wireframeLinewidth,this}}class _f extends Di{constructor(e){super(),this.isMeshDistanceMaterial=!0,this.type="MeshDistanceMaterial",this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.setValues(e)}copy(e){return super.copy(e),this.map=e.map,this.alphaMap=e.alphaMap,this.displacementMap=e.displacementMap,this.displacementScale=e.displacementScale,this.displacementBias=e.displacementBias,this}}const wr=new N,Rr=new hi,Sn=new N;class $h extends kt{constructor(){super(),this.isCamera=!0,this.type="Camera",this.matrixWorldInverse=new gt,this.projectionMatrix=new gt,this.projectionMatrixInverse=new gt,this.coordinateSystem=wn,this._reversedDepth=!1}get reversedDepth(){return this._reversedDepth}copy(e,t){return super.copy(e,t),this.matrixWorldInverse.copy(e.matrixWorldInverse),this.projectionMatrix.copy(e.projectionMatrix),this.projectionMatrixInverse.copy(e.projectionMatrixInverse),this.coordinateSystem=e.coordinateSystem,this}getWorldDirection(e){return super.getWorldDirection(e).negate()}updateMatrixWorld(e){super.updateMatrixWorld(e),this.matrixWorld.decompose(wr,Rr,Sn),Sn.x===1&&Sn.y===1&&Sn.z===1?this.matrixWorldInverse.copy(this.matrixWorld).invert():this.matrixWorldInverse.compose(wr,Rr,Sn.set(1,1,1)).invert()}updateWorldMatrix(e,t){super.updateWorldMatrix(e,t),this.matrixWorld.decompose(wr,Rr,Sn),Sn.x===1&&Sn.y===1&&Sn.z===1?this.matrixWorldInverse.copy(this.matrixWorld).invert():this.matrixWorldInverse.compose(wr,Rr,Sn.set(1,1,1)).invert()}clone(){return new this.constructor().copy(this)}}const ni=new N,Pc=new Oe,Lc=new Oe;class rn extends $h{constructor(e=50,t=1,i=.1,s=2e3){super(),this.isPerspectiveCamera=!0,this.type="PerspectiveCamera",this.fov=e,this.zoom=1,this.near=i,this.far=s,this.focus=10,this.aspect=t,this.view=null,this.filmGauge=35,this.filmOffset=0,this.updateProjectionMatrix()}copy(e,t){return super.copy(e,t),this.fov=e.fov,this.zoom=e.zoom,this.near=e.near,this.far=e.far,this.focus=e.focus,this.aspect=e.aspect,this.view=e.view===null?null:Object.assign({},e.view),this.filmGauge=e.filmGauge,this.filmOffset=e.filmOffset,this}setFocalLength(e){const t=.5*this.getFilmHeight()/e;this.fov=Jo*2*Math.atan(t),this.updateProjectionMatrix()}getFocalLength(){const e=Math.tan(Wr*.5*this.fov);return .5*this.getFilmHeight()/e}getEffectiveFOV(){return Jo*2*Math.atan(Math.tan(Wr*.5*this.fov)/this.zoom)}getFilmWidth(){return this.filmGauge*Math.min(this.aspect,1)}getFilmHeight(){return this.filmGauge/Math.max(this.aspect,1)}getViewBounds(e,t,i){ni.set(-1,-1,.5).applyMatrix4(this.projectionMatrixInverse),t.set(ni.x,ni.y).multiplyScalar(-e/ni.z),ni.set(1,1,.5).applyMatrix4(this.projectionMatrixInverse),i.set(ni.x,ni.y).multiplyScalar(-e/ni.z)}getViewSize(e,t){return this.getViewBounds(e,Pc,Lc),t.subVectors(Lc,Pc)}setViewOffset(e,t,i,s,r,a){this.aspect=e/t,this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=e,this.view.fullHeight=t,this.view.offsetX=i,this.view.offsetY=s,this.view.width=r,this.view.height=a,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const e=this.near;let t=e*Math.tan(Wr*.5*this.fov)/this.zoom,i=2*t,s=this.aspect*i,r=-.5*s;const a=this.view;if(this.view!==null&&this.view.enabled){const c=a.fullWidth,l=a.fullHeight;r+=a.offsetX*s/c,t-=a.offsetY*i/l,s*=a.width/c,i*=a.height/l}const o=this.filmOffset;o!==0&&(r+=e*o/this.getFilmWidth()),this.projectionMatrix.makePerspective(r,r+s,t,t-i,e,this.far,this.coordinateSystem,this.reversedDepth),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(e){const t=super.toJSON(e);return t.object.fov=this.fov,t.object.zoom=this.zoom,t.object.near=this.near,t.object.far=this.far,t.object.focus=this.focus,t.object.aspect=this.aspect,this.view!==null&&(t.object.view=Object.assign({},this.view)),t.object.filmGauge=this.filmGauge,t.object.filmOffset=this.filmOffset,t}}class Yh extends $h{constructor(e=-1,t=1,i=1,s=-1,r=.1,a=2e3){super(),this.isOrthographicCamera=!0,this.type="OrthographicCamera",this.zoom=1,this.view=null,this.left=e,this.right=t,this.top=i,this.bottom=s,this.near=r,this.far=a,this.updateProjectionMatrix()}copy(e,t){return super.copy(e,t),this.left=e.left,this.right=e.right,this.top=e.top,this.bottom=e.bottom,this.near=e.near,this.far=e.far,this.zoom=e.zoom,this.view=e.view===null?null:Object.assign({},e.view),this}setViewOffset(e,t,i,s,r,a){this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=e,this.view.fullHeight=t,this.view.offsetX=i,this.view.offsetY=s,this.view.width=r,this.view.height=a,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const e=(this.right-this.left)/(2*this.zoom),t=(this.top-this.bottom)/(2*this.zoom),i=(this.right+this.left)/2,s=(this.top+this.bottom)/2;let r=i-e,a=i+e,o=s+t,c=s-t;if(this.view!==null&&this.view.enabled){const l=(this.right-this.left)/this.view.fullWidth/this.zoom,u=(this.top-this.bottom)/this.view.fullHeight/this.zoom;r+=l*this.view.offsetX,a=r+l*this.view.width,o-=u*this.view.offsetY,c=o-u*this.view.height}this.projectionMatrix.makeOrthographic(r,a,o,c,this.near,this.far,this.coordinateSystem,this.reversedDepth),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(e){const t=super.toJSON(e);return t.object.zoom=this.zoom,t.object.left=this.left,t.object.right=this.right,t.object.top=this.top,t.object.bottom=this.bottom,t.object.near=this.near,t.object.far=this.far,this.view!==null&&(t.object.view=Object.assign({},this.view)),t}}const es=-90,ts=1;class xf extends kt{constructor(e,t,i){super(),this.type="CubeCamera",this.renderTarget=i,this.coordinateSystem=null,this.activeMipmapLevel=0;const s=new rn(es,ts,e,t);s.layers=this.layers,this.add(s);const r=new rn(es,ts,e,t);r.layers=this.layers,this.add(r);const a=new rn(es,ts,e,t);a.layers=this.layers,this.add(a);const o=new rn(es,ts,e,t);o.layers=this.layers,this.add(o);const c=new rn(es,ts,e,t);c.layers=this.layers,this.add(c);const l=new rn(es,ts,e,t);l.layers=this.layers,this.add(l)}updateCoordinateSystem(){const e=this.coordinateSystem,t=this.children.concat(),[i,s,r,a,o,c]=t;for(const l of t)this.remove(l);if(e===wn)i.up.set(0,1,0),i.lookAt(1,0,0),s.up.set(0,1,0),s.lookAt(-1,0,0),r.up.set(0,0,-1),r.lookAt(0,1,0),a.up.set(0,0,1),a.lookAt(0,-1,0),o.up.set(0,1,0),o.lookAt(0,0,1),c.up.set(0,1,0),c.lookAt(0,0,-1);else if(e===ra)i.up.set(0,-1,0),i.lookAt(-1,0,0),s.up.set(0,-1,0),s.lookAt(1,0,0),r.up.set(0,0,1),r.lookAt(0,1,0),a.up.set(0,0,-1),a.lookAt(0,-1,0),o.up.set(0,-1,0),o.lookAt(0,0,1),c.up.set(0,-1,0),c.lookAt(0,0,-1);else throw new Error("THREE.CubeCamera.updateCoordinateSystem(): Invalid coordinate system: "+e);for(const l of t)this.add(l),l.updateMatrixWorld()}update(e,t){this.parent===null&&this.updateMatrixWorld();const{renderTarget:i,activeMipmapLevel:s}=this;this.coordinateSystem!==e.coordinateSystem&&(this.coordinateSystem=e.coordinateSystem,this.updateCoordinateSystem());const[r,a,o,c,l,u]=this.children,g=e.getRenderTarget(),h=e.getActiveCubeFace(),m=e.getActiveMipmapLevel(),_=e.xr.enabled;e.xr.enabled=!1;const S=i.texture.generateMipmaps;i.texture.generateMipmaps=!1;let p=!1;e.isWebGLRenderer===!0?p=e.state.buffers.depth.getReversed():p=e.reversedDepthBuffer,e.setRenderTarget(i,0,s),p&&e.autoClear===!1&&e.clearDepth(),e.render(t,r),e.setRenderTarget(i,1,s),p&&e.autoClear===!1&&e.clearDepth(),e.render(t,a),e.setRenderTarget(i,2,s),p&&e.autoClear===!1&&e.clearDepth(),e.render(t,o),e.setRenderTarget(i,3,s),p&&e.autoClear===!1&&e.clearDepth(),e.render(t,c),e.setRenderTarget(i,4,s),p&&e.autoClear===!1&&e.clearDepth(),e.render(t,l),i.texture.generateMipmaps=S,e.setRenderTarget(i,5,s),p&&e.autoClear===!1&&e.clearDepth(),e.render(t,u),e.setRenderTarget(g,h,m),e.xr.enabled=_,i.texture.needsPMREMUpdate=!0}}class vf extends rn{constructor(e=[]){super(),this.isArrayCamera=!0,this.isMultiViewCamera=!1,this.cameras=e}}class Dc{constructor(e=1,t=0,i=0){this.radius=e,this.phi=t,this.theta=i}set(e,t,i){return this.radius=e,this.phi=t,this.theta=i,this}copy(e){return this.radius=e.radius,this.phi=e.phi,this.theta=e.theta,this}makeSafe(){return this.phi=je(this.phi,1e-6,Math.PI-1e-6),this}setFromVector3(e){return this.setFromCartesianCoords(e.x,e.y,e.z)}setFromCartesianCoords(e,t,i){return this.radius=Math.sqrt(e*e+t*t+i*i),this.radius===0?(this.theta=0,this.phi=0):(this.theta=Math.atan2(e,i),this.phi=Math.acos(je(t/this.radius,-1,1))),this}clone(){return new this.constructor().copy(this)}}class qh{static{qh.prototype.isMatrix2=!0}constructor(e,t,i,s){this.elements=[1,0,0,1],e!==void 0&&this.set(e,t,i,s)}identity(){return this.set(1,0,0,1),this}fromArray(e,t=0){for(let i=0;i<4;i++)this.elements[i]=e[i+t];return this}set(e,t,i,s){const r=this.elements;return r[0]=e,r[2]=t,r[1]=i,r[3]=s,this}}class Mf extends fi{constructor(e,t=null){super(),this.object=e,this.domElement=t,this.enabled=!0,this.state=-1,this.keys={},this.mouseButtons={LEFT:null,MIDDLE:null,RIGHT:null},this.touches={ONE:null,TWO:null}}connect(e){if(e===void 0){Ne("Controls: connect() now requires an element.");return}this.domElement!==null&&this.disconnect(),this.domElement=e}disconnect(){}dispose(){}update(){}}function Ic(n,e,t,i){const s=Sf(i);switch(t){case Rh:return n*e;case Ph:return n*e/s.components*s.byteLength;case vl:return n*e/s.components*s.byteLength;case Pi:return n*e*2/s.components*s.byteLength;case Ml:return n*e*2/s.components*s.byteLength;case Ch:return n*e*3/s.components*s.byteLength;case pn:return n*e*4/s.components*s.byteLength;case Sl:return n*e*4/s.components*s.byteLength;case kr:case Hr:return Math.floor((n+3)/4)*Math.floor((e+3)/4)*8;case Gr:case Vr:return Math.floor((n+3)/4)*Math.floor((e+3)/4)*16;case So:case Eo:return Math.max(n,16)*Math.max(e,8)/4;case Mo:case yo:return Math.max(n,8)*Math.max(e,8)/2;case bo:case To:case wo:case Ro:return Math.floor((n+3)/4)*Math.floor((e+3)/4)*8;case Ao:case ta:case Co:return Math.floor((n+3)/4)*Math.floor((e+3)/4)*16;case Po:return Math.floor((n+3)/4)*Math.floor((e+3)/4)*16;case Lo:return Math.floor((n+4)/5)*Math.floor((e+3)/4)*16;case Do:return Math.floor((n+4)/5)*Math.floor((e+4)/5)*16;case Io:return Math.floor((n+5)/6)*Math.floor((e+4)/5)*16;case Uo:return Math.floor((n+5)/6)*Math.floor((e+5)/6)*16;case No:return Math.floor((n+7)/8)*Math.floor((e+4)/5)*16;case Fo:return Math.floor((n+7)/8)*Math.floor((e+5)/6)*16;case Oo:return Math.floor((n+7)/8)*Math.floor((e+7)/8)*16;case Bo:return Math.floor((n+9)/10)*Math.floor((e+4)/5)*16;case zo:return Math.floor((n+9)/10)*Math.floor((e+5)/6)*16;case ko:return Math.floor((n+9)/10)*Math.floor((e+7)/8)*16;case Ho:return Math.floor((n+9)/10)*Math.floor((e+9)/10)*16;case Go:return Math.floor((n+11)/12)*Math.floor((e+9)/10)*16;case Vo:return Math.floor((n+11)/12)*Math.floor((e+11)/12)*16;case Wo:case Xo:case $o:return Math.ceil(n/4)*Math.ceil(e/4)*16;case Yo:case qo:return Math.ceil(n/4)*Math.ceil(e/4)*8;case na:case jo:return Math.ceil(n/4)*Math.ceil(e/4)*16}throw new Error(`Unable to determine texture byte length for ${t} format.`)}function Sf(n){switch(n){case an:case bh:return{byteLength:1,components:1};case ks:case Th:case Vn:return{byteLength:2,components:1};case _l:case xl:return{byteLength:2,components:4};case Ln:case gl:case An:return{byteLength:4,components:1};case Ah:case wh:return{byteLength:4,components:3}}throw new Error(`Unknown texture type ${n}.`)}typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("register",{detail:{revision:ml}}));typeof window<"u"&&(window.__THREE__?Ne("WARNING: Multiple instances of Three.js being imported."):window.__THREE__=ml);/**
 * @license
 * Copyright 2010-2026 Three.js Authors
 * SPDX-License-Identifier: MIT
 */function jh(){let n=null,e=!1,t=null,i=null;function s(r,a){t(r,a),i=n.requestAnimationFrame(s)}return{start:function(){e!==!0&&t!==null&&n!==null&&(i=n.requestAnimationFrame(s),e=!0)},stop:function(){n!==null&&n.cancelAnimationFrame(i),e=!1},setAnimationLoop:function(r){t=r},setContext:function(r){n=r}}}function yf(n){const e=new WeakMap;function t(o,c){const l=o.array,u=o.usage,g=l.byteLength,h=n.createBuffer();n.bindBuffer(c,h),n.bufferData(c,l,u),o.onUploadCallback();let m;if(l instanceof Float32Array)m=n.FLOAT;else if(typeof Float16Array<"u"&&l instanceof Float16Array)m=n.HALF_FLOAT;else if(l instanceof Uint16Array)o.isFloat16BufferAttribute?m=n.HALF_FLOAT:m=n.UNSIGNED_SHORT;else if(l instanceof Int16Array)m=n.SHORT;else if(l instanceof Uint32Array)m=n.UNSIGNED_INT;else if(l instanceof Int32Array)m=n.INT;else if(l instanceof Int8Array)m=n.BYTE;else if(l instanceof Uint8Array)m=n.UNSIGNED_BYTE;else if(l instanceof Uint8ClampedArray)m=n.UNSIGNED_BYTE;else throw new Error("THREE.WebGLAttributes: Unsupported buffer data format: "+l);return{buffer:h,type:m,bytesPerElement:l.BYTES_PER_ELEMENT,version:o.version,size:g}}function i(o,c,l){const u=c.array,g=c.updateRanges;if(n.bindBuffer(l,o),g.length===0)n.bufferSubData(l,0,u);else{g.sort((m,_)=>m.start-_.start);let h=0;for(let m=1;m<g.length;m++){const _=g[h],S=g[m];S.start<=_.start+_.count+1?_.count=Math.max(_.count,S.start+S.count-_.start):(++h,g[h]=S)}g.length=h+1;for(let m=0,_=g.length;m<_;m++){const S=g[m];n.bufferSubData(l,S.start*u.BYTES_PER_ELEMENT,u,S.start,S.count)}c.clearUpdateRanges()}c.onUploadCallback()}function s(o){return o.isInterleavedBufferAttribute&&(o=o.data),e.get(o)}function r(o){o.isInterleavedBufferAttribute&&(o=o.data);const c=e.get(o);c&&(n.deleteBuffer(c.buffer),e.delete(o))}function a(o,c){if(o.isInterleavedBufferAttribute&&(o=o.data),o.isGLBufferAttribute){const u=e.get(o);(!u||u.version<o.version)&&e.set(o,{buffer:o.buffer,type:o.type,bytesPerElement:o.elementSize,version:o.version});return}const l=e.get(o);if(l===void 0)e.set(o,t(o,c));else if(l.version<o.version){if(l.size!==o.array.byteLength)throw new Error("THREE.WebGLAttributes: The size of the buffer attribute's array buffer does not match the original size. Resizing buffer attributes is not supported.");i(l.buffer,o,c),l.version=o.version}}return{get:s,remove:r,update:a}}var Ef=`#ifdef USE_ALPHAHASH
	if ( diffuseColor.a < getAlphaHashThreshold( vPosition ) ) discard;
#endif`,bf=`#ifdef USE_ALPHAHASH
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
#endif`,Tf=`#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, vAlphaMapUv ).g;
#endif`,Af=`#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,wf=`#ifdef USE_ALPHATEST
	#ifdef ALPHA_TO_COVERAGE
	diffuseColor.a = smoothstep( alphaTest, alphaTest + fwidth( diffuseColor.a ), diffuseColor.a );
	if ( diffuseColor.a == 0.0 ) discard;
	#else
	if ( diffuseColor.a < alphaTest ) discard;
	#endif
#endif`,Rf=`#ifdef USE_ALPHATEST
	uniform float alphaTest;
#endif`,Cf=`#ifdef USE_AOMAP
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
#endif`,Pf=`#ifdef USE_AOMAP
	uniform sampler2D aoMap;
	uniform float aoMapIntensity;
#endif`,Lf=`#ifdef USE_BATCHING
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
#endif`,Df=`#ifdef USE_BATCHING
	mat4 batchingMatrix = getBatchingMatrix( getIndirectIndex( gl_DrawID ) );
#endif`,If=`vec3 transformed = vec3( position );
#ifdef USE_ALPHAHASH
	vPosition = vec3( position );
#endif`,Uf=`vec3 objectNormal = vec3( normal );
#ifdef USE_TANGENT
	vec3 objectTangent = vec3( tangent.xyz );
#endif`,Nf=`float G_BlinnPhong_Implicit( ) {
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
} // validated`,Ff=`#ifdef USE_IRIDESCENCE
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
#endif`,Of=`#ifdef USE_BUMPMAP
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
#endif`,Bf=`#if NUM_CLIPPING_PLANES > 0
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
#endif`,zf=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
	uniform vec4 clippingPlanes[ NUM_CLIPPING_PLANES ];
#endif`,kf=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
#endif`,Hf=`#if NUM_CLIPPING_PLANES > 0
	vClipPosition = - mvPosition.xyz;
#endif`,Gf=`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )
	diffuseColor *= vColor;
#endif`,Vf=`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )
	varying vec4 vColor;
#endif`,Wf=`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA ) || defined( USE_INSTANCING_COLOR ) || defined( USE_BATCHING_COLOR )
	varying vec4 vColor;
#endif`,Xf=`#if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA ) || defined( USE_INSTANCING_COLOR ) || defined( USE_BATCHING_COLOR )
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
#endif`,$f=`#define PI 3.141592653589793
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
} // validated`,Yf=`#ifdef ENVMAP_TYPE_CUBE_UV
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
#endif`,qf=`vec3 transformedNormal = objectNormal;
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
#endif`,jf=`#ifdef USE_DISPLACEMENTMAP
	uniform sampler2D displacementMap;
	uniform float displacementScale;
	uniform float displacementBias;
#endif`,Kf=`#ifdef USE_DISPLACEMENTMAP
	transformed += normalize( objectNormal ) * ( texture2D( displacementMap, vDisplacementMapUv ).x * displacementScale + displacementBias );
#endif`,Zf=`#ifdef USE_EMISSIVEMAP
	vec4 emissiveColor = texture2D( emissiveMap, vEmissiveMapUv );
	#ifdef DECODE_VIDEO_TEXTURE_EMISSIVE
		emissiveColor = sRGBTransferEOTF( emissiveColor );
	#endif
	totalEmissiveRadiance *= emissiveColor.rgb;
#endif`,Jf=`#ifdef USE_EMISSIVEMAP
	uniform sampler2D emissiveMap;
#endif`,Qf="gl_FragColor = linearToOutputTexel( gl_FragColor );",ep=`vec4 LinearTransferOETF( in vec4 value ) {
	return value;
}
vec4 sRGBTransferEOTF( in vec4 value ) {
	return vec4( mix( pow( value.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), value.rgb * 0.0773993808, vec3( lessThanEqual( value.rgb, vec3( 0.04045 ) ) ) ), value.a );
}
vec4 sRGBTransferOETF( in vec4 value ) {
	return vec4( mix( pow( value.rgb, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), value.rgb * 12.92, vec3( lessThanEqual( value.rgb, vec3( 0.0031308 ) ) ) ), value.a );
}`,tp=`#ifdef USE_ENVMAP
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
#endif`,np=`#ifdef USE_ENVMAP
	uniform float envMapIntensity;
	uniform mat3 envMapRotation;
	#ifdef ENVMAP_TYPE_CUBE
		uniform samplerCube envMap;
	#else
		uniform sampler2D envMap;
	#endif
#endif`,ip=`#ifdef USE_ENVMAP
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
#endif`,sp=`#ifdef USE_ENVMAP
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		
		varying vec3 vWorldPosition;
	#else
		varying vec3 vReflect;
		uniform float refractionRatio;
	#endif
#endif`,rp=`#ifdef USE_ENVMAP
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
#endif`,ap=`#ifdef USE_FOG
	vFogDepth = - mvPosition.z;
#endif`,op=`#ifdef USE_FOG
	varying float vFogDepth;
#endif`,lp=`#ifdef USE_FOG
	#ifdef FOG_EXP2
		float fogFactor = 1.0 - exp( - fogDensity * fogDensity * vFogDepth * vFogDepth );
	#else
		float fogFactor = smoothstep( fogNear, fogFar, vFogDepth );
	#endif
	gl_FragColor.rgb = mix( gl_FragColor.rgb, fogColor, fogFactor );
#endif`,cp=`#ifdef USE_FOG
	uniform vec3 fogColor;
	varying float vFogDepth;
	#ifdef FOG_EXP2
		uniform float fogDensity;
	#else
		uniform float fogNear;
		uniform float fogFar;
	#endif
#endif`,hp=`#ifdef USE_GRADIENTMAP
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
}`,up=`#ifdef USE_LIGHTMAP
	uniform sampler2D lightMap;
	uniform float lightMapIntensity;
#endif`,dp=`LambertMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularStrength = specularStrength;`,fp=`varying vec3 vViewPosition;
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
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Lambert`,pp=`uniform bool receiveShadow;
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
#include <lightprobes_pars_fragment>`,mp=`#ifdef USE_ENVMAP
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
#endif`,gp=`ToonMaterial material;
material.diffuseColor = diffuseColor.rgb;`,_p=`varying vec3 vViewPosition;
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
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Toon`,xp=`BlinnPhongMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularColor = specular;
material.specularShininess = shininess;
material.specularStrength = specularStrength;`,vp=`varying vec3 vViewPosition;
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
#define RE_IndirectDiffuse		RE_IndirectDiffuse_BlinnPhong`,Mp=`PhysicalMaterial material;
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
#endif`,Sp=`uniform sampler2D dfgLUT;
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
}`,yp=`
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
#endif`,Ep=`#if defined( RE_IndirectDiffuse )
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
#endif`,bp=`#if defined( RE_IndirectDiffuse )
	#if defined( LAMBERT ) || defined( PHONG )
		irradiance += iblIrradiance;
	#endif
	RE_IndirectDiffuse( irradiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif
#if defined( RE_IndirectSpecular )
	RE_IndirectSpecular( radiance, iblIrradiance, clearcoatRadiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif`,Tp=`#ifdef USE_LIGHT_PROBES_GRID
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
#endif`,Ap=`#if defined( USE_LOGARITHMIC_DEPTH_BUFFER )
	gl_FragDepth = vIsPerspective == 0.0 ? gl_FragCoord.z : log2( vFragDepth ) * logDepthBufFC * 0.5;
#endif`,wp=`#if defined( USE_LOGARITHMIC_DEPTH_BUFFER )
	uniform float logDepthBufFC;
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,Rp=`#ifdef USE_LOGARITHMIC_DEPTH_BUFFER
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,Cp=`#ifdef USE_LOGARITHMIC_DEPTH_BUFFER
	vFragDepth = 1.0 + gl_Position.w;
	vIsPerspective = float( isPerspectiveMatrix( projectionMatrix ) );
#endif`,Pp=`#ifdef USE_MAP
	vec4 sampledDiffuseColor = texture2D( map, vMapUv );
	#ifdef DECODE_VIDEO_TEXTURE
		sampledDiffuseColor = sRGBTransferEOTF( sampledDiffuseColor );
	#endif
	diffuseColor *= sampledDiffuseColor;
#endif`,Lp=`#ifdef USE_MAP
	uniform sampler2D map;
#endif`,Dp=`#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
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
#endif`,Ip=`#if defined( USE_POINTS_UV )
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
#endif`,Up=`float metalnessFactor = metalness;
#ifdef USE_METALNESSMAP
	vec4 texelMetalness = texture2D( metalnessMap, vMetalnessMapUv );
	metalnessFactor *= texelMetalness.b;
#endif`,Np=`#ifdef USE_METALNESSMAP
	uniform sampler2D metalnessMap;
#endif`,Fp=`#ifdef USE_INSTANCING_MORPH
	float morphTargetInfluences[ MORPHTARGETS_COUNT ];
	float morphTargetBaseInfluence = texelFetch( morphTexture, ivec2( 0, gl_InstanceID ), 0 ).r;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		morphTargetInfluences[i] =  texelFetch( morphTexture, ivec2( i + 1, gl_InstanceID ), 0 ).r;
	}
#endif`,Op=`#if defined( USE_MORPHCOLORS )
	vColor *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		#if defined( USE_COLOR_ALPHA )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ) * morphTargetInfluences[ i ];
		#elif defined( USE_COLOR )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ).rgb * morphTargetInfluences[ i ];
		#endif
	}
#endif`,Bp=`#ifdef USE_MORPHNORMALS
	objectNormal *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) objectNormal += getMorph( gl_VertexID, i, 1 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,zp=`#ifdef USE_MORPHTARGETS
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
#endif`,kp=`#ifdef USE_MORPHTARGETS
	transformed *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) transformed += getMorph( gl_VertexID, i, 0 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,Hp=`float faceDirection = gl_FrontFacing ? 1.0 : - 1.0;
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
vec3 nonPerturbedNormal = normal;`,Gp=`#ifdef USE_NORMALMAP_OBJECTSPACE
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
#endif`,Vp=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,Wp=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,Xp=`#ifndef FLAT_SHADED
	vNormal = normalize( transformedNormal );
	#ifdef USE_TANGENT
		vTangent = normalize( transformedTangent );
		vBitangent = normalize( cross( vNormal, vTangent ) * tangent.w );
	#endif
#endif`,$p=`#ifdef USE_NORMALMAP
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
#endif`,Yp=`#ifdef USE_CLEARCOAT
	vec3 clearcoatNormal = nonPerturbedNormal;
#endif`,qp=`#ifdef USE_CLEARCOAT_NORMALMAP
	vec3 clearcoatMapN = texture2D( clearcoatNormalMap, vClearcoatNormalMapUv ).xyz * 2.0 - 1.0;
	clearcoatMapN.xy *= clearcoatNormalScale;
	clearcoatNormal = normalize( tbn2 * clearcoatMapN );
#endif`,jp=`#ifdef USE_CLEARCOATMAP
	uniform sampler2D clearcoatMap;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform sampler2D clearcoatNormalMap;
	uniform vec2 clearcoatNormalScale;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform sampler2D clearcoatRoughnessMap;
#endif`,Kp=`#ifdef USE_IRIDESCENCEMAP
	uniform sampler2D iridescenceMap;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform sampler2D iridescenceThicknessMap;
#endif`,Zp=`#ifdef OPAQUE
diffuseColor.a = 1.0;
#endif
#ifdef USE_TRANSMISSION
diffuseColor.a *= material.transmissionAlpha;
#endif
gl_FragColor = vec4( outgoingLight, diffuseColor.a );`,Jp=`vec3 packNormalToRGB( const in vec3 normal ) {
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
}`,Qp=`#ifdef PREMULTIPLIED_ALPHA
	gl_FragColor.rgb *= gl_FragColor.a;
#endif`,em=`vec4 mvPosition = vec4( transformed, 1.0 );
#ifdef USE_BATCHING
	mvPosition = batchingMatrix * mvPosition;
#endif
#ifdef USE_INSTANCING
	mvPosition = instanceMatrix * mvPosition;
#endif
mvPosition = modelViewMatrix * mvPosition;
gl_Position = projectionMatrix * mvPosition;`,tm=`#ifdef DITHERING
	gl_FragColor.rgb = dithering( gl_FragColor.rgb );
#endif`,nm=`#ifdef DITHERING
	vec3 dithering( vec3 color ) {
		float grid_position = rand( gl_FragCoord.xy );
		vec3 dither_shift_RGB = vec3( 0.25 / 255.0, -0.25 / 255.0, 0.25 / 255.0 );
		dither_shift_RGB = mix( 2.0 * dither_shift_RGB, -2.0 * dither_shift_RGB, grid_position );
		return color + dither_shift_RGB;
	}
#endif`,im=`float roughnessFactor = roughness;
#ifdef USE_ROUGHNESSMAP
	vec4 texelRoughness = texture2D( roughnessMap, vRoughnessMapUv );
	roughnessFactor *= texelRoughness.g;
#endif`,sm=`#ifdef USE_ROUGHNESSMAP
	uniform sampler2D roughnessMap;
#endif`,rm=`#if NUM_SPOT_LIGHT_COORDS > 0
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
#endif`,am=`#if NUM_SPOT_LIGHT_COORDS > 0
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
#endif`,om=`#if ( defined( USE_SHADOWMAP ) && ( NUM_DIR_LIGHT_SHADOWS > 0 || NUM_POINT_LIGHT_SHADOWS > 0 ) ) || ( NUM_SPOT_LIGHT_COORDS > 0 )
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
#endif`,lm=`float getShadowMask() {
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
}`,cm=`#ifdef USE_SKINNING
	mat4 boneMatX = getBoneMatrix( skinIndex.x );
	mat4 boneMatY = getBoneMatrix( skinIndex.y );
	mat4 boneMatZ = getBoneMatrix( skinIndex.z );
	mat4 boneMatW = getBoneMatrix( skinIndex.w );
#endif`,hm=`#ifdef USE_SKINNING
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
#endif`,um=`#ifdef USE_SKINNING
	vec4 skinVertex = bindMatrix * vec4( transformed, 1.0 );
	vec4 skinned = vec4( 0.0 );
	skinned += boneMatX * skinVertex * skinWeight.x;
	skinned += boneMatY * skinVertex * skinWeight.y;
	skinned += boneMatZ * skinVertex * skinWeight.z;
	skinned += boneMatW * skinVertex * skinWeight.w;
	transformed = ( bindMatrixInverse * skinned ).xyz;
#endif`,dm=`#ifdef USE_SKINNING
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
#endif`,fm=`float specularStrength;
#ifdef USE_SPECULARMAP
	vec4 texelSpecular = texture2D( specularMap, vSpecularMapUv );
	specularStrength = texelSpecular.r;
#else
	specularStrength = 1.0;
#endif`,pm=`#ifdef USE_SPECULARMAP
	uniform sampler2D specularMap;
#endif`,mm=`#if defined( TONE_MAPPING )
	gl_FragColor.rgb = toneMapping( gl_FragColor.rgb );
#endif`,gm=`#ifndef saturate
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
vec3 CustomToneMapping( vec3 color ) { return color; }`,_m=`#ifdef USE_TRANSMISSION
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
#endif`,xm=`#ifdef USE_TRANSMISSION
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
#endif`,vm=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
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
#endif`,Mm=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
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
#endif`,Sm=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
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
#endif`,ym=`#if defined( USE_ENVMAP ) || defined( DISTANCE ) || defined ( USE_SHADOWMAP ) || defined ( USE_TRANSMISSION ) || NUM_SPOT_LIGHT_COORDS > 0
	vec4 worldPosition = vec4( transformed, 1.0 );
	#ifdef USE_BATCHING
		worldPosition = batchingMatrix * worldPosition;
	#endif
	#ifdef USE_INSTANCING
		worldPosition = instanceMatrix * worldPosition;
	#endif
	worldPosition = modelMatrix * worldPosition;
#endif`;const Em=`varying vec2 vUv;
uniform mat3 uvTransform;
void main() {
	vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	gl_Position = vec4( position.xy, 1.0, 1.0 );
}`,bm=`uniform sampler2D t2D;
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
}`,Tm=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Am=`#ifdef ENVMAP_TYPE_CUBE
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
}`,wm=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Rm=`uniform samplerCube tCube;
uniform float tFlip;
uniform float opacity;
varying vec3 vWorldDirection;
void main() {
	vec4 texColor = textureCube( tCube, vec3( tFlip * vWorldDirection.x, vWorldDirection.yz ) );
	gl_FragColor = texColor;
	gl_FragColor.a *= opacity;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Cm=`#include <common>
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
}`,Pm=`#if DEPTH_PACKING == 3200
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
}`,Lm=`#define DISTANCE
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
}`,Dm=`#define DISTANCE
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
}`,Im=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
}`,Um=`uniform sampler2D tEquirect;
varying vec3 vWorldDirection;
#include <common>
void main() {
	vec3 direction = normalize( vWorldDirection );
	vec2 sampleUV = equirectUv( direction );
	gl_FragColor = texture2D( tEquirect, sampleUV );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Nm=`uniform float scale;
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
}`,Fm=`uniform vec3 diffuse;
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
}`,Om=`#include <common>
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
}`,Bm=`uniform vec3 diffuse;
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
}`,zm=`#define LAMBERT
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
}`,km=`#define LAMBERT
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
}`,Hm=`#define MATCAP
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
}`,Gm=`#define MATCAP
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
}`,Vm=`#define NORMAL
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
}`,Wm=`#define NORMAL
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
}`,Xm=`#define PHONG
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
}`,$m=`#define PHONG
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
}`,Ym=`#define STANDARD
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
}`,qm=`#define STANDARD
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
}`,jm=`#define TOON
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
}`,Km=`#define TOON
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
}`,Zm=`uniform float size;
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
}`,Jm=`uniform vec3 diffuse;
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
}`,Qm=`#include <common>
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
}`,eg=`uniform vec3 color;
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
}`,tg=`uniform float rotation;
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
}`,ng=`uniform vec3 diffuse;
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
}`,Ye={alphahash_fragment:Ef,alphahash_pars_fragment:bf,alphamap_fragment:Tf,alphamap_pars_fragment:Af,alphatest_fragment:wf,alphatest_pars_fragment:Rf,aomap_fragment:Cf,aomap_pars_fragment:Pf,batching_pars_vertex:Lf,batching_vertex:Df,begin_vertex:If,beginnormal_vertex:Uf,bsdfs:Nf,iridescence_fragment:Ff,bumpmap_pars_fragment:Of,clipping_planes_fragment:Bf,clipping_planes_pars_fragment:zf,clipping_planes_pars_vertex:kf,clipping_planes_vertex:Hf,color_fragment:Gf,color_pars_fragment:Vf,color_pars_vertex:Wf,color_vertex:Xf,common:$f,cube_uv_reflection_fragment:Yf,defaultnormal_vertex:qf,displacementmap_pars_vertex:jf,displacementmap_vertex:Kf,emissivemap_fragment:Zf,emissivemap_pars_fragment:Jf,colorspace_fragment:Qf,colorspace_pars_fragment:ep,envmap_fragment:tp,envmap_common_pars_fragment:np,envmap_pars_fragment:ip,envmap_pars_vertex:sp,envmap_physical_pars_fragment:mp,envmap_vertex:rp,fog_vertex:ap,fog_pars_vertex:op,fog_fragment:lp,fog_pars_fragment:cp,gradientmap_pars_fragment:hp,lightmap_pars_fragment:up,lights_lambert_fragment:dp,lights_lambert_pars_fragment:fp,lights_pars_begin:pp,lights_toon_fragment:gp,lights_toon_pars_fragment:_p,lights_phong_fragment:xp,lights_phong_pars_fragment:vp,lights_physical_fragment:Mp,lights_physical_pars_fragment:Sp,lights_fragment_begin:yp,lights_fragment_maps:Ep,lights_fragment_end:bp,lightprobes_pars_fragment:Tp,logdepthbuf_fragment:Ap,logdepthbuf_pars_fragment:wp,logdepthbuf_pars_vertex:Rp,logdepthbuf_vertex:Cp,map_fragment:Pp,map_pars_fragment:Lp,map_particle_fragment:Dp,map_particle_pars_fragment:Ip,metalnessmap_fragment:Up,metalnessmap_pars_fragment:Np,morphinstance_vertex:Fp,morphcolor_vertex:Op,morphnormal_vertex:Bp,morphtarget_pars_vertex:zp,morphtarget_vertex:kp,normal_fragment_begin:Hp,normal_fragment_maps:Gp,normal_pars_fragment:Vp,normal_pars_vertex:Wp,normal_vertex:Xp,normalmap_pars_fragment:$p,clearcoat_normal_fragment_begin:Yp,clearcoat_normal_fragment_maps:qp,clearcoat_pars_fragment:jp,iridescence_pars_fragment:Kp,opaque_fragment:Zp,packing:Jp,premultiplied_alpha_fragment:Qp,project_vertex:em,dithering_fragment:tm,dithering_pars_fragment:nm,roughnessmap_fragment:im,roughnessmap_pars_fragment:sm,shadowmap_pars_fragment:rm,shadowmap_pars_vertex:am,shadowmap_vertex:om,shadowmask_pars_fragment:lm,skinbase_vertex:cm,skinning_pars_vertex:hm,skinning_vertex:um,skinnormal_vertex:dm,specularmap_fragment:fm,specularmap_pars_fragment:pm,tonemapping_fragment:mm,tonemapping_pars_fragment:gm,transmission_fragment:_m,transmission_pars_fragment:xm,uv_pars_fragment:vm,uv_pars_vertex:Mm,uv_vertex:Sm,worldpos_vertex:ym,background_vert:Em,background_frag:bm,backgroundCube_vert:Tm,backgroundCube_frag:Am,cube_vert:wm,cube_frag:Rm,depth_vert:Cm,depth_frag:Pm,distance_vert:Lm,distance_frag:Dm,equirect_vert:Im,equirect_frag:Um,linedashed_vert:Nm,linedashed_frag:Fm,meshbasic_vert:Om,meshbasic_frag:Bm,meshlambert_vert:zm,meshlambert_frag:km,meshmatcap_vert:Hm,meshmatcap_frag:Gm,meshnormal_vert:Vm,meshnormal_frag:Wm,meshphong_vert:Xm,meshphong_frag:$m,meshphysical_vert:Ym,meshphysical_frag:qm,meshtoon_vert:jm,meshtoon_frag:Km,points_vert:Zm,points_frag:Jm,shadow_vert:Qm,shadow_frag:eg,sprite_vert:tg,sprite_frag:ng},ge={common:{diffuse:{value:new ke(16777215)},opacity:{value:1},map:{value:null},mapTransform:{value:new He},alphaMap:{value:null},alphaMapTransform:{value:new He},alphaTest:{value:0}},specularmap:{specularMap:{value:null},specularMapTransform:{value:new He}},envmap:{envMap:{value:null},envMapRotation:{value:new He},reflectivity:{value:1},ior:{value:1.5},refractionRatio:{value:.98},dfgLUT:{value:null}},aomap:{aoMap:{value:null},aoMapIntensity:{value:1},aoMapTransform:{value:new He}},lightmap:{lightMap:{value:null},lightMapIntensity:{value:1},lightMapTransform:{value:new He}},bumpmap:{bumpMap:{value:null},bumpMapTransform:{value:new He},bumpScale:{value:1}},normalmap:{normalMap:{value:null},normalMapTransform:{value:new He},normalScale:{value:new Oe(1,1)}},displacementmap:{displacementMap:{value:null},displacementMapTransform:{value:new He},displacementScale:{value:1},displacementBias:{value:0}},emissivemap:{emissiveMap:{value:null},emissiveMapTransform:{value:new He}},metalnessmap:{metalnessMap:{value:null},metalnessMapTransform:{value:new He}},roughnessmap:{roughnessMap:{value:null},roughnessMapTransform:{value:new He}},gradientmap:{gradientMap:{value:null}},fog:{fogDensity:{value:25e-5},fogNear:{value:1},fogFar:{value:2e3},fogColor:{value:new ke(16777215)}},lights:{ambientLightColor:{value:[]},lightProbe:{value:[]},directionalLights:{value:[],properties:{direction:{},color:{}}},directionalLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},directionalShadowMatrix:{value:[]},spotLights:{value:[],properties:{color:{},position:{},direction:{},distance:{},coneCos:{},penumbraCos:{},decay:{}}},spotLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},spotLightMap:{value:[]},spotLightMatrix:{value:[]},pointLights:{value:[],properties:{color:{},position:{},decay:{},distance:{}}},pointLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{},shadowCameraNear:{},shadowCameraFar:{}}},pointShadowMatrix:{value:[]},hemisphereLights:{value:[],properties:{direction:{},skyColor:{},groundColor:{}}},rectAreaLights:{value:[],properties:{color:{},position:{},width:{},height:{}}},ltc_1:{value:null},ltc_2:{value:null},probesSH:{value:null},probesMin:{value:new N},probesMax:{value:new N},probesResolution:{value:new N}},points:{diffuse:{value:new ke(16777215)},opacity:{value:1},size:{value:1},scale:{value:1},map:{value:null},alphaMap:{value:null},alphaMapTransform:{value:new He},alphaTest:{value:0},uvTransform:{value:new He}},sprite:{diffuse:{value:new ke(16777215)},opacity:{value:1},center:{value:new Oe(.5,.5)},rotation:{value:0},map:{value:null},mapTransform:{value:new He},alphaMap:{value:null},alphaMapTransform:{value:new He},alphaTest:{value:0}}},En={basic:{uniforms:Gt([ge.common,ge.specularmap,ge.envmap,ge.aomap,ge.lightmap,ge.fog]),vertexShader:Ye.meshbasic_vert,fragmentShader:Ye.meshbasic_frag},lambert:{uniforms:Gt([ge.common,ge.specularmap,ge.envmap,ge.aomap,ge.lightmap,ge.emissivemap,ge.bumpmap,ge.normalmap,ge.displacementmap,ge.fog,ge.lights,{emissive:{value:new ke(0)},envMapIntensity:{value:1}}]),vertexShader:Ye.meshlambert_vert,fragmentShader:Ye.meshlambert_frag},phong:{uniforms:Gt([ge.common,ge.specularmap,ge.envmap,ge.aomap,ge.lightmap,ge.emissivemap,ge.bumpmap,ge.normalmap,ge.displacementmap,ge.fog,ge.lights,{emissive:{value:new ke(0)},specular:{value:new ke(1118481)},shininess:{value:30},envMapIntensity:{value:1}}]),vertexShader:Ye.meshphong_vert,fragmentShader:Ye.meshphong_frag},standard:{uniforms:Gt([ge.common,ge.envmap,ge.aomap,ge.lightmap,ge.emissivemap,ge.bumpmap,ge.normalmap,ge.displacementmap,ge.roughnessmap,ge.metalnessmap,ge.fog,ge.lights,{emissive:{value:new ke(0)},roughness:{value:1},metalness:{value:0},envMapIntensity:{value:1}}]),vertexShader:Ye.meshphysical_vert,fragmentShader:Ye.meshphysical_frag},toon:{uniforms:Gt([ge.common,ge.aomap,ge.lightmap,ge.emissivemap,ge.bumpmap,ge.normalmap,ge.displacementmap,ge.gradientmap,ge.fog,ge.lights,{emissive:{value:new ke(0)}}]),vertexShader:Ye.meshtoon_vert,fragmentShader:Ye.meshtoon_frag},matcap:{uniforms:Gt([ge.common,ge.bumpmap,ge.normalmap,ge.displacementmap,ge.fog,{matcap:{value:null}}]),vertexShader:Ye.meshmatcap_vert,fragmentShader:Ye.meshmatcap_frag},points:{uniforms:Gt([ge.points,ge.fog]),vertexShader:Ye.points_vert,fragmentShader:Ye.points_frag},dashed:{uniforms:Gt([ge.common,ge.fog,{scale:{value:1},dashSize:{value:1},totalSize:{value:2}}]),vertexShader:Ye.linedashed_vert,fragmentShader:Ye.linedashed_frag},depth:{uniforms:Gt([ge.common,ge.displacementmap]),vertexShader:Ye.depth_vert,fragmentShader:Ye.depth_frag},normal:{uniforms:Gt([ge.common,ge.bumpmap,ge.normalmap,ge.displacementmap,{opacity:{value:1}}]),vertexShader:Ye.meshnormal_vert,fragmentShader:Ye.meshnormal_frag},sprite:{uniforms:Gt([ge.sprite,ge.fog]),vertexShader:Ye.sprite_vert,fragmentShader:Ye.sprite_frag},background:{uniforms:{uvTransform:{value:new He},t2D:{value:null},backgroundIntensity:{value:1}},vertexShader:Ye.background_vert,fragmentShader:Ye.background_frag},backgroundCube:{uniforms:{envMap:{value:null},backgroundBlurriness:{value:0},backgroundIntensity:{value:1},backgroundRotation:{value:new He}},vertexShader:Ye.backgroundCube_vert,fragmentShader:Ye.backgroundCube_frag},cube:{uniforms:{tCube:{value:null},tFlip:{value:-1},opacity:{value:1}},vertexShader:Ye.cube_vert,fragmentShader:Ye.cube_frag},equirect:{uniforms:{tEquirect:{value:null}},vertexShader:Ye.equirect_vert,fragmentShader:Ye.equirect_frag},distance:{uniforms:Gt([ge.common,ge.displacementmap,{referencePosition:{value:new N},nearDistance:{value:1},farDistance:{value:1e3}}]),vertexShader:Ye.distance_vert,fragmentShader:Ye.distance_frag},shadow:{uniforms:Gt([ge.lights,ge.fog,{color:{value:new ke(0)},opacity:{value:1}}]),vertexShader:Ye.shadow_vert,fragmentShader:Ye.shadow_frag}};En.physical={uniforms:Gt([En.standard.uniforms,{clearcoat:{value:0},clearcoatMap:{value:null},clearcoatMapTransform:{value:new He},clearcoatNormalMap:{value:null},clearcoatNormalMapTransform:{value:new He},clearcoatNormalScale:{value:new Oe(1,1)},clearcoatRoughness:{value:0},clearcoatRoughnessMap:{value:null},clearcoatRoughnessMapTransform:{value:new He},dispersion:{value:0},iridescence:{value:0},iridescenceMap:{value:null},iridescenceMapTransform:{value:new He},iridescenceIOR:{value:1.3},iridescenceThicknessMinimum:{value:100},iridescenceThicknessMaximum:{value:400},iridescenceThicknessMap:{value:null},iridescenceThicknessMapTransform:{value:new He},sheen:{value:0},sheenColor:{value:new ke(0)},sheenColorMap:{value:null},sheenColorMapTransform:{value:new He},sheenRoughness:{value:1},sheenRoughnessMap:{value:null},sheenRoughnessMapTransform:{value:new He},transmission:{value:0},transmissionMap:{value:null},transmissionMapTransform:{value:new He},transmissionSamplerSize:{value:new Oe},transmissionSamplerMap:{value:null},thickness:{value:0},thicknessMap:{value:null},thicknessMapTransform:{value:new He},attenuationDistance:{value:0},attenuationColor:{value:new ke(0)},specularColor:{value:new ke(1,1,1)},specularColorMap:{value:null},specularColorMapTransform:{value:new He},specularIntensity:{value:1},specularIntensityMap:{value:null},specularIntensityMapTransform:{value:new He},anisotropyVector:{value:new Oe},anisotropyMap:{value:null},anisotropyMapTransform:{value:new He}}]),vertexShader:Ye.meshphysical_vert,fragmentShader:Ye.meshphysical_frag};const Cr={r:0,b:0,g:0},ig=new gt,Kh=new He;Kh.set(-1,0,0,0,1,0,0,0,1);function sg(n,e,t,i,s,r){const a=new ke(0);let o=s===!0?0:1,c,l,u=null,g=0,h=null;function m(y){let A=y.isScene===!0?y.background:null;if(A&&A.isTexture){const T=y.backgroundBlurriness>0;A=e.get(A,T)}return A}function _(y){let A=!1;const T=m(y);T===null?p(a,o):T&&T.isColor&&(p(T,1),A=!0);const L=n.xr.getEnvironmentBlendMode();L==="additive"?t.buffers.color.setClear(0,0,0,1,r):L==="alpha-blend"&&t.buffers.color.setClear(0,0,0,0,r),(n.autoClear||A)&&(t.buffers.depth.setTest(!0),t.buffers.depth.setMask(!0),t.buffers.color.setMask(!0),n.clear(n.autoClearColor,n.autoClearDepth,n.autoClearStencil))}function S(y,A){const T=m(A);T&&(T.isCubeTexture||T.mapping===ga)?(l===void 0&&(l=new Xn(new Ks(1,1,1),new gn({name:"BackgroundCubeMaterial",uniforms:_s(En.backgroundCube.uniforms),vertexShader:En.backgroundCube.vertexShader,fragmentShader:En.backgroundCube.fragmentShader,side:Kt,depthTest:!1,depthWrite:!1,fog:!1,allowOverride:!1})),l.geometry.deleteAttribute("normal"),l.geometry.deleteAttribute("uv"),l.onBeforeRender=function(L,b,I){this.matrixWorld.copyPosition(I.matrixWorld)},Object.defineProperty(l.material,"envMap",{get:function(){return this.uniforms.envMap.value}}),i.update(l)),l.material.uniforms.envMap.value=T,l.material.uniforms.backgroundBlurriness.value=A.backgroundBlurriness,l.material.uniforms.backgroundIntensity.value=A.backgroundIntensity,l.material.uniforms.backgroundRotation.value.setFromMatrix4(ig.makeRotationFromEuler(A.backgroundRotation)).transpose(),T.isCubeTexture&&T.isRenderTargetTexture===!1&&l.material.uniforms.backgroundRotation.value.premultiply(Kh),l.material.toneMapped=Ze.getTransfer(T.colorSpace)!==at,(u!==T||g!==T.version||h!==n.toneMapping)&&(l.material.needsUpdate=!0,u=T,g=T.version,h=n.toneMapping),l.layers.enableAll(),y.unshift(l,l.geometry,l.material,0,0,null)):T&&T.isTexture&&(c===void 0&&(c=new Xn(new xa(2,2),new gn({name:"BackgroundMaterial",uniforms:_s(En.background.uniforms),vertexShader:En.background.vertexShader,fragmentShader:En.background.fragmentShader,side:ci,depthTest:!1,depthWrite:!1,fog:!1,allowOverride:!1})),c.geometry.deleteAttribute("normal"),Object.defineProperty(c.material,"map",{get:function(){return this.uniforms.t2D.value}}),i.update(c)),c.material.uniforms.t2D.value=T,c.material.uniforms.backgroundIntensity.value=A.backgroundIntensity,c.material.toneMapped=Ze.getTransfer(T.colorSpace)!==at,T.matrixAutoUpdate===!0&&T.updateMatrix(),c.material.uniforms.uvTransform.value.copy(T.matrix),(u!==T||g!==T.version||h!==n.toneMapping)&&(c.material.needsUpdate=!0,u=T,g=T.version,h=n.toneMapping),c.layers.enableAll(),y.unshift(c,c.geometry,c.material,0,0,null))}function p(y,A){y.getRGB(Cr,Xh(n)),t.buffers.color.setClear(Cr.r,Cr.g,Cr.b,A,r)}function d(){l!==void 0&&(l.geometry.dispose(),l.material.dispose(),l=void 0),c!==void 0&&(c.geometry.dispose(),c.material.dispose(),c=void 0)}return{getClearColor:function(){return a},setClearColor:function(y,A=1){a.set(y),o=A,p(a,o)},getClearAlpha:function(){return o},setClearAlpha:function(y){o=y,p(a,o)},render:_,addToRenderList:S,dispose:d}}function rg(n,e){const t=n.getParameter(n.MAX_VERTEX_ATTRIBS),i={},s=h(null);let r=s,a=!1;function o(C,z,Z,J,O){let W=!1;const $=g(C,J,Z,z);r!==$&&(r=$,l(r.object)),W=m(C,J,Z,O),W&&_(C,J,Z,O),O!==null&&e.update(O,n.ELEMENT_ARRAY_BUFFER),(W||a)&&(a=!1,T(C,z,Z,J),O!==null&&n.bindBuffer(n.ELEMENT_ARRAY_BUFFER,e.get(O).buffer))}function c(){return n.createVertexArray()}function l(C){return n.bindVertexArray(C)}function u(C){return n.deleteVertexArray(C)}function g(C,z,Z,J){const O=J.wireframe===!0;let W=i[z.id];W===void 0&&(W={},i[z.id]=W);const $=C.isInstancedMesh===!0?C.id:0;let se=W[$];se===void 0&&(se={},W[$]=se);let oe=se[Z.id];oe===void 0&&(oe={},se[Z.id]=oe);let he=oe[O];return he===void 0&&(he=h(c()),oe[O]=he),he}function h(C){const z=[],Z=[],J=[];for(let O=0;O<t;O++)z[O]=0,Z[O]=0,J[O]=0;return{geometry:null,program:null,wireframe:!1,newAttributes:z,enabledAttributes:Z,attributeDivisors:J,object:C,attributes:{},index:null}}function m(C,z,Z,J){const O=r.attributes,W=z.attributes;let $=0;const se=Z.getAttributes();for(const oe in se)if(se[oe].location>=0){const Ee=O[oe];let Pe=W[oe];if(Pe===void 0&&(oe==="instanceMatrix"&&C.instanceMatrix&&(Pe=C.instanceMatrix),oe==="instanceColor"&&C.instanceColor&&(Pe=C.instanceColor)),Ee===void 0||Ee.attribute!==Pe||Pe&&Ee.data!==Pe.data)return!0;$++}return r.attributesNum!==$||r.index!==J}function _(C,z,Z,J){const O={},W=z.attributes;let $=0;const se=Z.getAttributes();for(const oe in se)if(se[oe].location>=0){let Ee=W[oe];Ee===void 0&&(oe==="instanceMatrix"&&C.instanceMatrix&&(Ee=C.instanceMatrix),oe==="instanceColor"&&C.instanceColor&&(Ee=C.instanceColor));const Pe={};Pe.attribute=Ee,Ee&&Ee.data&&(Pe.data=Ee.data),O[oe]=Pe,$++}r.attributes=O,r.attributesNum=$,r.index=J}function S(){const C=r.newAttributes;for(let z=0,Z=C.length;z<Z;z++)C[z]=0}function p(C){d(C,0)}function d(C,z){const Z=r.newAttributes,J=r.enabledAttributes,O=r.attributeDivisors;Z[C]=1,J[C]===0&&(n.enableVertexAttribArray(C),J[C]=1),O[C]!==z&&(n.vertexAttribDivisor(C,z),O[C]=z)}function y(){const C=r.newAttributes,z=r.enabledAttributes;for(let Z=0,J=z.length;Z<J;Z++)z[Z]!==C[Z]&&(n.disableVertexAttribArray(Z),z[Z]=0)}function A(C,z,Z,J,O,W,$){$===!0?n.vertexAttribIPointer(C,z,Z,O,W):n.vertexAttribPointer(C,z,Z,J,O,W)}function T(C,z,Z,J){S();const O=J.attributes,W=Z.getAttributes(),$=z.defaultAttributeValues;for(const se in W){const oe=W[se];if(oe.location>=0){let he=O[se];if(he===void 0&&(se==="instanceMatrix"&&C.instanceMatrix&&(he=C.instanceMatrix),se==="instanceColor"&&C.instanceColor&&(he=C.instanceColor)),he!==void 0){const Ee=he.normalized,Pe=he.itemSize,Ge=e.get(he);if(Ge===void 0)continue;const Je=Ge.buffer,R=Ge.type,U=Ge.bytesPerElement,ie=R===n.INT||R===n.UNSIGNED_INT||he.gpuType===gl;if(he.isInterleavedBufferAttribute){const re=he.data,Me=re.stride,Se=he.offset;if(re.isInstancedInterleavedBuffer){for(let Ce=0;Ce<oe.locationSize;Ce++)d(oe.location+Ce,re.meshPerAttribute);C.isInstancedMesh!==!0&&J._maxInstanceCount===void 0&&(J._maxInstanceCount=re.meshPerAttribute*re.count)}else for(let Ce=0;Ce<oe.locationSize;Ce++)p(oe.location+Ce);n.bindBuffer(n.ARRAY_BUFFER,Je);for(let Ce=0;Ce<oe.locationSize;Ce++)A(oe.location+Ce,Pe/oe.locationSize,R,Ee,Me*U,(Se+Pe/oe.locationSize*Ce)*U,ie)}else{if(he.isInstancedBufferAttribute){for(let re=0;re<oe.locationSize;re++)d(oe.location+re,he.meshPerAttribute);C.isInstancedMesh!==!0&&J._maxInstanceCount===void 0&&(J._maxInstanceCount=he.meshPerAttribute*he.count)}else for(let re=0;re<oe.locationSize;re++)p(oe.location+re);n.bindBuffer(n.ARRAY_BUFFER,Je);for(let re=0;re<oe.locationSize;re++)A(oe.location+re,Pe/oe.locationSize,R,Ee,Pe*U,Pe/oe.locationSize*re*U,ie)}}else if($!==void 0){const Ee=$[se];if(Ee!==void 0)switch(Ee.length){case 2:n.vertexAttrib2fv(oe.location,Ee);break;case 3:n.vertexAttrib3fv(oe.location,Ee);break;case 4:n.vertexAttrib4fv(oe.location,Ee);break;default:n.vertexAttrib1fv(oe.location,Ee)}}}}y()}function L(){w();for(const C in i){const z=i[C];for(const Z in z){const J=z[Z];for(const O in J){const W=J[O];for(const $ in W)u(W[$].object),delete W[$];delete J[O]}}delete i[C]}}function b(C){if(i[C.id]===void 0)return;const z=i[C.id];for(const Z in z){const J=z[Z];for(const O in J){const W=J[O];for(const $ in W)u(W[$].object),delete W[$];delete J[O]}}delete i[C.id]}function I(C){for(const z in i){const Z=i[z];for(const J in Z){const O=Z[J];if(O[C.id]===void 0)continue;const W=O[C.id];for(const $ in W)u(W[$].object),delete W[$];delete O[C.id]}}}function M(C){for(const z in i){const Z=i[z],J=C.isInstancedMesh===!0?C.id:0,O=Z[J];if(O!==void 0){for(const W in O){const $=O[W];for(const se in $)u($[se].object),delete $[se];delete O[W]}delete Z[J],Object.keys(Z).length===0&&delete i[z]}}}function w(){B(),a=!0,r!==s&&(r=s,l(r.object))}function B(){s.geometry=null,s.program=null,s.wireframe=!1}return{setup:o,reset:w,resetDefaultState:B,dispose:L,releaseStatesOfGeometry:b,releaseStatesOfObject:M,releaseStatesOfProgram:I,initAttributes:S,enableAttribute:p,disableUnusedAttributes:y}}function ag(n,e,t){let i;function s(c){i=c}function r(c,l){n.drawArrays(i,c,l),t.update(l,i,1)}function a(c,l,u){u!==0&&(n.drawArraysInstanced(i,c,l,u),t.update(l,i,u))}function o(c,l,u){if(u===0)return;e.get("WEBGL_multi_draw").multiDrawArraysWEBGL(i,c,0,l,0,u);let h=0;for(let m=0;m<u;m++)h+=l[m];t.update(h,i,1)}this.setMode=s,this.render=r,this.renderInstances=a,this.renderMultiDraw=o}function og(n,e,t,i){let s;function r(){if(s!==void 0)return s;if(e.has("EXT_texture_filter_anisotropic")===!0){const I=e.get("EXT_texture_filter_anisotropic");s=n.getParameter(I.MAX_TEXTURE_MAX_ANISOTROPY_EXT)}else s=0;return s}function a(I){return!(I!==pn&&i.convert(I)!==n.getParameter(n.IMPLEMENTATION_COLOR_READ_FORMAT))}function o(I){const M=I===Vn&&(e.has("EXT_color_buffer_half_float")||e.has("EXT_color_buffer_float"));return!(I!==an&&i.convert(I)!==n.getParameter(n.IMPLEMENTATION_COLOR_READ_TYPE)&&I!==An&&!M)}function c(I){if(I==="highp"){if(n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.HIGH_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.HIGH_FLOAT).precision>0)return"highp";I="mediump"}return I==="mediump"&&n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.MEDIUM_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.MEDIUM_FLOAT).precision>0?"mediump":"lowp"}let l=t.precision!==void 0?t.precision:"highp";const u=c(l);u!==l&&(Ne("WebGLRenderer:",l,"not supported, using",u,"instead."),l=u);const g=t.logarithmicDepthBuffer===!0,h=t.reversedDepthBuffer===!0&&e.has("EXT_clip_control");t.reversedDepthBuffer===!0&&h===!1&&Ne("WebGLRenderer: Unable to use reversed depth buffer due to missing EXT_clip_control extension. Fallback to default depth buffer.");const m=n.getParameter(n.MAX_TEXTURE_IMAGE_UNITS),_=n.getParameter(n.MAX_VERTEX_TEXTURE_IMAGE_UNITS),S=n.getParameter(n.MAX_TEXTURE_SIZE),p=n.getParameter(n.MAX_CUBE_MAP_TEXTURE_SIZE),d=n.getParameter(n.MAX_VERTEX_ATTRIBS),y=n.getParameter(n.MAX_VERTEX_UNIFORM_VECTORS),A=n.getParameter(n.MAX_VARYING_VECTORS),T=n.getParameter(n.MAX_FRAGMENT_UNIFORM_VECTORS),L=n.getParameter(n.MAX_SAMPLES),b=n.getParameter(n.SAMPLES);return{isWebGL2:!0,getMaxAnisotropy:r,getMaxPrecision:c,textureFormatReadable:a,textureTypeReadable:o,precision:l,logarithmicDepthBuffer:g,reversedDepthBuffer:h,maxTextures:m,maxVertexTextures:_,maxTextureSize:S,maxCubemapSize:p,maxAttributes:d,maxVertexUniforms:y,maxVaryings:A,maxFragmentUniforms:T,maxSamples:L,samples:b}}function lg(n){const e=this;let t=null,i=0,s=!1,r=!1;const a=new ii,o=new He,c={value:null,needsUpdate:!1};this.uniform=c,this.numPlanes=0,this.numIntersection=0,this.init=function(g,h){const m=g.length!==0||h||i!==0||s;return s=h,i=g.length,m},this.beginShadows=function(){r=!0,u(null)},this.endShadows=function(){r=!1},this.setGlobalState=function(g,h){t=u(g,h,0)},this.setState=function(g,h,m){const _=g.clippingPlanes,S=g.clipIntersection,p=g.clipShadows,d=n.get(g);if(!s||_===null||_.length===0||r&&!p)r?u(null):l();else{const y=r?0:i,A=y*4;let T=d.clippingState||null;c.value=T,T=u(_,h,A,m);for(let L=0;L!==A;++L)T[L]=t[L];d.clippingState=T,this.numIntersection=S?this.numPlanes:0,this.numPlanes+=y}};function l(){c.value!==t&&(c.value=t,c.needsUpdate=i>0),e.numPlanes=i,e.numIntersection=0}function u(g,h,m,_){const S=g!==null?g.length:0;let p=null;if(S!==0){if(p=c.value,_!==!0||p===null){const d=m+S*4,y=h.matrixWorldInverse;o.getNormalMatrix(y),(p===null||p.length<d)&&(p=new Float32Array(d));for(let A=0,T=m;A!==S;++A,T+=4)a.copy(g[A]).applyMatrix4(y,o),a.normal.toArray(p,T),p[T+3]=a.constant}c.value=p,c.needsUpdate=!0}return e.numPlanes=S,e.numIntersection=0,p}}const ri=4,Uc=[.125,.215,.35,.446,.526,.582],yi=20,cg=256,Rs=new Yh,Nc=new ke;let Qa=null,eo=0,to=0,no=!1;const hg=new N;class Fc{constructor(e){this._renderer=e,this._pingPongRenderTarget=null,this._lodMax=0,this._cubeSize=0,this._sizeLods=[],this._sigmas=[],this._lodMeshes=[],this._backgroundBox=null,this._cubemapMaterial=null,this._equirectMaterial=null,this._blurMaterial=null,this._ggxMaterial=null}fromScene(e,t=0,i=.1,s=100,r={}){const{size:a=256,position:o=hg}=r;Qa=this._renderer.getRenderTarget(),eo=this._renderer.getActiveCubeFace(),to=this._renderer.getActiveMipmapLevel(),no=this._renderer.xr.enabled,this._renderer.xr.enabled=!1,this._setSize(a);const c=this._allocateTargets();return c.depthBuffer=!0,this._sceneToCubeUV(e,i,s,c,o),t>0&&this._blur(c,0,0,t),this._applyPMREM(c),this._cleanup(c),c}fromEquirectangular(e,t=null){return this._fromTexture(e,t)}fromCubemap(e,t=null){return this._fromTexture(e,t)}compileCubemapShader(){this._cubemapMaterial===null&&(this._cubemapMaterial=zc(),this._compileMaterial(this._cubemapMaterial))}compileEquirectangularShader(){this._equirectMaterial===null&&(this._equirectMaterial=Bc(),this._compileMaterial(this._equirectMaterial))}dispose(){this._dispose(),this._cubemapMaterial!==null&&this._cubemapMaterial.dispose(),this._equirectMaterial!==null&&this._equirectMaterial.dispose(),this._backgroundBox!==null&&(this._backgroundBox.geometry.dispose(),this._backgroundBox.material.dispose())}_setSize(e){this._lodMax=Math.floor(Math.log2(e)),this._cubeSize=Math.pow(2,this._lodMax)}_dispose(){this._blurMaterial!==null&&this._blurMaterial.dispose(),this._ggxMaterial!==null&&this._ggxMaterial.dispose(),this._pingPongRenderTarget!==null&&this._pingPongRenderTarget.dispose();for(let e=0;e<this._lodMeshes.length;e++)this._lodMeshes[e].geometry.dispose()}_cleanup(e){this._renderer.setRenderTarget(Qa,eo,to),this._renderer.xr.enabled=no,e.scissorTest=!1,ns(e,0,0,e.width,e.height)}_fromTexture(e,t){e.mapping===Ci||e.mapping===ms?this._setSize(e.image.length===0?16:e.image[0].width||e.image[0].image.width):this._setSize(e.image.width/4),Qa=this._renderer.getRenderTarget(),eo=this._renderer.getActiveCubeFace(),to=this._renderer.getActiveMipmapLevel(),no=this._renderer.xr.enabled,this._renderer.xr.enabled=!1;const i=t||this._allocateTargets();return this._textureToCubeUV(e,i),this._applyPMREM(i),this._cleanup(i),i}_allocateTargets(){const e=3*Math.max(this._cubeSize,112),t=4*this._cubeSize,i={magFilter:Bt,minFilter:Bt,generateMipmaps:!1,type:Vn,format:pn,colorSpace:ia,depthBuffer:!1},s=Oc(e,t,i);if(this._pingPongRenderTarget===null||this._pingPongRenderTarget.width!==e||this._pingPongRenderTarget.height!==t){this._pingPongRenderTarget!==null&&this._dispose(),this._pingPongRenderTarget=Oc(e,t,i);const{_lodMax:r}=this;({lodMeshes:this._lodMeshes,sizeLods:this._sizeLods,sigmas:this._sigmas}=ug(r)),this._blurMaterial=fg(r,e,t),this._ggxMaterial=dg(r,e,t)}return s}_compileMaterial(e){const t=new Xn(new Ut,e);this._renderer.compile(t,Rs)}_sceneToCubeUV(e,t,i,s,r){const c=new rn(90,1,t,i),l=[1,-1,1,1,1,1],u=[1,1,1,-1,-1,-1],g=this._renderer,h=g.autoClear,m=g.toneMapping;g.getClearColor(Nc),g.toneMapping=Cn,g.autoClear=!1,g.state.buffers.depth.getReversed()&&(g.setRenderTarget(s),g.clearDepth(),g.setRenderTarget(null)),this._backgroundBox===null&&(this._backgroundBox=new Xn(new Ks,new zh({name:"PMREM.Background",side:Kt,depthWrite:!1,depthTest:!1})));const S=this._backgroundBox,p=S.material;let d=!1;const y=e.background;y?y.isColor&&(p.color.copy(y),e.background=null,d=!0):(p.color.copy(Nc),d=!0);for(let A=0;A<6;A++){const T=A%3;T===0?(c.up.set(0,l[A],0),c.position.set(r.x,r.y,r.z),c.lookAt(r.x+u[A],r.y,r.z)):T===1?(c.up.set(0,0,l[A]),c.position.set(r.x,r.y,r.z),c.lookAt(r.x,r.y+u[A],r.z)):(c.up.set(0,l[A],0),c.position.set(r.x,r.y,r.z),c.lookAt(r.x,r.y,r.z+u[A]));const L=this._cubeSize;ns(s,T*L,A>2?L:0,L,L),g.setRenderTarget(s),d&&g.render(S,c),g.render(e,c)}g.toneMapping=m,g.autoClear=h,e.background=y}_textureToCubeUV(e,t){const i=this._renderer,s=e.mapping===Ci||e.mapping===ms;s?(this._cubemapMaterial===null&&(this._cubemapMaterial=zc()),this._cubemapMaterial.uniforms.flipEnvMap.value=e.isRenderTargetTexture===!1?-1:1):this._equirectMaterial===null&&(this._equirectMaterial=Bc());const r=s?this._cubemapMaterial:this._equirectMaterial,a=this._lodMeshes[0];a.material=r;const o=r.uniforms;o.envMap.value=e;const c=this._cubeSize;ns(t,0,0,3*c,2*c),i.setRenderTarget(t),i.render(a,Rs)}_applyPMREM(e){const t=this._renderer,i=t.autoClear;t.autoClear=!1;const s=this._lodMeshes.length;for(let r=1;r<s;r++)this._applyGGXFilter(e,r-1,r);t.autoClear=i}_applyGGXFilter(e,t,i){const s=this._renderer,r=this._pingPongRenderTarget,a=this._ggxMaterial,o=this._lodMeshes[i];o.material=a;const c=a.uniforms,l=i/(this._lodMeshes.length-1),u=t/(this._lodMeshes.length-1),g=Math.sqrt(l*l-u*u),h=0+l*1.25,m=g*h,{_lodMax:_}=this,S=this._sizeLods[i],p=3*S*(i>_-ri?i-_+ri:0),d=4*(this._cubeSize-S);c.envMap.value=e.texture,c.roughness.value=m,c.mipInt.value=_-t,ns(r,p,d,3*S,2*S),s.setRenderTarget(r),s.render(o,Rs),c.envMap.value=r.texture,c.roughness.value=0,c.mipInt.value=_-i,ns(e,p,d,3*S,2*S),s.setRenderTarget(e),s.render(o,Rs)}_blur(e,t,i,s,r){const a=this._pingPongRenderTarget;this._halfBlur(e,a,t,i,s,"latitudinal",r),this._halfBlur(a,e,i,i,s,"longitudinal",r)}_halfBlur(e,t,i,s,r,a,o){const c=this._renderer,l=this._blurMaterial;a!=="latitudinal"&&a!=="longitudinal"&&Qe("blur direction must be either latitudinal or longitudinal!");const u=3,g=this._lodMeshes[s];g.material=l;const h=l.uniforms,m=this._sizeLods[i]-1,_=isFinite(r)?Math.PI/(2*m):2*Math.PI/(2*yi-1),S=r/_,p=isFinite(r)?1+Math.floor(u*S):yi;p>yi&&Ne(`sigmaRadians, ${r}, is too large and will clip, as it requested ${p} samples when the maximum is set to ${yi}`);const d=[];let y=0;for(let I=0;I<yi;++I){const M=I/S,w=Math.exp(-M*M/2);d.push(w),I===0?y+=w:I<p&&(y+=2*w)}for(let I=0;I<d.length;I++)d[I]=d[I]/y;h.envMap.value=e.texture,h.samples.value=p,h.weights.value=d,h.latitudinal.value=a==="latitudinal",o&&(h.poleAxis.value=o);const{_lodMax:A}=this;h.dTheta.value=_,h.mipInt.value=A-i;const T=this._sizeLods[s],L=3*T*(s>A-ri?s-A+ri:0),b=4*(this._cubeSize-T);ns(t,L,b,3*T,2*T),c.setRenderTarget(t),c.render(g,Rs)}}function ug(n){const e=[],t=[],i=[];let s=n;const r=n-ri+1+Uc.length;for(let a=0;a<r;a++){const o=Math.pow(2,s);e.push(o);let c=1/o;a>n-ri?c=Uc[a-n+ri-1]:a===0&&(c=0),t.push(c);const l=1/(o-2),u=-l,g=1+l,h=[u,u,g,u,g,g,u,u,g,g,u,g],m=6,_=6,S=3,p=2,d=1,y=new Float32Array(S*_*m),A=new Float32Array(p*_*m),T=new Float32Array(d*_*m);for(let b=0;b<m;b++){const I=b%3*2/3-1,M=b>2?0:-1,w=[I,M,0,I+2/3,M,0,I+2/3,M+1,0,I,M,0,I+2/3,M+1,0,I,M+1,0];y.set(w,S*_*b),A.set(h,p*_*b);const B=[b,b,b,b,b,b];T.set(B,d*_*b)}const L=new Ut;L.setAttribute("position",new At(y,S)),L.setAttribute("uv",new At(A,p)),L.setAttribute("faceIndex",new At(T,d)),i.push(new Xn(L,null)),s>ri&&s--}return{lodMeshes:i,sizeLods:e,sigmas:t}}function Oc(n,e,t){const i=new Pn(n,e,t);return i.texture.mapping=ga,i.texture.name="PMREM.cubeUv",i.scissorTest=!0,i}function ns(n,e,t,i,s){n.viewport.set(e,t,i,s),n.scissor.set(e,t,i,s)}function dg(n,e,t){return new gn({name:"PMREMGGXConvolution",defines:{GGX_SAMPLES:cg,CUBEUV_TEXEL_WIDTH:1/e,CUBEUV_TEXEL_HEIGHT:1/t,CUBEUV_MAX_MIP:`${n}.0`},uniforms:{envMap:{value:null},roughness:{value:0},mipInt:{value:0}},vertexShader:va(),fragmentShader:`

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
		`,blending:Hn,depthTest:!1,depthWrite:!1})}function fg(n,e,t){const i=new Float32Array(yi),s=new N(0,1,0);return new gn({name:"SphericalGaussianBlur",defines:{n:yi,CUBEUV_TEXEL_WIDTH:1/e,CUBEUV_TEXEL_HEIGHT:1/t,CUBEUV_MAX_MIP:`${n}.0`},uniforms:{envMap:{value:null},samples:{value:1},weights:{value:i},latitudinal:{value:!1},dTheta:{value:0},mipInt:{value:0},poleAxis:{value:s}},vertexShader:va(),fragmentShader:`

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
		`,blending:Hn,depthTest:!1,depthWrite:!1})}function Bc(){return new gn({name:"EquirectangularToCubeUV",uniforms:{envMap:{value:null}},vertexShader:va(),fragmentShader:`

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
		`,blending:Hn,depthTest:!1,depthWrite:!1})}function zc(){return new gn({name:"CubemapToCubeUV",uniforms:{envMap:{value:null},flipEnvMap:{value:-1}},vertexShader:va(),fragmentShader:`

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
	`}class Zh extends Pn{constructor(e=1,t={}){super(e,e,t),this.isWebGLCubeRenderTarget=!0;const i={width:e,height:e,depth:1},s=[i,i,i,i,i,i];this.texture=new Gh(s),this._setTextureOptions(t),this.texture.isRenderTargetTexture=!0}fromEquirectangularTexture(e,t){this.texture.type=t.type,this.texture.colorSpace=t.colorSpace,this.texture.generateMipmaps=t.generateMipmaps,this.texture.minFilter=t.minFilter,this.texture.magFilter=t.magFilter;const i={uniforms:{tEquirect:{value:null}},vertexShader:`

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
			`},s=new Ks(5,5,5),r=new gn({name:"CubemapFromEquirect",uniforms:_s(i.uniforms),vertexShader:i.vertexShader,fragmentShader:i.fragmentShader,side:Kt,blending:Hn});r.uniforms.tEquirect.value=t;const a=new Xn(s,r),o=t.minFilter;return t.minFilter===Ti&&(t.minFilter=Bt),new xf(1,10,this).update(e,a),t.minFilter=o,a.geometry.dispose(),a.material.dispose(),this}clear(e,t=!0,i=!0,s=!0){const r=e.getRenderTarget();for(let a=0;a<6;a++)e.setRenderTarget(this,a),e.clear(t,i,s);e.setRenderTarget(r)}}function pg(n){let e=new WeakMap,t=new WeakMap,i=null;function s(h,m=!1){return h==null?null:m?a(h):r(h)}function r(h){if(h&&h.isTexture){const m=h.mapping;if(m===Ta||m===Aa)if(e.has(h)){const _=e.get(h).texture;return o(_,h.mapping)}else{const _=h.image;if(_&&_.height>0){const S=new Zh(_.height);return S.fromEquirectangularTexture(n,h),e.set(h,S),h.addEventListener("dispose",l),o(S.texture,h.mapping)}else return null}}return h}function a(h){if(h&&h.isTexture){const m=h.mapping,_=m===Ta||m===Aa,S=m===Ci||m===ms;if(_||S){let p=t.get(h);const d=p!==void 0?p.texture.pmremVersion:0;if(h.isRenderTargetTexture&&h.pmremVersion!==d)return i===null&&(i=new Fc(n)),p=_?i.fromEquirectangular(h,p):i.fromCubemap(h,p),p.texture.pmremVersion=h.pmremVersion,t.set(h,p),p.texture;if(p!==void 0)return p.texture;{const y=h.image;return _&&y&&y.height>0||S&&y&&c(y)?(i===null&&(i=new Fc(n)),p=_?i.fromEquirectangular(h):i.fromCubemap(h),p.texture.pmremVersion=h.pmremVersion,t.set(h,p),h.addEventListener("dispose",u),p.texture):null}}}return h}function o(h,m){return m===Ta?h.mapping=Ci:m===Aa&&(h.mapping=ms),h}function c(h){let m=0;const _=6;for(let S=0;S<_;S++)h[S]!==void 0&&m++;return m===_}function l(h){const m=h.target;m.removeEventListener("dispose",l);const _=e.get(m);_!==void 0&&(e.delete(m),_.dispose())}function u(h){const m=h.target;m.removeEventListener("dispose",u);const _=t.get(m);_!==void 0&&(t.delete(m),_.dispose())}function g(){e=new WeakMap,t=new WeakMap,i!==null&&(i.dispose(),i=null)}return{get:s,dispose:g}}function mg(n){const e={};function t(i){if(e[i]!==void 0)return e[i];const s=n.getExtension(i);return e[i]=s,s}return{has:function(i){return t(i)!==null},init:function(){t("EXT_color_buffer_float"),t("WEBGL_clip_cull_distance"),t("OES_texture_float_linear"),t("EXT_color_buffer_half_float"),t("WEBGL_multisampled_render_to_texture"),t("WEBGL_render_shared_exponent")},get:function(i){const s=t(i);return s===null&&Zo("WebGLRenderer: "+i+" extension not supported."),s}}}function gg(n,e,t,i){const s={},r=new WeakMap;function a(g){const h=g.target;h.index!==null&&e.remove(h.index);for(const _ in h.attributes)e.remove(h.attributes[_]);h.removeEventListener("dispose",a),delete s[h.id];const m=r.get(h);m&&(e.remove(m),r.delete(h)),i.releaseStatesOfGeometry(h),h.isInstancedBufferGeometry===!0&&delete h._maxInstanceCount,t.memory.geometries--}function o(g,h){return s[h.id]===!0||(h.addEventListener("dispose",a),s[h.id]=!0,t.memory.geometries++),h}function c(g){const h=g.attributes;for(const m in h)e.update(h[m],n.ARRAY_BUFFER)}function l(g){const h=[],m=g.index,_=g.attributes.position;let S=0;if(_===void 0)return;if(m!==null){const y=m.array;S=m.version;for(let A=0,T=y.length;A<T;A+=3){const L=y[A+0],b=y[A+1],I=y[A+2];h.push(L,b,b,I,I,L)}}else{const y=_.array;S=_.version;for(let A=0,T=y.length/3-1;A<T;A+=3){const L=A+0,b=A+1,I=A+2;h.push(L,b,b,I,I,L)}}const p=new(_.count>=65535?Fh:Nh)(h,1);p.version=S;const d=r.get(g);d&&e.remove(d),r.set(g,p)}function u(g){const h=r.get(g);if(h){const m=g.index;m!==null&&h.version<m.version&&l(g)}else l(g);return r.get(g)}return{get:o,update:c,getWireframeAttribute:u}}function _g(n,e,t){let i;function s(g){i=g}let r,a;function o(g){r=g.type,a=g.bytesPerElement}function c(g,h){n.drawElements(i,h,r,g*a),t.update(h,i,1)}function l(g,h,m){m!==0&&(n.drawElementsInstanced(i,h,r,g*a,m),t.update(h,i,m))}function u(g,h,m){if(m===0)return;e.get("WEBGL_multi_draw").multiDrawElementsWEBGL(i,h,0,r,g,0,m);let S=0;for(let p=0;p<m;p++)S+=h[p];t.update(S,i,1)}this.setMode=s,this.setIndex=o,this.render=c,this.renderInstances=l,this.renderMultiDraw=u}function xg(n){const e={geometries:0,textures:0},t={frame:0,calls:0,triangles:0,points:0,lines:0};function i(r,a,o){switch(t.calls++,a){case n.TRIANGLES:t.triangles+=o*(r/3);break;case n.LINES:t.lines+=o*(r/2);break;case n.LINE_STRIP:t.lines+=o*(r-1);break;case n.LINE_LOOP:t.lines+=o*r;break;case n.POINTS:t.points+=o*r;break;default:Qe("WebGLInfo: Unknown draw mode:",a);break}}function s(){t.calls=0,t.triangles=0,t.points=0,t.lines=0}return{memory:e,render:t,programs:null,autoReset:!0,reset:s,update:i}}function vg(n,e,t){const i=new WeakMap,s=new Et;function r(a,o,c){const l=a.morphTargetInfluences,u=o.morphAttributes.position||o.morphAttributes.normal||o.morphAttributes.color,g=u!==void 0?u.length:0;let h=i.get(o);if(h===void 0||h.count!==g){let w=function(){I.dispose(),i.delete(o),o.removeEventListener("dispose",w)};h!==void 0&&h.texture.dispose();const m=o.morphAttributes.position!==void 0,_=o.morphAttributes.normal!==void 0,S=o.morphAttributes.color!==void 0,p=o.morphAttributes.position||[],d=o.morphAttributes.normal||[],y=o.morphAttributes.color||[];let A=0;m===!0&&(A=1),_===!0&&(A=2),S===!0&&(A=3);let T=o.attributes.position.count*A,L=1;T>e.maxTextureSize&&(L=Math.ceil(T/e.maxTextureSize),T=e.maxTextureSize);const b=new Float32Array(T*L*4*g),I=new Dh(b,T,L,g);I.type=An,I.needsUpdate=!0;const M=A*4;for(let B=0;B<g;B++){const C=p[B],z=d[B],Z=y[B],J=T*L*4*B;for(let O=0;O<C.count;O++){const W=O*M;m===!0&&(s.fromBufferAttribute(C,O),b[J+W+0]=s.x,b[J+W+1]=s.y,b[J+W+2]=s.z,b[J+W+3]=0),_===!0&&(s.fromBufferAttribute(z,O),b[J+W+4]=s.x,b[J+W+5]=s.y,b[J+W+6]=s.z,b[J+W+7]=0),S===!0&&(s.fromBufferAttribute(Z,O),b[J+W+8]=s.x,b[J+W+9]=s.y,b[J+W+10]=s.z,b[J+W+11]=Z.itemSize===4?s.w:1)}}h={count:g,texture:I,size:new Oe(T,L)},i.set(o,h),o.addEventListener("dispose",w)}if(a.isInstancedMesh===!0&&a.morphTexture!==null)c.getUniforms().setValue(n,"morphTexture",a.morphTexture,t);else{let m=0;for(let S=0;S<l.length;S++)m+=l[S];const _=o.morphTargetsRelative?1:1-m;c.getUniforms().setValue(n,"morphTargetBaseInfluence",_),c.getUniforms().setValue(n,"morphTargetInfluences",l)}c.getUniforms().setValue(n,"morphTargetsTexture",h.texture,t),c.getUniforms().setValue(n,"morphTargetsTextureSize",h.size)}return{update:r}}function Mg(n,e,t,i,s){let r=new WeakMap;function a(l){const u=s.render.frame,g=l.geometry,h=e.get(l,g);if(r.get(h)!==u&&(e.update(h),r.set(h,u)),l.isInstancedMesh&&(l.hasEventListener("dispose",c)===!1&&l.addEventListener("dispose",c),r.get(l)!==u&&(t.update(l.instanceMatrix,n.ARRAY_BUFFER),l.instanceColor!==null&&t.update(l.instanceColor,n.ARRAY_BUFFER),r.set(l,u))),l.isSkinnedMesh){const m=l.skeleton;r.get(m)!==u&&(m.update(),r.set(m,u))}return h}function o(){r=new WeakMap}function c(l){const u=l.target;u.removeEventListener("dispose",c),i.releaseStatesOfObject(u),t.remove(u.instanceMatrix),u.instanceColor!==null&&t.remove(u.instanceColor)}return{update:a,dispose:o}}const Sg={[gh]:"LINEAR_TONE_MAPPING",[_h]:"REINHARD_TONE_MAPPING",[xh]:"CINEON_TONE_MAPPING",[vh]:"ACES_FILMIC_TONE_MAPPING",[Sh]:"AGX_TONE_MAPPING",[yh]:"NEUTRAL_TONE_MAPPING",[Mh]:"CUSTOM_TONE_MAPPING"};function yg(n,e,t,i,s){const r=new Pn(e,t,{type:n,depthBuffer:i,stencilBuffer:s,depthTexture:i?new gs(e,t):void 0}),a=new Pn(e,t,{type:Vn,depthBuffer:!1,stencilBuffer:!1}),o=new Ut;o.setAttribute("position",new mn([-1,3,0,-1,-1,0,3,-1,0],3)),o.setAttribute("uv",new mn([0,2,0,0,2,0],2));const c=new mf({uniforms:{tDiffuse:{value:null}},vertexShader:`
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
			}`,depthTest:!1,depthWrite:!1}),l=new Xn(o,c),u=new Yh(-1,1,1,-1,0,1);let g=null,h=null,m=!1,_,S=null,p=[],d=!1;this.setSize=function(y,A){r.setSize(y,A),a.setSize(y,A);for(let T=0;T<p.length;T++){const L=p[T];L.setSize&&L.setSize(y,A)}},this.setEffects=function(y){p=y,d=p.length>0&&p[0].isRenderPass===!0;const A=r.width,T=r.height;for(let L=0;L<p.length;L++){const b=p[L];b.setSize&&b.setSize(A,T)}},this.begin=function(y,A){if(m||y.toneMapping===Cn&&p.length===0)return!1;if(S=A,A!==null){const T=A.width,L=A.height;(r.width!==T||r.height!==L)&&this.setSize(T,L)}return d===!1&&y.setRenderTarget(r),_=y.toneMapping,y.toneMapping=Cn,!0},this.hasRenderPass=function(){return d},this.end=function(y,A){y.toneMapping=_,m=!0;let T=r,L=a;for(let b=0;b<p.length;b++){const I=p[b];if(I.enabled!==!1&&(I.render(y,L,T,A),I.needsSwap!==!1)){const M=T;T=L,L=M}}if(g!==y.outputColorSpace||h!==y.toneMapping){g=y.outputColorSpace,h=y.toneMapping,c.defines={},Ze.getTransfer(g)===at&&(c.defines.SRGB_TRANSFER="");const b=Sg[h];b&&(c.defines[b]=""),c.needsUpdate=!0}c.uniforms.tDiffuse.value=T.texture,y.setRenderTarget(S),y.render(l,u),S=null,m=!1},this.isCompositing=function(){return m},this.dispose=function(){r.depthTexture&&r.depthTexture.dispose(),r.dispose(),a.dispose(),o.dispose(),c.dispose()}}const Jh=new zt,el=new gs(1,1),Qh=new Dh,eu=new Gd,tu=new Gh,kc=[],Hc=[],Gc=new Float32Array(16),Vc=new Float32Array(9),Wc=new Float32Array(4);function xs(n,e,t){const i=n[0];if(i<=0||i>0)return n;const s=e*t;let r=kc[s];if(r===void 0&&(r=new Float32Array(s),kc[s]=r),e!==0){i.toArray(r,0);for(let a=1,o=0;a!==e;++a)o+=t,n[a].toArray(r,o)}return r}function Rt(n,e){if(n.length!==e.length)return!1;for(let t=0,i=n.length;t<i;t++)if(n[t]!==e[t])return!1;return!0}function Ct(n,e){for(let t=0,i=e.length;t<i;t++)n[t]=e[t]}function Ma(n,e){let t=Hc[e];t===void 0&&(t=new Int32Array(e),Hc[e]=t);for(let i=0;i!==e;++i)t[i]=n.allocateTextureUnit();return t}function Eg(n,e){const t=this.cache;t[0]!==e&&(n.uniform1f(this.addr,e),t[0]=e)}function bg(n,e){const t=this.cache;if(e.x!==void 0)(t[0]!==e.x||t[1]!==e.y)&&(n.uniform2f(this.addr,e.x,e.y),t[0]=e.x,t[1]=e.y);else{if(Rt(t,e))return;n.uniform2fv(this.addr,e),Ct(t,e)}}function Tg(n,e){const t=this.cache;if(e.x!==void 0)(t[0]!==e.x||t[1]!==e.y||t[2]!==e.z)&&(n.uniform3f(this.addr,e.x,e.y,e.z),t[0]=e.x,t[1]=e.y,t[2]=e.z);else if(e.r!==void 0)(t[0]!==e.r||t[1]!==e.g||t[2]!==e.b)&&(n.uniform3f(this.addr,e.r,e.g,e.b),t[0]=e.r,t[1]=e.g,t[2]=e.b);else{if(Rt(t,e))return;n.uniform3fv(this.addr,e),Ct(t,e)}}function Ag(n,e){const t=this.cache;if(e.x!==void 0)(t[0]!==e.x||t[1]!==e.y||t[2]!==e.z||t[3]!==e.w)&&(n.uniform4f(this.addr,e.x,e.y,e.z,e.w),t[0]=e.x,t[1]=e.y,t[2]=e.z,t[3]=e.w);else{if(Rt(t,e))return;n.uniform4fv(this.addr,e),Ct(t,e)}}function wg(n,e){const t=this.cache,i=e.elements;if(i===void 0){if(Rt(t,e))return;n.uniformMatrix2fv(this.addr,!1,e),Ct(t,e)}else{if(Rt(t,i))return;Wc.set(i),n.uniformMatrix2fv(this.addr,!1,Wc),Ct(t,i)}}function Rg(n,e){const t=this.cache,i=e.elements;if(i===void 0){if(Rt(t,e))return;n.uniformMatrix3fv(this.addr,!1,e),Ct(t,e)}else{if(Rt(t,i))return;Vc.set(i),n.uniformMatrix3fv(this.addr,!1,Vc),Ct(t,i)}}function Cg(n,e){const t=this.cache,i=e.elements;if(i===void 0){if(Rt(t,e))return;n.uniformMatrix4fv(this.addr,!1,e),Ct(t,e)}else{if(Rt(t,i))return;Gc.set(i),n.uniformMatrix4fv(this.addr,!1,Gc),Ct(t,i)}}function Pg(n,e){const t=this.cache;t[0]!==e&&(n.uniform1i(this.addr,e),t[0]=e)}function Lg(n,e){const t=this.cache;if(e.x!==void 0)(t[0]!==e.x||t[1]!==e.y)&&(n.uniform2i(this.addr,e.x,e.y),t[0]=e.x,t[1]=e.y);else{if(Rt(t,e))return;n.uniform2iv(this.addr,e),Ct(t,e)}}function Dg(n,e){const t=this.cache;if(e.x!==void 0)(t[0]!==e.x||t[1]!==e.y||t[2]!==e.z)&&(n.uniform3i(this.addr,e.x,e.y,e.z),t[0]=e.x,t[1]=e.y,t[2]=e.z);else{if(Rt(t,e))return;n.uniform3iv(this.addr,e),Ct(t,e)}}function Ig(n,e){const t=this.cache;if(e.x!==void 0)(t[0]!==e.x||t[1]!==e.y||t[2]!==e.z||t[3]!==e.w)&&(n.uniform4i(this.addr,e.x,e.y,e.z,e.w),t[0]=e.x,t[1]=e.y,t[2]=e.z,t[3]=e.w);else{if(Rt(t,e))return;n.uniform4iv(this.addr,e),Ct(t,e)}}function Ug(n,e){const t=this.cache;t[0]!==e&&(n.uniform1ui(this.addr,e),t[0]=e)}function Ng(n,e){const t=this.cache;if(e.x!==void 0)(t[0]!==e.x||t[1]!==e.y)&&(n.uniform2ui(this.addr,e.x,e.y),t[0]=e.x,t[1]=e.y);else{if(Rt(t,e))return;n.uniform2uiv(this.addr,e),Ct(t,e)}}function Fg(n,e){const t=this.cache;if(e.x!==void 0)(t[0]!==e.x||t[1]!==e.y||t[2]!==e.z)&&(n.uniform3ui(this.addr,e.x,e.y,e.z),t[0]=e.x,t[1]=e.y,t[2]=e.z);else{if(Rt(t,e))return;n.uniform3uiv(this.addr,e),Ct(t,e)}}function Og(n,e){const t=this.cache;if(e.x!==void 0)(t[0]!==e.x||t[1]!==e.y||t[2]!==e.z||t[3]!==e.w)&&(n.uniform4ui(this.addr,e.x,e.y,e.z,e.w),t[0]=e.x,t[1]=e.y,t[2]=e.z,t[3]=e.w);else{if(Rt(t,e))return;n.uniform4uiv(this.addr,e),Ct(t,e)}}function Bg(n,e,t){const i=this.cache,s=t.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s);let r;this.type===n.SAMPLER_2D_SHADOW?(el.compareFunction=t.isReversedDepthBuffer()?El:yl,r=el):r=Jh,t.setTexture2D(e||r,s)}function zg(n,e,t){const i=this.cache,s=t.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s),t.setTexture3D(e||eu,s)}function kg(n,e,t){const i=this.cache,s=t.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s),t.setTextureCube(e||tu,s)}function Hg(n,e,t){const i=this.cache,s=t.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s),t.setTexture2DArray(e||Qh,s)}function Gg(n){switch(n){case 5126:return Eg;case 35664:return bg;case 35665:return Tg;case 35666:return Ag;case 35674:return wg;case 35675:return Rg;case 35676:return Cg;case 5124:case 35670:return Pg;case 35667:case 35671:return Lg;case 35668:case 35672:return Dg;case 35669:case 35673:return Ig;case 5125:return Ug;case 36294:return Ng;case 36295:return Fg;case 36296:return Og;case 35678:case 36198:case 36298:case 36306:case 35682:return Bg;case 35679:case 36299:case 36307:return zg;case 35680:case 36300:case 36308:case 36293:return kg;case 36289:case 36303:case 36311:case 36292:return Hg}}function Vg(n,e){n.uniform1fv(this.addr,e)}function Wg(n,e){const t=xs(e,this.size,2);n.uniform2fv(this.addr,t)}function Xg(n,e){const t=xs(e,this.size,3);n.uniform3fv(this.addr,t)}function $g(n,e){const t=xs(e,this.size,4);n.uniform4fv(this.addr,t)}function Yg(n,e){const t=xs(e,this.size,4);n.uniformMatrix2fv(this.addr,!1,t)}function qg(n,e){const t=xs(e,this.size,9);n.uniformMatrix3fv(this.addr,!1,t)}function jg(n,e){const t=xs(e,this.size,16);n.uniformMatrix4fv(this.addr,!1,t)}function Kg(n,e){n.uniform1iv(this.addr,e)}function Zg(n,e){n.uniform2iv(this.addr,e)}function Jg(n,e){n.uniform3iv(this.addr,e)}function Qg(n,e){n.uniform4iv(this.addr,e)}function e_(n,e){n.uniform1uiv(this.addr,e)}function t_(n,e){n.uniform2uiv(this.addr,e)}function n_(n,e){n.uniform3uiv(this.addr,e)}function i_(n,e){n.uniform4uiv(this.addr,e)}function s_(n,e,t){const i=this.cache,s=e.length,r=Ma(t,s);Rt(i,r)||(n.uniform1iv(this.addr,r),Ct(i,r));let a;this.type===n.SAMPLER_2D_SHADOW?a=el:a=Jh;for(let o=0;o!==s;++o)t.setTexture2D(e[o]||a,r[o])}function r_(n,e,t){const i=this.cache,s=e.length,r=Ma(t,s);Rt(i,r)||(n.uniform1iv(this.addr,r),Ct(i,r));for(let a=0;a!==s;++a)t.setTexture3D(e[a]||eu,r[a])}function a_(n,e,t){const i=this.cache,s=e.length,r=Ma(t,s);Rt(i,r)||(n.uniform1iv(this.addr,r),Ct(i,r));for(let a=0;a!==s;++a)t.setTextureCube(e[a]||tu,r[a])}function o_(n,e,t){const i=this.cache,s=e.length,r=Ma(t,s);Rt(i,r)||(n.uniform1iv(this.addr,r),Ct(i,r));for(let a=0;a!==s;++a)t.setTexture2DArray(e[a]||Qh,r[a])}function l_(n){switch(n){case 5126:return Vg;case 35664:return Wg;case 35665:return Xg;case 35666:return $g;case 35674:return Yg;case 35675:return qg;case 35676:return jg;case 5124:case 35670:return Kg;case 35667:case 35671:return Zg;case 35668:case 35672:return Jg;case 35669:case 35673:return Qg;case 5125:return e_;case 36294:return t_;case 36295:return n_;case 36296:return i_;case 35678:case 36198:case 36298:case 36306:case 35682:return s_;case 35679:case 36299:case 36307:return r_;case 35680:case 36300:case 36308:case 36293:return a_;case 36289:case 36303:case 36311:case 36292:return o_}}class c_{constructor(e,t,i){this.id=e,this.addr=i,this.cache=[],this.type=t.type,this.setValue=Gg(t.type)}}class h_{constructor(e,t,i){this.id=e,this.addr=i,this.cache=[],this.type=t.type,this.size=t.size,this.setValue=l_(t.type)}}class u_{constructor(e){this.id=e,this.seq=[],this.map={}}setValue(e,t,i){const s=this.seq;for(let r=0,a=s.length;r!==a;++r){const o=s[r];o.setValue(e,t[o.id],i)}}}const io=/(\w+)(\])?(\[|\.)?/g;function Xc(n,e){n.seq.push(e),n.map[e.id]=e}function d_(n,e,t){const i=n.name,s=i.length;for(io.lastIndex=0;;){const r=io.exec(i),a=io.lastIndex;let o=r[1];const c=r[2]==="]",l=r[3];if(c&&(o=o|0),l===void 0||l==="["&&a+2===s){Xc(t,l===void 0?new c_(o,n,e):new h_(o,n,e));break}else{let g=t.map[o];g===void 0&&(g=new u_(o),Xc(t,g)),t=g}}}class Xr{constructor(e,t){this.seq=[],this.map={};const i=e.getProgramParameter(t,e.ACTIVE_UNIFORMS);for(let a=0;a<i;++a){const o=e.getActiveUniform(t,a),c=e.getUniformLocation(t,o.name);d_(o,c,this)}const s=[],r=[];for(const a of this.seq)a.type===e.SAMPLER_2D_SHADOW||a.type===e.SAMPLER_CUBE_SHADOW||a.type===e.SAMPLER_2D_ARRAY_SHADOW?s.push(a):r.push(a);s.length>0&&(this.seq=s.concat(r))}setValue(e,t,i,s){const r=this.map[t];r!==void 0&&r.setValue(e,i,s)}setOptional(e,t,i){const s=t[i];s!==void 0&&this.setValue(e,i,s)}static upload(e,t,i,s){for(let r=0,a=t.length;r!==a;++r){const o=t[r],c=i[o.id];c.needsUpdate!==!1&&o.setValue(e,c.value,s)}}static seqWithValue(e,t){const i=[];for(let s=0,r=e.length;s!==r;++s){const a=e[s];a.id in t&&i.push(a)}return i}}function $c(n,e,t){const i=n.createShader(e);return n.shaderSource(i,t),n.compileShader(i),i}const f_=37297;let p_=0;function m_(n,e){const t=n.split(`
`),i=[],s=Math.max(e-6,0),r=Math.min(e+6,t.length);for(let a=s;a<r;a++){const o=a+1;i.push(`${o===e?">":" "} ${o}: ${t[a]}`)}return i.join(`
`)}const Yc=new He;function g_(n){Ze._getMatrix(Yc,Ze.workingColorSpace,n);const e=`mat3( ${Yc.elements.map(t=>t.toFixed(4))} )`;switch(Ze.getTransfer(n)){case sa:return[e,"LinearTransferOETF"];case at:return[e,"sRGBTransferOETF"];default:return Ne("WebGLProgram: Unsupported color space: ",n),[e,"LinearTransferOETF"]}}function qc(n,e,t){const i=n.getShaderParameter(e,n.COMPILE_STATUS),r=(n.getShaderInfoLog(e)||"").trim();if(i&&r==="")return"";const a=/ERROR: 0:(\d+)/.exec(r);if(a){const o=parseInt(a[1]);return t.toUpperCase()+`

`+r+`

`+m_(n.getShaderSource(e),o)}else return r}function __(n,e){const t=g_(e);return[`vec4 ${n}( vec4 value ) {`,`	return ${t[1]}( vec4( value.rgb * ${t[0]}, value.a ) );`,"}"].join(`
`)}const x_={[gh]:"Linear",[_h]:"Reinhard",[xh]:"Cineon",[vh]:"ACESFilmic",[Sh]:"AgX",[yh]:"Neutral",[Mh]:"Custom"};function v_(n,e){const t=x_[e];return t===void 0?(Ne("WebGLProgram: Unsupported toneMapping:",e),"vec3 "+n+"( vec3 color ) { return LinearToneMapping( color ); }"):"vec3 "+n+"( vec3 color ) { return "+t+"ToneMapping( color ); }"}const Pr=new N;function M_(){Ze.getLuminanceCoefficients(Pr);const n=Pr.x.toFixed(4),e=Pr.y.toFixed(4),t=Pr.z.toFixed(4);return["float luminance( const in vec3 rgb ) {",`	const vec3 weights = vec3( ${n}, ${e}, ${t} );`,"	return dot( weights, rgb );","}"].join(`
`)}function S_(n){return[n.extensionClipCullDistance?"#extension GL_ANGLE_clip_cull_distance : require":"",n.extensionMultiDraw?"#extension GL_ANGLE_multi_draw : require":""].filter(Ls).join(`
`)}function y_(n){const e=[];for(const t in n){const i=n[t];i!==!1&&e.push("#define "+t+" "+i)}return e.join(`
`)}function E_(n,e){const t={},i=n.getProgramParameter(e,n.ACTIVE_ATTRIBUTES);for(let s=0;s<i;s++){const r=n.getActiveAttrib(e,s),a=r.name;let o=1;r.type===n.FLOAT_MAT2&&(o=2),r.type===n.FLOAT_MAT3&&(o=3),r.type===n.FLOAT_MAT4&&(o=4),t[a]={type:r.type,location:n.getAttribLocation(e,a),locationSize:o}}return t}function Ls(n){return n!==""}function jc(n,e){const t=e.numSpotLightShadows+e.numSpotLightMaps-e.numSpotLightShadowsWithMaps;return n.replace(/NUM_DIR_LIGHTS/g,e.numDirLights).replace(/NUM_SPOT_LIGHTS/g,e.numSpotLights).replace(/NUM_SPOT_LIGHT_MAPS/g,e.numSpotLightMaps).replace(/NUM_SPOT_LIGHT_COORDS/g,t).replace(/NUM_RECT_AREA_LIGHTS/g,e.numRectAreaLights).replace(/NUM_POINT_LIGHTS/g,e.numPointLights).replace(/NUM_HEMI_LIGHTS/g,e.numHemiLights).replace(/NUM_DIR_LIGHT_SHADOWS/g,e.numDirLightShadows).replace(/NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS/g,e.numSpotLightShadowsWithMaps).replace(/NUM_SPOT_LIGHT_SHADOWS/g,e.numSpotLightShadows).replace(/NUM_POINT_LIGHT_SHADOWS/g,e.numPointLightShadows)}function Kc(n,e){return n.replace(/NUM_CLIPPING_PLANES/g,e.numClippingPlanes).replace(/UNION_CLIPPING_PLANES/g,e.numClippingPlanes-e.numClipIntersection)}const b_=/^[ \t]*#include +<([\w\d./]+)>/gm;function tl(n){return n.replace(b_,A_)}const T_=new Map;function A_(n,e){let t=Ye[e];if(t===void 0){const i=T_.get(e);if(i!==void 0)t=Ye[i],Ne('WebGLRenderer: Shader chunk "%s" has been deprecated. Use "%s" instead.',e,i);else throw new Error("Can not resolve #include <"+e+">")}return tl(t)}const w_=/#pragma unroll_loop_start\s+for\s*\(\s*int\s+i\s*=\s*(\d+)\s*;\s*i\s*<\s*(\d+)\s*;\s*i\s*\+\+\s*\)\s*{([\s\S]+?)}\s+#pragma unroll_loop_end/g;function Zc(n){return n.replace(w_,R_)}function R_(n,e,t,i){let s="";for(let r=parseInt(e);r<parseInt(t);r++)s+=i.replace(/\[\s*i\s*\]/g,"[ "+r+" ]").replace(/UNROLLED_LOOP_INDEX/g,r);return s}function Jc(n){let e=`precision ${n.precision} float;
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
	`;return n.precision==="highp"?e+=`
#define HIGH_PRECISION`:n.precision==="mediump"?e+=`
#define MEDIUM_PRECISION`:n.precision==="lowp"&&(e+=`
#define LOW_PRECISION`),e}const C_={[zr]:"SHADOWMAP_TYPE_PCF",[Ps]:"SHADOWMAP_TYPE_VSM"};function P_(n){return C_[n.shadowMapType]||"SHADOWMAP_TYPE_BASIC"}const L_={[Ci]:"ENVMAP_TYPE_CUBE",[ms]:"ENVMAP_TYPE_CUBE",[ga]:"ENVMAP_TYPE_CUBE_UV"};function D_(n){return n.envMap===!1?"ENVMAP_TYPE_CUBE":L_[n.envMapMode]||"ENVMAP_TYPE_CUBE"}const I_={[ms]:"ENVMAP_MODE_REFRACTION"};function U_(n){return n.envMap===!1?"ENVMAP_MODE_REFLECTION":I_[n.envMapMode]||"ENVMAP_MODE_REFLECTION"}const N_={[mh]:"ENVMAP_BLENDING_MULTIPLY",[Md]:"ENVMAP_BLENDING_MIX",[Sd]:"ENVMAP_BLENDING_ADD"};function F_(n){return n.envMap===!1?"ENVMAP_BLENDING_NONE":N_[n.combine]||"ENVMAP_BLENDING_NONE"}function O_(n){const e=n.envMapCubeUVHeight;if(e===null)return null;const t=Math.log2(e)-2,i=1/e;return{texelWidth:1/(3*Math.max(Math.pow(2,t),7*16)),texelHeight:i,maxMip:t}}function B_(n,e,t,i){const s=n.getContext(),r=t.defines;let a=t.vertexShader,o=t.fragmentShader;const c=P_(t),l=D_(t),u=U_(t),g=F_(t),h=O_(t),m=S_(t),_=y_(r),S=s.createProgram();let p,d,y=t.glslVersion?"#version "+t.glslVersion+`
`:"";t.isRawShaderMaterial?(p=["#define SHADER_TYPE "+t.shaderType,"#define SHADER_NAME "+t.shaderName,_].filter(Ls).join(`
`),p.length>0&&(p+=`
`),d=["#define SHADER_TYPE "+t.shaderType,"#define SHADER_NAME "+t.shaderName,_].filter(Ls).join(`
`),d.length>0&&(d+=`
`)):(p=[Jc(t),"#define SHADER_TYPE "+t.shaderType,"#define SHADER_NAME "+t.shaderName,_,t.extensionClipCullDistance?"#define USE_CLIP_DISTANCE":"",t.batching?"#define USE_BATCHING":"",t.batchingColor?"#define USE_BATCHING_COLOR":"",t.instancing?"#define USE_INSTANCING":"",t.instancingColor?"#define USE_INSTANCING_COLOR":"",t.instancingMorph?"#define USE_INSTANCING_MORPH":"",t.useFog&&t.fog?"#define USE_FOG":"",t.useFog&&t.fogExp2?"#define FOG_EXP2":"",t.map?"#define USE_MAP":"",t.envMap?"#define USE_ENVMAP":"",t.envMap?"#define "+u:"",t.lightMap?"#define USE_LIGHTMAP":"",t.aoMap?"#define USE_AOMAP":"",t.bumpMap?"#define USE_BUMPMAP":"",t.normalMap?"#define USE_NORMALMAP":"",t.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",t.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",t.displacementMap?"#define USE_DISPLACEMENTMAP":"",t.emissiveMap?"#define USE_EMISSIVEMAP":"",t.anisotropy?"#define USE_ANISOTROPY":"",t.anisotropyMap?"#define USE_ANISOTROPYMAP":"",t.clearcoatMap?"#define USE_CLEARCOATMAP":"",t.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",t.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",t.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",t.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",t.specularMap?"#define USE_SPECULARMAP":"",t.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",t.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",t.roughnessMap?"#define USE_ROUGHNESSMAP":"",t.metalnessMap?"#define USE_METALNESSMAP":"",t.alphaMap?"#define USE_ALPHAMAP":"",t.alphaHash?"#define USE_ALPHAHASH":"",t.transmission?"#define USE_TRANSMISSION":"",t.transmissionMap?"#define USE_TRANSMISSIONMAP":"",t.thicknessMap?"#define USE_THICKNESSMAP":"",t.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",t.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",t.mapUv?"#define MAP_UV "+t.mapUv:"",t.alphaMapUv?"#define ALPHAMAP_UV "+t.alphaMapUv:"",t.lightMapUv?"#define LIGHTMAP_UV "+t.lightMapUv:"",t.aoMapUv?"#define AOMAP_UV "+t.aoMapUv:"",t.emissiveMapUv?"#define EMISSIVEMAP_UV "+t.emissiveMapUv:"",t.bumpMapUv?"#define BUMPMAP_UV "+t.bumpMapUv:"",t.normalMapUv?"#define NORMALMAP_UV "+t.normalMapUv:"",t.displacementMapUv?"#define DISPLACEMENTMAP_UV "+t.displacementMapUv:"",t.metalnessMapUv?"#define METALNESSMAP_UV "+t.metalnessMapUv:"",t.roughnessMapUv?"#define ROUGHNESSMAP_UV "+t.roughnessMapUv:"",t.anisotropyMapUv?"#define ANISOTROPYMAP_UV "+t.anisotropyMapUv:"",t.clearcoatMapUv?"#define CLEARCOATMAP_UV "+t.clearcoatMapUv:"",t.clearcoatNormalMapUv?"#define CLEARCOAT_NORMALMAP_UV "+t.clearcoatNormalMapUv:"",t.clearcoatRoughnessMapUv?"#define CLEARCOAT_ROUGHNESSMAP_UV "+t.clearcoatRoughnessMapUv:"",t.iridescenceMapUv?"#define IRIDESCENCEMAP_UV "+t.iridescenceMapUv:"",t.iridescenceThicknessMapUv?"#define IRIDESCENCE_THICKNESSMAP_UV "+t.iridescenceThicknessMapUv:"",t.sheenColorMapUv?"#define SHEEN_COLORMAP_UV "+t.sheenColorMapUv:"",t.sheenRoughnessMapUv?"#define SHEEN_ROUGHNESSMAP_UV "+t.sheenRoughnessMapUv:"",t.specularMapUv?"#define SPECULARMAP_UV "+t.specularMapUv:"",t.specularColorMapUv?"#define SPECULAR_COLORMAP_UV "+t.specularColorMapUv:"",t.specularIntensityMapUv?"#define SPECULAR_INTENSITYMAP_UV "+t.specularIntensityMapUv:"",t.transmissionMapUv?"#define TRANSMISSIONMAP_UV "+t.transmissionMapUv:"",t.thicknessMapUv?"#define THICKNESSMAP_UV "+t.thicknessMapUv:"",t.vertexTangents&&t.flatShading===!1?"#define USE_TANGENT":"",t.vertexNormals?"#define HAS_NORMAL":"",t.vertexColors?"#define USE_COLOR":"",t.vertexAlphas?"#define USE_COLOR_ALPHA":"",t.vertexUv1s?"#define USE_UV1":"",t.vertexUv2s?"#define USE_UV2":"",t.vertexUv3s?"#define USE_UV3":"",t.pointsUvs?"#define USE_POINTS_UV":"",t.flatShading?"#define FLAT_SHADED":"",t.skinning?"#define USE_SKINNING":"",t.morphTargets?"#define USE_MORPHTARGETS":"",t.morphNormals&&t.flatShading===!1?"#define USE_MORPHNORMALS":"",t.morphColors?"#define USE_MORPHCOLORS":"",t.morphTargetsCount>0?"#define MORPHTARGETS_TEXTURE_STRIDE "+t.morphTextureStride:"",t.morphTargetsCount>0?"#define MORPHTARGETS_COUNT "+t.morphTargetsCount:"",t.doubleSided?"#define DOUBLE_SIDED":"",t.flipSided?"#define FLIP_SIDED":"",t.shadowMapEnabled?"#define USE_SHADOWMAP":"",t.shadowMapEnabled?"#define "+c:"",t.sizeAttenuation?"#define USE_SIZEATTENUATION":"",t.numLightProbes>0?"#define USE_LIGHT_PROBES":"",t.logarithmicDepthBuffer?"#define USE_LOGARITHMIC_DEPTH_BUFFER":"",t.reversedDepthBuffer?"#define USE_REVERSED_DEPTH_BUFFER":"","uniform mat4 modelMatrix;","uniform mat4 modelViewMatrix;","uniform mat4 projectionMatrix;","uniform mat4 viewMatrix;","uniform mat3 normalMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;","#ifdef USE_INSTANCING","	attribute mat4 instanceMatrix;","#endif","#ifdef USE_INSTANCING_COLOR","	attribute vec3 instanceColor;","#endif","#ifdef USE_INSTANCING_MORPH","	uniform sampler2D morphTexture;","#endif","attribute vec3 position;","attribute vec3 normal;","attribute vec2 uv;","#ifdef USE_UV1","	attribute vec2 uv1;","#endif","#ifdef USE_UV2","	attribute vec2 uv2;","#endif","#ifdef USE_UV3","	attribute vec2 uv3;","#endif","#ifdef USE_TANGENT","	attribute vec4 tangent;","#endif","#if defined( USE_COLOR_ALPHA )","	attribute vec4 color;","#elif defined( USE_COLOR )","	attribute vec3 color;","#endif","#ifdef USE_SKINNING","	attribute vec4 skinIndex;","	attribute vec4 skinWeight;","#endif",`
`].filter(Ls).join(`
`),d=[Jc(t),"#define SHADER_TYPE "+t.shaderType,"#define SHADER_NAME "+t.shaderName,_,t.useFog&&t.fog?"#define USE_FOG":"",t.useFog&&t.fogExp2?"#define FOG_EXP2":"",t.alphaToCoverage?"#define ALPHA_TO_COVERAGE":"",t.map?"#define USE_MAP":"",t.matcap?"#define USE_MATCAP":"",t.envMap?"#define USE_ENVMAP":"",t.envMap?"#define "+l:"",t.envMap?"#define "+u:"",t.envMap?"#define "+g:"",h?"#define CUBEUV_TEXEL_WIDTH "+h.texelWidth:"",h?"#define CUBEUV_TEXEL_HEIGHT "+h.texelHeight:"",h?"#define CUBEUV_MAX_MIP "+h.maxMip+".0":"",t.lightMap?"#define USE_LIGHTMAP":"",t.aoMap?"#define USE_AOMAP":"",t.bumpMap?"#define USE_BUMPMAP":"",t.normalMap?"#define USE_NORMALMAP":"",t.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",t.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",t.packedNormalMap?"#define USE_PACKED_NORMALMAP":"",t.emissiveMap?"#define USE_EMISSIVEMAP":"",t.anisotropy?"#define USE_ANISOTROPY":"",t.anisotropyMap?"#define USE_ANISOTROPYMAP":"",t.clearcoat?"#define USE_CLEARCOAT":"",t.clearcoatMap?"#define USE_CLEARCOATMAP":"",t.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",t.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",t.dispersion?"#define USE_DISPERSION":"",t.iridescence?"#define USE_IRIDESCENCE":"",t.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",t.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",t.specularMap?"#define USE_SPECULARMAP":"",t.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",t.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",t.roughnessMap?"#define USE_ROUGHNESSMAP":"",t.metalnessMap?"#define USE_METALNESSMAP":"",t.alphaMap?"#define USE_ALPHAMAP":"",t.alphaTest?"#define USE_ALPHATEST":"",t.alphaHash?"#define USE_ALPHAHASH":"",t.sheen?"#define USE_SHEEN":"",t.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",t.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",t.transmission?"#define USE_TRANSMISSION":"",t.transmissionMap?"#define USE_TRANSMISSIONMAP":"",t.thicknessMap?"#define USE_THICKNESSMAP":"",t.vertexTangents&&t.flatShading===!1?"#define USE_TANGENT":"",t.vertexColors||t.instancingColor?"#define USE_COLOR":"",t.vertexAlphas||t.batchingColor?"#define USE_COLOR_ALPHA":"",t.vertexUv1s?"#define USE_UV1":"",t.vertexUv2s?"#define USE_UV2":"",t.vertexUv3s?"#define USE_UV3":"",t.pointsUvs?"#define USE_POINTS_UV":"",t.gradientMap?"#define USE_GRADIENTMAP":"",t.flatShading?"#define FLAT_SHADED":"",t.doubleSided?"#define DOUBLE_SIDED":"",t.flipSided?"#define FLIP_SIDED":"",t.shadowMapEnabled?"#define USE_SHADOWMAP":"",t.shadowMapEnabled?"#define "+c:"",t.premultipliedAlpha?"#define PREMULTIPLIED_ALPHA":"",t.numLightProbes>0?"#define USE_LIGHT_PROBES":"",t.numLightProbeGrids>0?"#define USE_LIGHT_PROBES_GRID":"",t.decodeVideoTexture?"#define DECODE_VIDEO_TEXTURE":"",t.decodeVideoTextureEmissive?"#define DECODE_VIDEO_TEXTURE_EMISSIVE":"",t.logarithmicDepthBuffer?"#define USE_LOGARITHMIC_DEPTH_BUFFER":"",t.reversedDepthBuffer?"#define USE_REVERSED_DEPTH_BUFFER":"","uniform mat4 viewMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;",t.toneMapping!==Cn?"#define TONE_MAPPING":"",t.toneMapping!==Cn?Ye.tonemapping_pars_fragment:"",t.toneMapping!==Cn?v_("toneMapping",t.toneMapping):"",t.dithering?"#define DITHERING":"",t.opaque?"#define OPAQUE":"",Ye.colorspace_pars_fragment,__("linearToOutputTexel",t.outputColorSpace),M_(),t.useDepthPacking?"#define DEPTH_PACKING "+t.depthPacking:"",`
`].filter(Ls).join(`
`)),a=tl(a),a=jc(a,t),a=Kc(a,t),o=tl(o),o=jc(o,t),o=Kc(o,t),a=Zc(a),o=Zc(o),t.isRawShaderMaterial!==!0&&(y=`#version 300 es
`,p=[m,"#define attribute in","#define varying out","#define texture2D texture"].join(`
`)+`
`+p,d=["#define varying in",t.glslVersion===rc?"":"layout(location = 0) out highp vec4 pc_fragColor;",t.glslVersion===rc?"":"#define gl_FragColor pc_fragColor","#define gl_FragDepthEXT gl_FragDepth","#define texture2D texture","#define textureCube texture","#define texture2DProj textureProj","#define texture2DLodEXT textureLod","#define texture2DProjLodEXT textureProjLod","#define textureCubeLodEXT textureLod","#define texture2DGradEXT textureGrad","#define texture2DProjGradEXT textureProjGrad","#define textureCubeGradEXT textureGrad"].join(`
`)+`
`+d);const A=y+p+a,T=y+d+o,L=$c(s,s.VERTEX_SHADER,A),b=$c(s,s.FRAGMENT_SHADER,T);s.attachShader(S,L),s.attachShader(S,b),t.index0AttributeName!==void 0?s.bindAttribLocation(S,0,t.index0AttributeName):t.morphTargets===!0&&s.bindAttribLocation(S,0,"position"),s.linkProgram(S);function I(C){if(n.debug.checkShaderErrors){const z=s.getProgramInfoLog(S)||"",Z=s.getShaderInfoLog(L)||"",J=s.getShaderInfoLog(b)||"",O=z.trim(),W=Z.trim(),$=J.trim();let se=!0,oe=!0;if(s.getProgramParameter(S,s.LINK_STATUS)===!1)if(se=!1,typeof n.debug.onShaderError=="function")n.debug.onShaderError(s,S,L,b);else{const he=qc(s,L,"vertex"),Ee=qc(s,b,"fragment");Qe("THREE.WebGLProgram: Shader Error "+s.getError()+" - VALIDATE_STATUS "+s.getProgramParameter(S,s.VALIDATE_STATUS)+`

Material Name: `+C.name+`
Material Type: `+C.type+`

Program Info Log: `+O+`
`+he+`
`+Ee)}else O!==""?Ne("WebGLProgram: Program Info Log:",O):(W===""||$==="")&&(oe=!1);oe&&(C.diagnostics={runnable:se,programLog:O,vertexShader:{log:W,prefix:p},fragmentShader:{log:$,prefix:d}})}s.deleteShader(L),s.deleteShader(b),M=new Xr(s,S),w=E_(s,S)}let M;this.getUniforms=function(){return M===void 0&&I(this),M};let w;this.getAttributes=function(){return w===void 0&&I(this),w};let B=t.rendererExtensionParallelShaderCompile===!1;return this.isReady=function(){return B===!1&&(B=s.getProgramParameter(S,f_)),B},this.destroy=function(){i.releaseStatesOfProgram(this),s.deleteProgram(S),this.program=void 0},this.type=t.shaderType,this.name=t.shaderName,this.id=p_++,this.cacheKey=e,this.usedTimes=1,this.program=S,this.vertexShader=L,this.fragmentShader=b,this}let z_=0;class k_{constructor(){this.shaderCache=new Map,this.materialCache=new Map}update(e){const t=e.vertexShader,i=e.fragmentShader,s=this._getShaderStage(t),r=this._getShaderStage(i),a=this._getShaderCacheForMaterial(e);return a.has(s)===!1&&(a.add(s),s.usedTimes++),a.has(r)===!1&&(a.add(r),r.usedTimes++),this}remove(e){const t=this.materialCache.get(e);for(const i of t)i.usedTimes--,i.usedTimes===0&&this.shaderCache.delete(i.code);return this.materialCache.delete(e),this}getVertexShaderID(e){return this._getShaderStage(e.vertexShader).id}getFragmentShaderID(e){return this._getShaderStage(e.fragmentShader).id}dispose(){this.shaderCache.clear(),this.materialCache.clear()}_getShaderCacheForMaterial(e){const t=this.materialCache;let i=t.get(e);return i===void 0&&(i=new Set,t.set(e,i)),i}_getShaderStage(e){const t=this.shaderCache;let i=t.get(e);return i===void 0&&(i=new H_(e),t.set(e,i)),i}}class H_{constructor(e){this.id=z_++,this.code=e,this.usedTimes=0}}function G_(n){return n===Pi||n===ta||n===na}function V_(n,e,t,i,s,r){const a=new Ih,o=new k_,c=new Set,l=[],u=new Map,g=i.logarithmicDepthBuffer;let h=i.precision;const m={MeshDepthMaterial:"depth",MeshDistanceMaterial:"distance",MeshNormalMaterial:"normal",MeshBasicMaterial:"basic",MeshLambertMaterial:"lambert",MeshPhongMaterial:"phong",MeshToonMaterial:"toon",MeshStandardMaterial:"physical",MeshPhysicalMaterial:"physical",MeshMatcapMaterial:"matcap",LineBasicMaterial:"basic",LineDashedMaterial:"dashed",PointsMaterial:"points",ShadowMaterial:"shadow",SpriteMaterial:"sprite"};function _(M){return c.add(M),M===0?"uv":`uv${M}`}function S(M,w,B,C,z,Z){const J=C.fog,O=z.geometry,W=M.isMeshStandardMaterial||M.isMeshLambertMaterial||M.isMeshPhongMaterial?C.environment:null,$=M.isMeshStandardMaterial||M.isMeshLambertMaterial&&!M.envMap||M.isMeshPhongMaterial&&!M.envMap,se=e.get(M.envMap||W,$),oe=se&&se.mapping===ga?se.image.height:null,he=m[M.type];M.precision!==null&&(h=i.getMaxPrecision(M.precision),h!==M.precision&&Ne("WebGLProgram.getParameters:",M.precision,"not supported, using",h,"instead."));const Ee=O.morphAttributes.position||O.morphAttributes.normal||O.morphAttributes.color,Pe=Ee!==void 0?Ee.length:0;let Ge=0;O.morphAttributes.position!==void 0&&(Ge=1),O.morphAttributes.normal!==void 0&&(Ge=2),O.morphAttributes.color!==void 0&&(Ge=3);let Je,R,U,ie;if(he){const We=En[he];Je=We.vertexShader,R=We.fragmentShader}else Je=M.vertexShader,R=M.fragmentShader,o.update(M),U=o.getVertexShaderID(M),ie=o.getFragmentShaderID(M);const re=n.getRenderTarget(),Me=n.state.buffers.depth.getReversed(),Se=z.isInstancedMesh===!0,Ce=z.isBatchedMesh===!0,Ke=!!M.map,Fe=!!M.matcap,tt=!!se,rt=!!M.aoMap,Ve=!!M.lightMap,ft=!!M.bumpMap,G=!!M.normalMap,j=!!M.displacementMap,E=!!M.emissiveMap,ce=!!M.metalnessMap,le=!!M.roughnessMap,me=M.anisotropy>0,K=M.clearcoat>0,Re=M.dispersion>0,v=M.iridescence>0,f=M.sheen>0,D=M.transmission>0,q=me&&!!M.anisotropyMap,ne=K&&!!M.clearcoatMap,V=K&&!!M.clearcoatNormalMap,te=K&&!!M.clearcoatRoughnessMap,Y=v&&!!M.iridescenceMap,Q=v&&!!M.iridescenceThicknessMap,ve=f&&!!M.sheenColorMap,Te=f&&!!M.sheenRoughnessMap,fe=!!M.specularMap,ue=!!M.specularColorMap,Be=!!M.specularIntensityMap,$e=D&&!!M.transmissionMap,it=D&&!!M.thicknessMap,P=!!M.gradientMap,de=!!M.alphaMap,ee=M.alphaTest>0,ye=!!M.alphaHash,pe=!!M.extensions;let ae=Cn;M.toneMapped&&(re===null||re.isXRRenderTarget===!0)&&(ae=n.toneMapping);const De={shaderID:he,shaderType:M.type,shaderName:M.name,vertexShader:Je,fragmentShader:R,defines:M.defines,customVertexShaderID:U,customFragmentShaderID:ie,isRawShaderMaterial:M.isRawShaderMaterial===!0,glslVersion:M.glslVersion,precision:h,batching:Ce,batchingColor:Ce&&z._colorsTexture!==null,instancing:Se,instancingColor:Se&&z.instanceColor!==null,instancingMorph:Se&&z.morphTexture!==null,outputColorSpace:re===null?n.outputColorSpace:re.isXRRenderTarget===!0?re.texture.colorSpace:Ze.workingColorSpace,alphaToCoverage:!!M.alphaToCoverage,map:Ke,matcap:Fe,envMap:tt,envMapMode:tt&&se.mapping,envMapCubeUVHeight:oe,aoMap:rt,lightMap:Ve,bumpMap:ft,normalMap:G,displacementMap:j,emissiveMap:E,normalMapObjectSpace:G&&M.normalMapType===bd,normalMapTangentSpace:G&&M.normalMapType===ic,packedNormalMap:G&&M.normalMapType===ic&&G_(M.normalMap.format),metalnessMap:ce,roughnessMap:le,anisotropy:me,anisotropyMap:q,clearcoat:K,clearcoatMap:ne,clearcoatNormalMap:V,clearcoatRoughnessMap:te,dispersion:Re,iridescence:v,iridescenceMap:Y,iridescenceThicknessMap:Q,sheen:f,sheenColorMap:ve,sheenRoughnessMap:Te,specularMap:fe,specularColorMap:ue,specularIntensityMap:Be,transmission:D,transmissionMap:$e,thicknessMap:it,gradientMap:P,opaque:M.transparent===!1&&M.blending===hs&&M.alphaToCoverage===!1,alphaMap:de,alphaTest:ee,alphaHash:ye,combine:M.combine,mapUv:Ke&&_(M.map.channel),aoMapUv:rt&&_(M.aoMap.channel),lightMapUv:Ve&&_(M.lightMap.channel),bumpMapUv:ft&&_(M.bumpMap.channel),normalMapUv:G&&_(M.normalMap.channel),displacementMapUv:j&&_(M.displacementMap.channel),emissiveMapUv:E&&_(M.emissiveMap.channel),metalnessMapUv:ce&&_(M.metalnessMap.channel),roughnessMapUv:le&&_(M.roughnessMap.channel),anisotropyMapUv:q&&_(M.anisotropyMap.channel),clearcoatMapUv:ne&&_(M.clearcoatMap.channel),clearcoatNormalMapUv:V&&_(M.clearcoatNormalMap.channel),clearcoatRoughnessMapUv:te&&_(M.clearcoatRoughnessMap.channel),iridescenceMapUv:Y&&_(M.iridescenceMap.channel),iridescenceThicknessMapUv:Q&&_(M.iridescenceThicknessMap.channel),sheenColorMapUv:ve&&_(M.sheenColorMap.channel),sheenRoughnessMapUv:Te&&_(M.sheenRoughnessMap.channel),specularMapUv:fe&&_(M.specularMap.channel),specularColorMapUv:ue&&_(M.specularColorMap.channel),specularIntensityMapUv:Be&&_(M.specularIntensityMap.channel),transmissionMapUv:$e&&_(M.transmissionMap.channel),thicknessMapUv:it&&_(M.thicknessMap.channel),alphaMapUv:de&&_(M.alphaMap.channel),vertexTangents:!!O.attributes.tangent&&(G||me),vertexNormals:!!O.attributes.normal,vertexColors:M.vertexColors,vertexAlphas:M.vertexColors===!0&&!!O.attributes.color&&O.attributes.color.itemSize===4,pointsUvs:z.isPoints===!0&&!!O.attributes.uv&&(Ke||de),fog:!!J,useFog:M.fog===!0,fogExp2:!!J&&J.isFogExp2,flatShading:M.wireframe===!1&&(M.flatShading===!0||O.attributes.normal===void 0&&G===!1&&(M.isMeshLambertMaterial||M.isMeshPhongMaterial||M.isMeshStandardMaterial||M.isMeshPhysicalMaterial)),sizeAttenuation:M.sizeAttenuation===!0,logarithmicDepthBuffer:g,reversedDepthBuffer:Me,skinning:z.isSkinnedMesh===!0,morphTargets:O.morphAttributes.position!==void 0,morphNormals:O.morphAttributes.normal!==void 0,morphColors:O.morphAttributes.color!==void 0,morphTargetsCount:Pe,morphTextureStride:Ge,numDirLights:w.directional.length,numPointLights:w.point.length,numSpotLights:w.spot.length,numSpotLightMaps:w.spotLightMap.length,numRectAreaLights:w.rectArea.length,numHemiLights:w.hemi.length,numDirLightShadows:w.directionalShadowMap.length,numPointLightShadows:w.pointShadowMap.length,numSpotLightShadows:w.spotShadowMap.length,numSpotLightShadowsWithMaps:w.numSpotLightShadowsWithMaps,numLightProbes:w.numLightProbes,numLightProbeGrids:Z.length,numClippingPlanes:r.numPlanes,numClipIntersection:r.numIntersection,dithering:M.dithering,shadowMapEnabled:n.shadowMap.enabled&&B.length>0,shadowMapType:n.shadowMap.type,toneMapping:ae,decodeVideoTexture:Ke&&M.map.isVideoTexture===!0&&Ze.getTransfer(M.map.colorSpace)===at,decodeVideoTextureEmissive:E&&M.emissiveMap.isVideoTexture===!0&&Ze.getTransfer(M.emissiveMap.colorSpace)===at,premultipliedAlpha:M.premultipliedAlpha,doubleSided:M.side===Bn,flipSided:M.side===Kt,useDepthPacking:M.depthPacking>=0,depthPacking:M.depthPacking||0,index0AttributeName:M.index0AttributeName,extensionClipCullDistance:pe&&M.extensions.clipCullDistance===!0&&t.has("WEBGL_clip_cull_distance"),extensionMultiDraw:(pe&&M.extensions.multiDraw===!0||Ce)&&t.has("WEBGL_multi_draw"),rendererExtensionParallelShaderCompile:t.has("KHR_parallel_shader_compile"),customProgramCacheKey:M.customProgramCacheKey()};return De.vertexUv1s=c.has(1),De.vertexUv2s=c.has(2),De.vertexUv3s=c.has(3),c.clear(),De}function p(M){const w=[];if(M.shaderID?w.push(M.shaderID):(w.push(M.customVertexShaderID),w.push(M.customFragmentShaderID)),M.defines!==void 0)for(const B in M.defines)w.push(B),w.push(M.defines[B]);return M.isRawShaderMaterial===!1&&(d(w,M),y(w,M),w.push(n.outputColorSpace)),w.push(M.customProgramCacheKey),w.join()}function d(M,w){M.push(w.precision),M.push(w.outputColorSpace),M.push(w.envMapMode),M.push(w.envMapCubeUVHeight),M.push(w.mapUv),M.push(w.alphaMapUv),M.push(w.lightMapUv),M.push(w.aoMapUv),M.push(w.bumpMapUv),M.push(w.normalMapUv),M.push(w.displacementMapUv),M.push(w.emissiveMapUv),M.push(w.metalnessMapUv),M.push(w.roughnessMapUv),M.push(w.anisotropyMapUv),M.push(w.clearcoatMapUv),M.push(w.clearcoatNormalMapUv),M.push(w.clearcoatRoughnessMapUv),M.push(w.iridescenceMapUv),M.push(w.iridescenceThicknessMapUv),M.push(w.sheenColorMapUv),M.push(w.sheenRoughnessMapUv),M.push(w.specularMapUv),M.push(w.specularColorMapUv),M.push(w.specularIntensityMapUv),M.push(w.transmissionMapUv),M.push(w.thicknessMapUv),M.push(w.combine),M.push(w.fogExp2),M.push(w.sizeAttenuation),M.push(w.morphTargetsCount),M.push(w.morphAttributeCount),M.push(w.numDirLights),M.push(w.numPointLights),M.push(w.numSpotLights),M.push(w.numSpotLightMaps),M.push(w.numHemiLights),M.push(w.numRectAreaLights),M.push(w.numDirLightShadows),M.push(w.numPointLightShadows),M.push(w.numSpotLightShadows),M.push(w.numSpotLightShadowsWithMaps),M.push(w.numLightProbes),M.push(w.shadowMapType),M.push(w.toneMapping),M.push(w.numClippingPlanes),M.push(w.numClipIntersection),M.push(w.depthPacking)}function y(M,w){a.disableAll(),w.instancing&&a.enable(0),w.instancingColor&&a.enable(1),w.instancingMorph&&a.enable(2),w.matcap&&a.enable(3),w.envMap&&a.enable(4),w.normalMapObjectSpace&&a.enable(5),w.normalMapTangentSpace&&a.enable(6),w.clearcoat&&a.enable(7),w.iridescence&&a.enable(8),w.alphaTest&&a.enable(9),w.vertexColors&&a.enable(10),w.vertexAlphas&&a.enable(11),w.vertexUv1s&&a.enable(12),w.vertexUv2s&&a.enable(13),w.vertexUv3s&&a.enable(14),w.vertexTangents&&a.enable(15),w.anisotropy&&a.enable(16),w.alphaHash&&a.enable(17),w.batching&&a.enable(18),w.dispersion&&a.enable(19),w.batchingColor&&a.enable(20),w.gradientMap&&a.enable(21),w.packedNormalMap&&a.enable(22),w.vertexNormals&&a.enable(23),M.push(a.mask),a.disableAll(),w.fog&&a.enable(0),w.useFog&&a.enable(1),w.flatShading&&a.enable(2),w.logarithmicDepthBuffer&&a.enable(3),w.reversedDepthBuffer&&a.enable(4),w.skinning&&a.enable(5),w.morphTargets&&a.enable(6),w.morphNormals&&a.enable(7),w.morphColors&&a.enable(8),w.premultipliedAlpha&&a.enable(9),w.shadowMapEnabled&&a.enable(10),w.doubleSided&&a.enable(11),w.flipSided&&a.enable(12),w.useDepthPacking&&a.enable(13),w.dithering&&a.enable(14),w.transmission&&a.enable(15),w.sheen&&a.enable(16),w.opaque&&a.enable(17),w.pointsUvs&&a.enable(18),w.decodeVideoTexture&&a.enable(19),w.decodeVideoTextureEmissive&&a.enable(20),w.alphaToCoverage&&a.enable(21),w.numLightProbeGrids>0&&a.enable(22),M.push(a.mask)}function A(M){const w=m[M.type];let B;if(w){const C=En[w];B=df.clone(C.uniforms)}else B=M.uniforms;return B}function T(M,w){let B=u.get(w);return B!==void 0?++B.usedTimes:(B=new B_(n,w,M,s),l.push(B),u.set(w,B)),B}function L(M){if(--M.usedTimes===0){const w=l.indexOf(M);l[w]=l[l.length-1],l.pop(),u.delete(M.cacheKey),M.destroy()}}function b(M){o.remove(M)}function I(){o.dispose()}return{getParameters:S,getProgramCacheKey:p,getUniforms:A,acquireProgram:T,releaseProgram:L,releaseShaderCache:b,programs:l,dispose:I}}function W_(){let n=new WeakMap;function e(a){return n.has(a)}function t(a){let o=n.get(a);return o===void 0&&(o={},n.set(a,o)),o}function i(a){n.delete(a)}function s(a,o,c){n.get(a)[o]=c}function r(){n=new WeakMap}return{has:e,get:t,remove:i,update:s,dispose:r}}function X_(n,e){return n.groupOrder!==e.groupOrder?n.groupOrder-e.groupOrder:n.renderOrder!==e.renderOrder?n.renderOrder-e.renderOrder:n.material.id!==e.material.id?n.material.id-e.material.id:n.materialVariant!==e.materialVariant?n.materialVariant-e.materialVariant:n.z!==e.z?n.z-e.z:n.id-e.id}function Qc(n,e){return n.groupOrder!==e.groupOrder?n.groupOrder-e.groupOrder:n.renderOrder!==e.renderOrder?n.renderOrder-e.renderOrder:n.z!==e.z?e.z-n.z:n.id-e.id}function eh(){const n=[];let e=0;const t=[],i=[],s=[];function r(){e=0,t.length=0,i.length=0,s.length=0}function a(h){let m=0;return h.isInstancedMesh&&(m+=2),h.isSkinnedMesh&&(m+=1),m}function o(h,m,_,S,p,d){let y=n[e];return y===void 0?(y={id:h.id,object:h,geometry:m,material:_,materialVariant:a(h),groupOrder:S,renderOrder:h.renderOrder,z:p,group:d},n[e]=y):(y.id=h.id,y.object=h,y.geometry=m,y.material=_,y.materialVariant=a(h),y.groupOrder=S,y.renderOrder=h.renderOrder,y.z=p,y.group=d),e++,y}function c(h,m,_,S,p,d){const y=o(h,m,_,S,p,d);_.transmission>0?i.push(y):_.transparent===!0?s.push(y):t.push(y)}function l(h,m,_,S,p,d){const y=o(h,m,_,S,p,d);_.transmission>0?i.unshift(y):_.transparent===!0?s.unshift(y):t.unshift(y)}function u(h,m){t.length>1&&t.sort(h||X_),i.length>1&&i.sort(m||Qc),s.length>1&&s.sort(m||Qc)}function g(){for(let h=e,m=n.length;h<m;h++){const _=n[h];if(_.id===null)break;_.id=null,_.object=null,_.geometry=null,_.material=null,_.group=null}}return{opaque:t,transmissive:i,transparent:s,init:r,push:c,unshift:l,finish:g,sort:u}}function $_(){let n=new WeakMap;function e(i,s){const r=n.get(i);let a;return r===void 0?(a=new eh,n.set(i,[a])):s>=r.length?(a=new eh,r.push(a)):a=r[s],a}function t(){n=new WeakMap}return{get:e,dispose:t}}function Y_(){const n={};return{get:function(e){if(n[e.id]!==void 0)return n[e.id];let t;switch(e.type){case"DirectionalLight":t={direction:new N,color:new ke};break;case"SpotLight":t={position:new N,direction:new N,color:new ke,distance:0,coneCos:0,penumbraCos:0,decay:0};break;case"PointLight":t={position:new N,color:new ke,distance:0,decay:0};break;case"HemisphereLight":t={direction:new N,skyColor:new ke,groundColor:new ke};break;case"RectAreaLight":t={color:new ke,position:new N,halfWidth:new N,halfHeight:new N};break}return n[e.id]=t,t}}}function q_(){const n={};return{get:function(e){if(n[e.id]!==void 0)return n[e.id];let t;switch(e.type){case"DirectionalLight":t={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Oe};break;case"SpotLight":t={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Oe};break;case"PointLight":t={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Oe,shadowCameraNear:1,shadowCameraFar:1e3};break}return n[e.id]=t,t}}}let j_=0;function K_(n,e){return(e.castShadow?2:0)-(n.castShadow?2:0)+(e.map?1:0)-(n.map?1:0)}function Z_(n){const e=new Y_,t=q_(),i={version:0,hash:{directionalLength:-1,pointLength:-1,spotLength:-1,rectAreaLength:-1,hemiLength:-1,numDirectionalShadows:-1,numPointShadows:-1,numSpotShadows:-1,numSpotMaps:-1,numLightProbes:-1},ambient:[0,0,0],probe:[],directional:[],directionalShadow:[],directionalShadowMap:[],directionalShadowMatrix:[],spot:[],spotLightMap:[],spotShadow:[],spotShadowMap:[],spotLightMatrix:[],rectArea:[],rectAreaLTC1:null,rectAreaLTC2:null,point:[],pointShadow:[],pointShadowMap:[],pointShadowMatrix:[],hemi:[],numSpotLightShadowsWithMaps:0,numLightProbes:0};for(let l=0;l<9;l++)i.probe.push(new N);const s=new N,r=new gt,a=new gt;function o(l){let u=0,g=0,h=0;for(let w=0;w<9;w++)i.probe[w].set(0,0,0);let m=0,_=0,S=0,p=0,d=0,y=0,A=0,T=0,L=0,b=0,I=0;l.sort(K_);for(let w=0,B=l.length;w<B;w++){const C=l[w],z=C.color,Z=C.intensity,J=C.distance;let O=null;if(C.shadow&&C.shadow.map&&(C.shadow.map.texture.format===Pi?O=C.shadow.map.texture:O=C.shadow.map.depthTexture||C.shadow.map.texture),C.isAmbientLight)u+=z.r*Z,g+=z.g*Z,h+=z.b*Z;else if(C.isLightProbe){for(let W=0;W<9;W++)i.probe[W].addScaledVector(C.sh.coefficients[W],Z);I++}else if(C.isDirectionalLight){const W=e.get(C);if(W.color.copy(C.color).multiplyScalar(C.intensity),C.castShadow){const $=C.shadow,se=t.get(C);se.shadowIntensity=$.intensity,se.shadowBias=$.bias,se.shadowNormalBias=$.normalBias,se.shadowRadius=$.radius,se.shadowMapSize=$.mapSize,i.directionalShadow[m]=se,i.directionalShadowMap[m]=O,i.directionalShadowMatrix[m]=C.shadow.matrix,y++}i.directional[m]=W,m++}else if(C.isSpotLight){const W=e.get(C);W.position.setFromMatrixPosition(C.matrixWorld),W.color.copy(z).multiplyScalar(Z),W.distance=J,W.coneCos=Math.cos(C.angle),W.penumbraCos=Math.cos(C.angle*(1-C.penumbra)),W.decay=C.decay,i.spot[S]=W;const $=C.shadow;if(C.map&&(i.spotLightMap[L]=C.map,L++,$.updateMatrices(C),C.castShadow&&b++),i.spotLightMatrix[S]=$.matrix,C.castShadow){const se=t.get(C);se.shadowIntensity=$.intensity,se.shadowBias=$.bias,se.shadowNormalBias=$.normalBias,se.shadowRadius=$.radius,se.shadowMapSize=$.mapSize,i.spotShadow[S]=se,i.spotShadowMap[S]=O,T++}S++}else if(C.isRectAreaLight){const W=e.get(C);W.color.copy(z).multiplyScalar(Z),W.halfWidth.set(C.width*.5,0,0),W.halfHeight.set(0,C.height*.5,0),i.rectArea[p]=W,p++}else if(C.isPointLight){const W=e.get(C);if(W.color.copy(C.color).multiplyScalar(C.intensity),W.distance=C.distance,W.decay=C.decay,C.castShadow){const $=C.shadow,se=t.get(C);se.shadowIntensity=$.intensity,se.shadowBias=$.bias,se.shadowNormalBias=$.normalBias,se.shadowRadius=$.radius,se.shadowMapSize=$.mapSize,se.shadowCameraNear=$.camera.near,se.shadowCameraFar=$.camera.far,i.pointShadow[_]=se,i.pointShadowMap[_]=O,i.pointShadowMatrix[_]=C.shadow.matrix,A++}i.point[_]=W,_++}else if(C.isHemisphereLight){const W=e.get(C);W.skyColor.copy(C.color).multiplyScalar(Z),W.groundColor.copy(C.groundColor).multiplyScalar(Z),i.hemi[d]=W,d++}}p>0&&(n.has("OES_texture_float_linear")===!0?(i.rectAreaLTC1=ge.LTC_FLOAT_1,i.rectAreaLTC2=ge.LTC_FLOAT_2):(i.rectAreaLTC1=ge.LTC_HALF_1,i.rectAreaLTC2=ge.LTC_HALF_2)),i.ambient[0]=u,i.ambient[1]=g,i.ambient[2]=h;const M=i.hash;(M.directionalLength!==m||M.pointLength!==_||M.spotLength!==S||M.rectAreaLength!==p||M.hemiLength!==d||M.numDirectionalShadows!==y||M.numPointShadows!==A||M.numSpotShadows!==T||M.numSpotMaps!==L||M.numLightProbes!==I)&&(i.directional.length=m,i.spot.length=S,i.rectArea.length=p,i.point.length=_,i.hemi.length=d,i.directionalShadow.length=y,i.directionalShadowMap.length=y,i.pointShadow.length=A,i.pointShadowMap.length=A,i.spotShadow.length=T,i.spotShadowMap.length=T,i.directionalShadowMatrix.length=y,i.pointShadowMatrix.length=A,i.spotLightMatrix.length=T+L-b,i.spotLightMap.length=L,i.numSpotLightShadowsWithMaps=b,i.numLightProbes=I,M.directionalLength=m,M.pointLength=_,M.spotLength=S,M.rectAreaLength=p,M.hemiLength=d,M.numDirectionalShadows=y,M.numPointShadows=A,M.numSpotShadows=T,M.numSpotMaps=L,M.numLightProbes=I,i.version=j_++)}function c(l,u){let g=0,h=0,m=0,_=0,S=0;const p=u.matrixWorldInverse;for(let d=0,y=l.length;d<y;d++){const A=l[d];if(A.isDirectionalLight){const T=i.directional[g];T.direction.setFromMatrixPosition(A.matrixWorld),s.setFromMatrixPosition(A.target.matrixWorld),T.direction.sub(s),T.direction.transformDirection(p),g++}else if(A.isSpotLight){const T=i.spot[m];T.position.setFromMatrixPosition(A.matrixWorld),T.position.applyMatrix4(p),T.direction.setFromMatrixPosition(A.matrixWorld),s.setFromMatrixPosition(A.target.matrixWorld),T.direction.sub(s),T.direction.transformDirection(p),m++}else if(A.isRectAreaLight){const T=i.rectArea[_];T.position.setFromMatrixPosition(A.matrixWorld),T.position.applyMatrix4(p),a.identity(),r.copy(A.matrixWorld),r.premultiply(p),a.extractRotation(r),T.halfWidth.set(A.width*.5,0,0),T.halfHeight.set(0,A.height*.5,0),T.halfWidth.applyMatrix4(a),T.halfHeight.applyMatrix4(a),_++}else if(A.isPointLight){const T=i.point[h];T.position.setFromMatrixPosition(A.matrixWorld),T.position.applyMatrix4(p),h++}else if(A.isHemisphereLight){const T=i.hemi[S];T.direction.setFromMatrixPosition(A.matrixWorld),T.direction.transformDirection(p),S++}}}return{setup:o,setupView:c,state:i}}function th(n){const e=new Z_(n),t=[],i=[],s=[];function r(h){g.camera=h,t.length=0,i.length=0,s.length=0}function a(h){t.push(h)}function o(h){i.push(h)}function c(h){s.push(h)}function l(){e.setup(t)}function u(h){e.setupView(t,h)}const g={lightsArray:t,shadowsArray:i,lightProbeGridArray:s,camera:null,lights:e,transmissionRenderTarget:{},textureUnits:0};return{init:r,state:g,setupLights:l,setupLightsView:u,pushLight:a,pushShadow:o,pushLightProbeGrid:c}}function J_(n){let e=new WeakMap;function t(s,r=0){const a=e.get(s);let o;return a===void 0?(o=new th(n),e.set(s,[o])):r>=a.length?(o=new th(n),a.push(o)):o=a[r],o}function i(){e=new WeakMap}return{get:t,dispose:i}}const Q_=`void main() {
	gl_Position = vec4( position, 1.0 );
}`,e0=`uniform sampler2D shadow_pass;
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
}`,t0=[new N(1,0,0),new N(-1,0,0),new N(0,1,0),new N(0,-1,0),new N(0,0,1),new N(0,0,-1)],n0=[new N(0,-1,0),new N(0,-1,0),new N(0,0,1),new N(0,0,-1),new N(0,-1,0),new N(0,-1,0)],nh=new gt,Cs=new N,so=new N;function i0(n,e,t){let i=new kh;const s=new Oe,r=new Oe,a=new Et,o=new gf,c=new _f,l={},u=t.maxTextureSize,g={[ci]:Kt,[Kt]:ci,[Bn]:Bn},h=new gn({defines:{VSM_SAMPLES:8},uniforms:{shadow_pass:{value:null},resolution:{value:new Oe},radius:{value:4}},vertexShader:Q_,fragmentShader:e0}),m=h.clone();m.defines.HORIZONTAL_PASS=1;const _=new Ut;_.setAttribute("position",new At(new Float32Array([-1,-1,.5,3,-1,.5,-1,3,.5]),3));const S=new Xn(_,h),p=this;this.enabled=!1,this.autoUpdate=!0,this.needsUpdate=!1,this.type=zr;let d=this.type;this.render=function(b,I,M){if(p.enabled===!1||p.autoUpdate===!1&&p.needsUpdate===!1||b.length===0)return;this.type===td&&(Ne("WebGLShadowMap: PCFSoftShadowMap has been deprecated. Using PCFShadowMap instead."),this.type=zr);const w=n.getRenderTarget(),B=n.getActiveCubeFace(),C=n.getActiveMipmapLevel(),z=n.state;z.setBlending(Hn),z.buffers.depth.getReversed()===!0?z.buffers.color.setClear(0,0,0,0):z.buffers.color.setClear(1,1,1,1),z.buffers.depth.setTest(!0),z.setScissorTest(!1);const Z=d!==this.type;Z&&I.traverse(function(J){J.material&&(Array.isArray(J.material)?J.material.forEach(O=>O.needsUpdate=!0):J.material.needsUpdate=!0)});for(let J=0,O=b.length;J<O;J++){const W=b[J],$=W.shadow;if($===void 0){Ne("WebGLShadowMap:",W,"has no shadow.");continue}if($.autoUpdate===!1&&$.needsUpdate===!1)continue;s.copy($.mapSize);const se=$.getFrameExtents();s.multiply(se),r.copy($.mapSize),(s.x>u||s.y>u)&&(s.x>u&&(r.x=Math.floor(u/se.x),s.x=r.x*se.x,$.mapSize.x=r.x),s.y>u&&(r.y=Math.floor(u/se.y),s.y=r.y*se.y,$.mapSize.y=r.y));const oe=n.state.buffers.depth.getReversed();if($.camera._reversedDepth=oe,$.map===null||Z===!0){if($.map!==null&&($.map.depthTexture!==null&&($.map.depthTexture.dispose(),$.map.depthTexture=null),$.map.dispose()),this.type===Ps){if(W.isPointLight){Ne("WebGLShadowMap: VSM shadow maps are not supported for PointLights. Use PCF or BasicShadowMap instead.");continue}$.map=new Pn(s.x,s.y,{format:Pi,type:Vn,minFilter:Bt,magFilter:Bt,generateMipmaps:!1}),$.map.texture.name=W.name+".shadowMap",$.map.depthTexture=new gs(s.x,s.y,An),$.map.depthTexture.name=W.name+".shadowMapDepth",$.map.depthTexture.format=Wn,$.map.depthTexture.compareFunction=null,$.map.depthTexture.minFilter=Lt,$.map.depthTexture.magFilter=Lt}else W.isPointLight?($.map=new Zh(s.x),$.map.depthTexture=new hf(s.x,Ln)):($.map=new Pn(s.x,s.y),$.map.depthTexture=new gs(s.x,s.y,Ln)),$.map.depthTexture.name=W.name+".shadowMap",$.map.depthTexture.format=Wn,this.type===zr?($.map.depthTexture.compareFunction=oe?El:yl,$.map.depthTexture.minFilter=Bt,$.map.depthTexture.magFilter=Bt):($.map.depthTexture.compareFunction=null,$.map.depthTexture.minFilter=Lt,$.map.depthTexture.magFilter=Lt);$.camera.updateProjectionMatrix()}const he=$.map.isWebGLCubeRenderTarget?6:1;for(let Ee=0;Ee<he;Ee++){if($.map.isWebGLCubeRenderTarget)n.setRenderTarget($.map,Ee),n.clear();else{Ee===0&&(n.setRenderTarget($.map),n.clear());const Pe=$.getViewport(Ee);a.set(r.x*Pe.x,r.y*Pe.y,r.x*Pe.z,r.y*Pe.w),z.viewport(a)}if(W.isPointLight){const Pe=$.camera,Ge=$.matrix,Je=W.distance||Pe.far;Je!==Pe.far&&(Pe.far=Je,Pe.updateProjectionMatrix()),Cs.setFromMatrixPosition(W.matrixWorld),Pe.position.copy(Cs),so.copy(Pe.position),so.add(t0[Ee]),Pe.up.copy(n0[Ee]),Pe.lookAt(so),Pe.updateMatrixWorld(),Ge.makeTranslation(-Cs.x,-Cs.y,-Cs.z),nh.multiplyMatrices(Pe.projectionMatrix,Pe.matrixWorldInverse),$._frustum.setFromProjectionMatrix(nh,Pe.coordinateSystem,Pe.reversedDepth)}else $.updateMatrices(W);i=$.getFrustum(),T(I,M,$.camera,W,this.type)}$.isPointLightShadow!==!0&&this.type===Ps&&y($,M),$.needsUpdate=!1}d=this.type,p.needsUpdate=!1,n.setRenderTarget(w,B,C)};function y(b,I){const M=e.update(S);h.defines.VSM_SAMPLES!==b.blurSamples&&(h.defines.VSM_SAMPLES=b.blurSamples,m.defines.VSM_SAMPLES=b.blurSamples,h.needsUpdate=!0,m.needsUpdate=!0),b.mapPass===null&&(b.mapPass=new Pn(s.x,s.y,{format:Pi,type:Vn})),h.uniforms.shadow_pass.value=b.map.depthTexture,h.uniforms.resolution.value=b.mapSize,h.uniforms.radius.value=b.radius,n.setRenderTarget(b.mapPass),n.clear(),n.renderBufferDirect(I,null,M,h,S,null),m.uniforms.shadow_pass.value=b.mapPass.texture,m.uniforms.resolution.value=b.mapSize,m.uniforms.radius.value=b.radius,n.setRenderTarget(b.map),n.clear(),n.renderBufferDirect(I,null,M,m,S,null)}function A(b,I,M,w){let B=null;const C=M.isPointLight===!0?b.customDistanceMaterial:b.customDepthMaterial;if(C!==void 0)B=C;else if(B=M.isPointLight===!0?c:o,n.localClippingEnabled&&I.clipShadows===!0&&Array.isArray(I.clippingPlanes)&&I.clippingPlanes.length!==0||I.displacementMap&&I.displacementScale!==0||I.alphaMap&&I.alphaTest>0||I.map&&I.alphaTest>0||I.alphaToCoverage===!0){const z=B.uuid,Z=I.uuid;let J=l[z];J===void 0&&(J={},l[z]=J);let O=J[Z];O===void 0&&(O=B.clone(),J[Z]=O,I.addEventListener("dispose",L)),B=O}if(B.visible=I.visible,B.wireframe=I.wireframe,w===Ps?B.side=I.shadowSide!==null?I.shadowSide:I.side:B.side=I.shadowSide!==null?I.shadowSide:g[I.side],B.alphaMap=I.alphaMap,B.alphaTest=I.alphaToCoverage===!0?.5:I.alphaTest,B.map=I.map,B.clipShadows=I.clipShadows,B.clippingPlanes=I.clippingPlanes,B.clipIntersection=I.clipIntersection,B.displacementMap=I.displacementMap,B.displacementScale=I.displacementScale,B.displacementBias=I.displacementBias,B.wireframeLinewidth=I.wireframeLinewidth,B.linewidth=I.linewidth,M.isPointLight===!0&&B.isMeshDistanceMaterial===!0){const z=n.properties.get(B);z.light=M}return B}function T(b,I,M,w,B){if(b.visible===!1)return;if(b.layers.test(I.layers)&&(b.isMesh||b.isLine||b.isPoints)&&(b.castShadow||b.receiveShadow&&B===Ps)&&(!b.frustumCulled||i.intersectsObject(b))){b.modelViewMatrix.multiplyMatrices(M.matrixWorldInverse,b.matrixWorld);const Z=e.update(b),J=b.material;if(Array.isArray(J)){const O=Z.groups;for(let W=0,$=O.length;W<$;W++){const se=O[W],oe=J[se.materialIndex];if(oe&&oe.visible){const he=A(b,oe,w,B);b.onBeforeShadow(n,b,I,M,Z,he,se),n.renderBufferDirect(M,null,Z,he,b,se),b.onAfterShadow(n,b,I,M,Z,he,se)}}}else if(J.visible){const O=A(b,J,w,B);b.onBeforeShadow(n,b,I,M,Z,O,null),n.renderBufferDirect(M,null,Z,O,b,null),b.onAfterShadow(n,b,I,M,Z,O,null)}}const z=b.children;for(let Z=0,J=z.length;Z<J;Z++)T(z[Z],I,M,w,B)}function L(b){b.target.removeEventListener("dispose",L);for(const M in l){const w=l[M],B=b.target.uuid;B in w&&(w[B].dispose(),delete w[B])}}}function s0(n,e){function t(){let P=!1;const de=new Et;let ee=null;const ye=new Et(0,0,0,0);return{setMask:function(pe){ee!==pe&&!P&&(n.colorMask(pe,pe,pe,pe),ee=pe)},setLocked:function(pe){P=pe},setClear:function(pe,ae,De,We,vt){vt===!0&&(pe*=We,ae*=We,De*=We),de.set(pe,ae,De,We),ye.equals(de)===!1&&(n.clearColor(pe,ae,De,We),ye.copy(de))},reset:function(){P=!1,ee=null,ye.set(-1,0,0,0)}}}function i(){let P=!1,de=!1,ee=null,ye=null,pe=null;return{setReversed:function(ae){if(de!==ae){const De=e.get("EXT_clip_control");ae?De.clipControlEXT(De.LOWER_LEFT_EXT,De.ZERO_TO_ONE_EXT):De.clipControlEXT(De.LOWER_LEFT_EXT,De.NEGATIVE_ONE_TO_ONE_EXT),de=ae;const We=pe;pe=null,this.setClear(We)}},getReversed:function(){return de},setTest:function(ae){ae?re(n.DEPTH_TEST):Me(n.DEPTH_TEST)},setMask:function(ae){ee!==ae&&!P&&(n.depthMask(ae),ee=ae)},setFunc:function(ae){if(de&&(ae=Ud[ae]),ye!==ae){switch(ae){case ho:n.depthFunc(n.NEVER);break;case uo:n.depthFunc(n.ALWAYS);break;case fo:n.depthFunc(n.LESS);break;case ps:n.depthFunc(n.LEQUAL);break;case po:n.depthFunc(n.EQUAL);break;case mo:n.depthFunc(n.GEQUAL);break;case go:n.depthFunc(n.GREATER);break;case _o:n.depthFunc(n.NOTEQUAL);break;default:n.depthFunc(n.LEQUAL)}ye=ae}},setLocked:function(ae){P=ae},setClear:function(ae){pe!==ae&&(pe=ae,de&&(ae=1-ae),n.clearDepth(ae))},reset:function(){P=!1,ee=null,ye=null,pe=null,de=!1}}}function s(){let P=!1,de=null,ee=null,ye=null,pe=null,ae=null,De=null,We=null,vt=null;return{setTest:function(ot){P||(ot?re(n.STENCIL_TEST):Me(n.STENCIL_TEST))},setMask:function(ot){de!==ot&&!P&&(n.stencilMask(ot),de=ot)},setFunc:function(ot,Dn,vn){(ee!==ot||ye!==Dn||pe!==vn)&&(n.stencilFunc(ot,Dn,vn),ee=ot,ye=Dn,pe=vn)},setOp:function(ot,Dn,vn){(ae!==ot||De!==Dn||We!==vn)&&(n.stencilOp(ot,Dn,vn),ae=ot,De=Dn,We=vn)},setLocked:function(ot){P=ot},setClear:function(ot){vt!==ot&&(n.clearStencil(ot),vt=ot)},reset:function(){P=!1,de=null,ee=null,ye=null,pe=null,ae=null,De=null,We=null,vt=null}}}const r=new t,a=new i,o=new s,c=new WeakMap,l=new WeakMap;let u={},g={},h={},m=new WeakMap,_=[],S=null,p=!1,d=null,y=null,A=null,T=null,L=null,b=null,I=null,M=new ke(0,0,0),w=0,B=!1,C=null,z=null,Z=null,J=null,O=null;const W=n.getParameter(n.MAX_COMBINED_TEXTURE_IMAGE_UNITS);let $=!1,se=0;const oe=n.getParameter(n.VERSION);oe.indexOf("WebGL")!==-1?(se=parseFloat(/^WebGL (\d)/.exec(oe)[1]),$=se>=1):oe.indexOf("OpenGL ES")!==-1&&(se=parseFloat(/^OpenGL ES (\d)/.exec(oe)[1]),$=se>=2);let he=null,Ee={};const Pe=n.getParameter(n.SCISSOR_BOX),Ge=n.getParameter(n.VIEWPORT),Je=new Et().fromArray(Pe),R=new Et().fromArray(Ge);function U(P,de,ee,ye){const pe=new Uint8Array(4),ae=n.createTexture();n.bindTexture(P,ae),n.texParameteri(P,n.TEXTURE_MIN_FILTER,n.NEAREST),n.texParameteri(P,n.TEXTURE_MAG_FILTER,n.NEAREST);for(let De=0;De<ee;De++)P===n.TEXTURE_3D||P===n.TEXTURE_2D_ARRAY?n.texImage3D(de,0,n.RGBA,1,1,ye,0,n.RGBA,n.UNSIGNED_BYTE,pe):n.texImage2D(de+De,0,n.RGBA,1,1,0,n.RGBA,n.UNSIGNED_BYTE,pe);return ae}const ie={};ie[n.TEXTURE_2D]=U(n.TEXTURE_2D,n.TEXTURE_2D,1),ie[n.TEXTURE_CUBE_MAP]=U(n.TEXTURE_CUBE_MAP,n.TEXTURE_CUBE_MAP_POSITIVE_X,6),ie[n.TEXTURE_2D_ARRAY]=U(n.TEXTURE_2D_ARRAY,n.TEXTURE_2D_ARRAY,1,1),ie[n.TEXTURE_3D]=U(n.TEXTURE_3D,n.TEXTURE_3D,1,1),r.setClear(0,0,0,1),a.setClear(1),o.setClear(0),re(n.DEPTH_TEST),a.setFunc(ps),ft(!1),G(ec),re(n.CULL_FACE),rt(Hn);function re(P){u[P]!==!0&&(n.enable(P),u[P]=!0)}function Me(P){u[P]!==!1&&(n.disable(P),u[P]=!1)}function Se(P,de){return h[P]!==de?(n.bindFramebuffer(P,de),h[P]=de,P===n.DRAW_FRAMEBUFFER&&(h[n.FRAMEBUFFER]=de),P===n.FRAMEBUFFER&&(h[n.DRAW_FRAMEBUFFER]=de),!0):!1}function Ce(P,de){let ee=_,ye=!1;if(P){ee=m.get(de),ee===void 0&&(ee=[],m.set(de,ee));const pe=P.textures;if(ee.length!==pe.length||ee[0]!==n.COLOR_ATTACHMENT0){for(let ae=0,De=pe.length;ae<De;ae++)ee[ae]=n.COLOR_ATTACHMENT0+ae;ee.length=pe.length,ye=!0}}else ee[0]!==n.BACK&&(ee[0]=n.BACK,ye=!0);ye&&n.drawBuffers(ee)}function Ke(P){return S!==P?(n.useProgram(P),S=P,!0):!1}const Fe={[Si]:n.FUNC_ADD,[id]:n.FUNC_SUBTRACT,[sd]:n.FUNC_REVERSE_SUBTRACT};Fe[rd]=n.MIN,Fe[ad]=n.MAX;const tt={[od]:n.ZERO,[ld]:n.ONE,[cd]:n.SRC_COLOR,[lo]:n.SRC_ALPHA,[md]:n.SRC_ALPHA_SATURATE,[fd]:n.DST_COLOR,[ud]:n.DST_ALPHA,[hd]:n.ONE_MINUS_SRC_COLOR,[co]:n.ONE_MINUS_SRC_ALPHA,[pd]:n.ONE_MINUS_DST_COLOR,[dd]:n.ONE_MINUS_DST_ALPHA,[gd]:n.CONSTANT_COLOR,[_d]:n.ONE_MINUS_CONSTANT_COLOR,[xd]:n.CONSTANT_ALPHA,[vd]:n.ONE_MINUS_CONSTANT_ALPHA};function rt(P,de,ee,ye,pe,ae,De,We,vt,ot){if(P===Hn){p===!0&&(Me(n.BLEND),p=!1);return}if(p===!1&&(re(n.BLEND),p=!0),P!==nd){if(P!==d||ot!==B){if((y!==Si||L!==Si)&&(n.blendEquation(n.FUNC_ADD),y=Si,L=Si),ot)switch(P){case hs:n.blendFuncSeparate(n.ONE,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case ea:n.blendFunc(n.ONE,n.ONE);break;case tc:n.blendFuncSeparate(n.ZERO,n.ONE_MINUS_SRC_COLOR,n.ZERO,n.ONE);break;case nc:n.blendFuncSeparate(n.DST_COLOR,n.ONE_MINUS_SRC_ALPHA,n.ZERO,n.ONE);break;default:Qe("WebGLState: Invalid blending: ",P);break}else switch(P){case hs:n.blendFuncSeparate(n.SRC_ALPHA,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case ea:n.blendFuncSeparate(n.SRC_ALPHA,n.ONE,n.ONE,n.ONE);break;case tc:Qe("WebGLState: SubtractiveBlending requires material.premultipliedAlpha = true");break;case nc:Qe("WebGLState: MultiplyBlending requires material.premultipliedAlpha = true");break;default:Qe("WebGLState: Invalid blending: ",P);break}A=null,T=null,b=null,I=null,M.set(0,0,0),w=0,d=P,B=ot}return}pe=pe||de,ae=ae||ee,De=De||ye,(de!==y||pe!==L)&&(n.blendEquationSeparate(Fe[de],Fe[pe]),y=de,L=pe),(ee!==A||ye!==T||ae!==b||De!==I)&&(n.blendFuncSeparate(tt[ee],tt[ye],tt[ae],tt[De]),A=ee,T=ye,b=ae,I=De),(We.equals(M)===!1||vt!==w)&&(n.blendColor(We.r,We.g,We.b,vt),M.copy(We),w=vt),d=P,B=!1}function Ve(P,de){P.side===Bn?Me(n.CULL_FACE):re(n.CULL_FACE);let ee=P.side===Kt;de&&(ee=!ee),ft(ee),P.blending===hs&&P.transparent===!1?rt(Hn):rt(P.blending,P.blendEquation,P.blendSrc,P.blendDst,P.blendEquationAlpha,P.blendSrcAlpha,P.blendDstAlpha,P.blendColor,P.blendAlpha,P.premultipliedAlpha),a.setFunc(P.depthFunc),a.setTest(P.depthTest),a.setMask(P.depthWrite),r.setMask(P.colorWrite);const ye=P.stencilWrite;o.setTest(ye),ye&&(o.setMask(P.stencilWriteMask),o.setFunc(P.stencilFunc,P.stencilRef,P.stencilFuncMask),o.setOp(P.stencilFail,P.stencilZFail,P.stencilZPass)),E(P.polygonOffset,P.polygonOffsetFactor,P.polygonOffsetUnits),P.alphaToCoverage===!0?re(n.SAMPLE_ALPHA_TO_COVERAGE):Me(n.SAMPLE_ALPHA_TO_COVERAGE)}function ft(P){C!==P&&(P?n.frontFace(n.CW):n.frontFace(n.CCW),C=P)}function G(P){P!==Qu?(re(n.CULL_FACE),P!==z&&(P===ec?n.cullFace(n.BACK):P===ed?n.cullFace(n.FRONT):n.cullFace(n.FRONT_AND_BACK))):Me(n.CULL_FACE),z=P}function j(P){P!==Z&&($&&n.lineWidth(P),Z=P)}function E(P,de,ee){P?(re(n.POLYGON_OFFSET_FILL),(J!==de||O!==ee)&&(J=de,O=ee,a.getReversed()&&(de=-de),n.polygonOffset(de,ee))):Me(n.POLYGON_OFFSET_FILL)}function ce(P){P?re(n.SCISSOR_TEST):Me(n.SCISSOR_TEST)}function le(P){P===void 0&&(P=n.TEXTURE0+W-1),he!==P&&(n.activeTexture(P),he=P)}function me(P,de,ee){ee===void 0&&(he===null?ee=n.TEXTURE0+W-1:ee=he);let ye=Ee[ee];ye===void 0&&(ye={type:void 0,texture:void 0},Ee[ee]=ye),(ye.type!==P||ye.texture!==de)&&(he!==ee&&(n.activeTexture(ee),he=ee),n.bindTexture(P,de||ie[P]),ye.type=P,ye.texture=de)}function K(){const P=Ee[he];P!==void 0&&P.type!==void 0&&(n.bindTexture(P.type,null),P.type=void 0,P.texture=void 0)}function Re(){try{n.compressedTexImage2D(...arguments)}catch(P){Qe("WebGLState:",P)}}function v(){try{n.compressedTexImage3D(...arguments)}catch(P){Qe("WebGLState:",P)}}function f(){try{n.texSubImage2D(...arguments)}catch(P){Qe("WebGLState:",P)}}function D(){try{n.texSubImage3D(...arguments)}catch(P){Qe("WebGLState:",P)}}function q(){try{n.compressedTexSubImage2D(...arguments)}catch(P){Qe("WebGLState:",P)}}function ne(){try{n.compressedTexSubImage3D(...arguments)}catch(P){Qe("WebGLState:",P)}}function V(){try{n.texStorage2D(...arguments)}catch(P){Qe("WebGLState:",P)}}function te(){try{n.texStorage3D(...arguments)}catch(P){Qe("WebGLState:",P)}}function Y(){try{n.texImage2D(...arguments)}catch(P){Qe("WebGLState:",P)}}function Q(){try{n.texImage3D(...arguments)}catch(P){Qe("WebGLState:",P)}}function ve(P){return g[P]!==void 0?g[P]:n.getParameter(P)}function Te(P,de){g[P]!==de&&(n.pixelStorei(P,de),g[P]=de)}function fe(P){Je.equals(P)===!1&&(n.scissor(P.x,P.y,P.z,P.w),Je.copy(P))}function ue(P){R.equals(P)===!1&&(n.viewport(P.x,P.y,P.z,P.w),R.copy(P))}function Be(P,de){let ee=l.get(de);ee===void 0&&(ee=new WeakMap,l.set(de,ee));let ye=ee.get(P);ye===void 0&&(ye=n.getUniformBlockIndex(de,P.name),ee.set(P,ye))}function $e(P,de){const ye=l.get(de).get(P);c.get(de)!==ye&&(n.uniformBlockBinding(de,ye,P.__bindingPointIndex),c.set(de,ye))}function it(){n.disable(n.BLEND),n.disable(n.CULL_FACE),n.disable(n.DEPTH_TEST),n.disable(n.POLYGON_OFFSET_FILL),n.disable(n.SCISSOR_TEST),n.disable(n.STENCIL_TEST),n.disable(n.SAMPLE_ALPHA_TO_COVERAGE),n.blendEquation(n.FUNC_ADD),n.blendFunc(n.ONE,n.ZERO),n.blendFuncSeparate(n.ONE,n.ZERO,n.ONE,n.ZERO),n.blendColor(0,0,0,0),n.colorMask(!0,!0,!0,!0),n.clearColor(0,0,0,0),n.depthMask(!0),n.depthFunc(n.LESS),a.setReversed(!1),n.clearDepth(1),n.stencilMask(4294967295),n.stencilFunc(n.ALWAYS,0,4294967295),n.stencilOp(n.KEEP,n.KEEP,n.KEEP),n.clearStencil(0),n.cullFace(n.BACK),n.frontFace(n.CCW),n.polygonOffset(0,0),n.activeTexture(n.TEXTURE0),n.bindFramebuffer(n.FRAMEBUFFER,null),n.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),n.bindFramebuffer(n.READ_FRAMEBUFFER,null),n.useProgram(null),n.lineWidth(1),n.scissor(0,0,n.canvas.width,n.canvas.height),n.viewport(0,0,n.canvas.width,n.canvas.height),n.pixelStorei(n.PACK_ALIGNMENT,4),n.pixelStorei(n.UNPACK_ALIGNMENT,4),n.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,!1),n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!1),n.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,n.BROWSER_DEFAULT_WEBGL),n.pixelStorei(n.PACK_ROW_LENGTH,0),n.pixelStorei(n.PACK_SKIP_PIXELS,0),n.pixelStorei(n.PACK_SKIP_ROWS,0),n.pixelStorei(n.UNPACK_ROW_LENGTH,0),n.pixelStorei(n.UNPACK_IMAGE_HEIGHT,0),n.pixelStorei(n.UNPACK_SKIP_PIXELS,0),n.pixelStorei(n.UNPACK_SKIP_ROWS,0),n.pixelStorei(n.UNPACK_SKIP_IMAGES,0),u={},g={},he=null,Ee={},h={},m=new WeakMap,_=[],S=null,p=!1,d=null,y=null,A=null,T=null,L=null,b=null,I=null,M=new ke(0,0,0),w=0,B=!1,C=null,z=null,Z=null,J=null,O=null,Je.set(0,0,n.canvas.width,n.canvas.height),R.set(0,0,n.canvas.width,n.canvas.height),r.reset(),a.reset(),o.reset()}return{buffers:{color:r,depth:a,stencil:o},enable:re,disable:Me,bindFramebuffer:Se,drawBuffers:Ce,useProgram:Ke,setBlending:rt,setMaterial:Ve,setFlipSided:ft,setCullFace:G,setLineWidth:j,setPolygonOffset:E,setScissorTest:ce,activeTexture:le,bindTexture:me,unbindTexture:K,compressedTexImage2D:Re,compressedTexImage3D:v,texImage2D:Y,texImage3D:Q,pixelStorei:Te,getParameter:ve,updateUBOMapping:Be,uniformBlockBinding:$e,texStorage2D:V,texStorage3D:te,texSubImage2D:f,texSubImage3D:D,compressedTexSubImage2D:q,compressedTexSubImage3D:ne,scissor:fe,viewport:ue,reset:it}}function r0(n,e,t,i,s,r,a){const o=e.has("WEBGL_multisampled_render_to_texture")?e.get("WEBGL_multisampled_render_to_texture"):null,c=typeof navigator>"u"?!1:/OculusBrowser/g.test(navigator.userAgent),l=new Oe,u=new WeakMap,g=new Set;let h;const m=new WeakMap;let _=!1;try{_=typeof OffscreenCanvas<"u"&&new OffscreenCanvas(1,1).getContext("2d")!==null}catch{}function S(v,f){return _?new OffscreenCanvas(v,f):aa("canvas")}function p(v,f,D){let q=1;const ne=Re(v);if((ne.width>D||ne.height>D)&&(q=D/Math.max(ne.width,ne.height)),q<1)if(typeof HTMLImageElement<"u"&&v instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&v instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&v instanceof ImageBitmap||typeof VideoFrame<"u"&&v instanceof VideoFrame){const V=Math.floor(q*ne.width),te=Math.floor(q*ne.height);h===void 0&&(h=S(V,te));const Y=f?S(V,te):h;return Y.width=V,Y.height=te,Y.getContext("2d").drawImage(v,0,0,V,te),Ne("WebGLRenderer: Texture has been resized from ("+ne.width+"x"+ne.height+") to ("+V+"x"+te+")."),Y}else return"data"in v&&Ne("WebGLRenderer: Image in DataTexture is too big ("+ne.width+"x"+ne.height+")."),v;return v}function d(v){return v.generateMipmaps}function y(v){n.generateMipmap(v)}function A(v){return v.isWebGLCubeRenderTarget?n.TEXTURE_CUBE_MAP:v.isWebGL3DRenderTarget?n.TEXTURE_3D:v.isWebGLArrayRenderTarget||v.isCompressedArrayTexture?n.TEXTURE_2D_ARRAY:n.TEXTURE_2D}function T(v,f,D,q,ne,V=!1){if(v!==null){if(n[v]!==void 0)return n[v];Ne("WebGLRenderer: Attempt to use non-existing WebGL internal format '"+v+"'")}let te;q&&(te=e.get("EXT_texture_norm16"),te||Ne("WebGLRenderer: Unable to use normalized textures without EXT_texture_norm16 extension"));let Y=f;if(f===n.RED&&(D===n.FLOAT&&(Y=n.R32F),D===n.HALF_FLOAT&&(Y=n.R16F),D===n.UNSIGNED_BYTE&&(Y=n.R8),D===n.UNSIGNED_SHORT&&te&&(Y=te.R16_EXT),D===n.SHORT&&te&&(Y=te.R16_SNORM_EXT)),f===n.RED_INTEGER&&(D===n.UNSIGNED_BYTE&&(Y=n.R8UI),D===n.UNSIGNED_SHORT&&(Y=n.R16UI),D===n.UNSIGNED_INT&&(Y=n.R32UI),D===n.BYTE&&(Y=n.R8I),D===n.SHORT&&(Y=n.R16I),D===n.INT&&(Y=n.R32I)),f===n.RG&&(D===n.FLOAT&&(Y=n.RG32F),D===n.HALF_FLOAT&&(Y=n.RG16F),D===n.UNSIGNED_BYTE&&(Y=n.RG8),D===n.UNSIGNED_SHORT&&te&&(Y=te.RG16_EXT),D===n.SHORT&&te&&(Y=te.RG16_SNORM_EXT)),f===n.RG_INTEGER&&(D===n.UNSIGNED_BYTE&&(Y=n.RG8UI),D===n.UNSIGNED_SHORT&&(Y=n.RG16UI),D===n.UNSIGNED_INT&&(Y=n.RG32UI),D===n.BYTE&&(Y=n.RG8I),D===n.SHORT&&(Y=n.RG16I),D===n.INT&&(Y=n.RG32I)),f===n.RGB_INTEGER&&(D===n.UNSIGNED_BYTE&&(Y=n.RGB8UI),D===n.UNSIGNED_SHORT&&(Y=n.RGB16UI),D===n.UNSIGNED_INT&&(Y=n.RGB32UI),D===n.BYTE&&(Y=n.RGB8I),D===n.SHORT&&(Y=n.RGB16I),D===n.INT&&(Y=n.RGB32I)),f===n.RGBA_INTEGER&&(D===n.UNSIGNED_BYTE&&(Y=n.RGBA8UI),D===n.UNSIGNED_SHORT&&(Y=n.RGBA16UI),D===n.UNSIGNED_INT&&(Y=n.RGBA32UI),D===n.BYTE&&(Y=n.RGBA8I),D===n.SHORT&&(Y=n.RGBA16I),D===n.INT&&(Y=n.RGBA32I)),f===n.RGB&&(D===n.UNSIGNED_SHORT&&te&&(Y=te.RGB16_EXT),D===n.SHORT&&te&&(Y=te.RGB16_SNORM_EXT),D===n.UNSIGNED_INT_5_9_9_9_REV&&(Y=n.RGB9_E5),D===n.UNSIGNED_INT_10F_11F_11F_REV&&(Y=n.R11F_G11F_B10F)),f===n.RGBA){const Q=V?sa:Ze.getTransfer(ne);D===n.FLOAT&&(Y=n.RGBA32F),D===n.HALF_FLOAT&&(Y=n.RGBA16F),D===n.UNSIGNED_BYTE&&(Y=Q===at?n.SRGB8_ALPHA8:n.RGBA8),D===n.UNSIGNED_SHORT&&te&&(Y=te.RGBA16_EXT),D===n.SHORT&&te&&(Y=te.RGBA16_SNORM_EXT),D===n.UNSIGNED_SHORT_4_4_4_4&&(Y=n.RGBA4),D===n.UNSIGNED_SHORT_5_5_5_1&&(Y=n.RGB5_A1)}return(Y===n.R16F||Y===n.R32F||Y===n.RG16F||Y===n.RG32F||Y===n.RGBA16F||Y===n.RGBA32F)&&e.get("EXT_color_buffer_float"),Y}function L(v,f){let D;return v?f===null||f===Ln||f===Hs?D=n.DEPTH24_STENCIL8:f===An?D=n.DEPTH32F_STENCIL8:f===ks&&(D=n.DEPTH24_STENCIL8,Ne("DepthTexture: 16 bit depth attachment is not supported with stencil. Using 24-bit attachment.")):f===null||f===Ln||f===Hs?D=n.DEPTH_COMPONENT24:f===An?D=n.DEPTH_COMPONENT32F:f===ks&&(D=n.DEPTH_COMPONENT16),D}function b(v,f){return d(v)===!0||v.isFramebufferTexture&&v.minFilter!==Lt&&v.minFilter!==Bt?Math.log2(Math.max(f.width,f.height))+1:v.mipmaps!==void 0&&v.mipmaps.length>0?v.mipmaps.length:v.isCompressedTexture&&Array.isArray(v.image)?f.mipmaps.length:1}function I(v){const f=v.target;f.removeEventListener("dispose",I),w(f),f.isVideoTexture&&u.delete(f),f.isHTMLTexture&&g.delete(f)}function M(v){const f=v.target;f.removeEventListener("dispose",M),C(f)}function w(v){const f=i.get(v);if(f.__webglInit===void 0)return;const D=v.source,q=m.get(D);if(q){const ne=q[f.__cacheKey];ne.usedTimes--,ne.usedTimes===0&&B(v),Object.keys(q).length===0&&m.delete(D)}i.remove(v)}function B(v){const f=i.get(v);n.deleteTexture(f.__webglTexture);const D=v.source,q=m.get(D);delete q[f.__cacheKey],a.memory.textures--}function C(v){const f=i.get(v);if(v.depthTexture&&(v.depthTexture.dispose(),i.remove(v.depthTexture)),v.isWebGLCubeRenderTarget)for(let q=0;q<6;q++){if(Array.isArray(f.__webglFramebuffer[q]))for(let ne=0;ne<f.__webglFramebuffer[q].length;ne++)n.deleteFramebuffer(f.__webglFramebuffer[q][ne]);else n.deleteFramebuffer(f.__webglFramebuffer[q]);f.__webglDepthbuffer&&n.deleteRenderbuffer(f.__webglDepthbuffer[q])}else{if(Array.isArray(f.__webglFramebuffer))for(let q=0;q<f.__webglFramebuffer.length;q++)n.deleteFramebuffer(f.__webglFramebuffer[q]);else n.deleteFramebuffer(f.__webglFramebuffer);if(f.__webglDepthbuffer&&n.deleteRenderbuffer(f.__webglDepthbuffer),f.__webglMultisampledFramebuffer&&n.deleteFramebuffer(f.__webglMultisampledFramebuffer),f.__webglColorRenderbuffer)for(let q=0;q<f.__webglColorRenderbuffer.length;q++)f.__webglColorRenderbuffer[q]&&n.deleteRenderbuffer(f.__webglColorRenderbuffer[q]);f.__webglDepthRenderbuffer&&n.deleteRenderbuffer(f.__webglDepthRenderbuffer)}const D=v.textures;for(let q=0,ne=D.length;q<ne;q++){const V=i.get(D[q]);V.__webglTexture&&(n.deleteTexture(V.__webglTexture),a.memory.textures--),i.remove(D[q])}i.remove(v)}let z=0;function Z(){z=0}function J(){return z}function O(v){z=v}function W(){const v=z;return v>=s.maxTextures&&Ne("WebGLTextures: Trying to use "+v+" texture units while this GPU supports only "+s.maxTextures),z+=1,v}function $(v){const f=[];return f.push(v.wrapS),f.push(v.wrapT),f.push(v.wrapR||0),f.push(v.magFilter),f.push(v.minFilter),f.push(v.anisotropy),f.push(v.internalFormat),f.push(v.format),f.push(v.type),f.push(v.generateMipmaps),f.push(v.premultiplyAlpha),f.push(v.flipY),f.push(v.unpackAlignment),f.push(v.colorSpace),f.join()}function se(v,f){const D=i.get(v);if(v.isVideoTexture&&me(v),v.isRenderTargetTexture===!1&&v.isExternalTexture!==!0&&v.version>0&&D.__version!==v.version){const q=v.image;if(q===null)Ne("WebGLRenderer: Texture marked for update but no image data found.");else if(q.complete===!1)Ne("WebGLRenderer: Texture marked for update but image is incomplete");else{Me(D,v,f);return}}else v.isExternalTexture&&(D.__webglTexture=v.sourceTexture?v.sourceTexture:null);t.bindTexture(n.TEXTURE_2D,D.__webglTexture,n.TEXTURE0+f)}function oe(v,f){const D=i.get(v);if(v.isRenderTargetTexture===!1&&v.version>0&&D.__version!==v.version){Me(D,v,f);return}else v.isExternalTexture&&(D.__webglTexture=v.sourceTexture?v.sourceTexture:null);t.bindTexture(n.TEXTURE_2D_ARRAY,D.__webglTexture,n.TEXTURE0+f)}function he(v,f){const D=i.get(v);if(v.isRenderTargetTexture===!1&&v.version>0&&D.__version!==v.version){Me(D,v,f);return}t.bindTexture(n.TEXTURE_3D,D.__webglTexture,n.TEXTURE0+f)}function Ee(v,f){const D=i.get(v);if(v.isCubeDepthTexture!==!0&&v.version>0&&D.__version!==v.version){Se(D,v,f);return}t.bindTexture(n.TEXTURE_CUBE_MAP,D.__webglTexture,n.TEXTURE0+f)}const Pe={[xo]:n.REPEAT,[kn]:n.CLAMP_TO_EDGE,[vo]:n.MIRRORED_REPEAT},Ge={[Lt]:n.NEAREST,[yd]:n.NEAREST_MIPMAP_NEAREST,[tr]:n.NEAREST_MIPMAP_LINEAR,[Bt]:n.LINEAR,[wa]:n.LINEAR_MIPMAP_NEAREST,[Ti]:n.LINEAR_MIPMAP_LINEAR},Je={[Td]:n.NEVER,[Pd]:n.ALWAYS,[Ad]:n.LESS,[yl]:n.LEQUAL,[wd]:n.EQUAL,[El]:n.GEQUAL,[Rd]:n.GREATER,[Cd]:n.NOTEQUAL};function R(v,f){if(f.type===An&&e.has("OES_texture_float_linear")===!1&&(f.magFilter===Bt||f.magFilter===wa||f.magFilter===tr||f.magFilter===Ti||f.minFilter===Bt||f.minFilter===wa||f.minFilter===tr||f.minFilter===Ti)&&Ne("WebGLRenderer: Unable to use linear filtering with floating point textures. OES_texture_float_linear not supported on this device."),n.texParameteri(v,n.TEXTURE_WRAP_S,Pe[f.wrapS]),n.texParameteri(v,n.TEXTURE_WRAP_T,Pe[f.wrapT]),(v===n.TEXTURE_3D||v===n.TEXTURE_2D_ARRAY)&&n.texParameteri(v,n.TEXTURE_WRAP_R,Pe[f.wrapR]),n.texParameteri(v,n.TEXTURE_MAG_FILTER,Ge[f.magFilter]),n.texParameteri(v,n.TEXTURE_MIN_FILTER,Ge[f.minFilter]),f.compareFunction&&(n.texParameteri(v,n.TEXTURE_COMPARE_MODE,n.COMPARE_REF_TO_TEXTURE),n.texParameteri(v,n.TEXTURE_COMPARE_FUNC,Je[f.compareFunction])),e.has("EXT_texture_filter_anisotropic")===!0){if(f.magFilter===Lt||f.minFilter!==tr&&f.minFilter!==Ti||f.type===An&&e.has("OES_texture_float_linear")===!1)return;if(f.anisotropy>1||i.get(f).__currentAnisotropy){const D=e.get("EXT_texture_filter_anisotropic");n.texParameterf(v,D.TEXTURE_MAX_ANISOTROPY_EXT,Math.min(f.anisotropy,s.getMaxAnisotropy())),i.get(f).__currentAnisotropy=f.anisotropy}}}function U(v,f){let D=!1;v.__webglInit===void 0&&(v.__webglInit=!0,f.addEventListener("dispose",I));const q=f.source;let ne=m.get(q);ne===void 0&&(ne={},m.set(q,ne));const V=$(f);if(V!==v.__cacheKey){ne[V]===void 0&&(ne[V]={texture:n.createTexture(),usedTimes:0},a.memory.textures++,D=!0),ne[V].usedTimes++;const te=ne[v.__cacheKey];te!==void 0&&(ne[v.__cacheKey].usedTimes--,te.usedTimes===0&&B(f)),v.__cacheKey=V,v.__webglTexture=ne[V].texture}return D}function ie(v,f,D){return Math.floor(Math.floor(v/D)/f)}function re(v,f,D,q){const V=v.updateRanges;if(V.length===0)t.texSubImage2D(n.TEXTURE_2D,0,0,0,f.width,f.height,D,q,f.data);else{V.sort((Te,fe)=>Te.start-fe.start);let te=0;for(let Te=1;Te<V.length;Te++){const fe=V[te],ue=V[Te],Be=fe.start+fe.count,$e=ie(ue.start,f.width,4),it=ie(fe.start,f.width,4);ue.start<=Be+1&&$e===it&&ie(ue.start+ue.count-1,f.width,4)===$e?fe.count=Math.max(fe.count,ue.start+ue.count-fe.start):(++te,V[te]=ue)}V.length=te+1;const Y=t.getParameter(n.UNPACK_ROW_LENGTH),Q=t.getParameter(n.UNPACK_SKIP_PIXELS),ve=t.getParameter(n.UNPACK_SKIP_ROWS);t.pixelStorei(n.UNPACK_ROW_LENGTH,f.width);for(let Te=0,fe=V.length;Te<fe;Te++){const ue=V[Te],Be=Math.floor(ue.start/4),$e=Math.ceil(ue.count/4),it=Be%f.width,P=Math.floor(Be/f.width),de=$e,ee=1;t.pixelStorei(n.UNPACK_SKIP_PIXELS,it),t.pixelStorei(n.UNPACK_SKIP_ROWS,P),t.texSubImage2D(n.TEXTURE_2D,0,it,P,de,ee,D,q,f.data)}v.clearUpdateRanges(),t.pixelStorei(n.UNPACK_ROW_LENGTH,Y),t.pixelStorei(n.UNPACK_SKIP_PIXELS,Q),t.pixelStorei(n.UNPACK_SKIP_ROWS,ve)}}function Me(v,f,D){let q=n.TEXTURE_2D;(f.isDataArrayTexture||f.isCompressedArrayTexture)&&(q=n.TEXTURE_2D_ARRAY),f.isData3DTexture&&(q=n.TEXTURE_3D);const ne=U(v,f),V=f.source;t.bindTexture(q,v.__webglTexture,n.TEXTURE0+D);const te=i.get(V);if(V.version!==te.__version||ne===!0){if(t.activeTexture(n.TEXTURE0+D),(typeof ImageBitmap<"u"&&f.image instanceof ImageBitmap)===!1){const ee=Ze.getPrimaries(Ze.workingColorSpace),ye=f.colorSpace===si?null:Ze.getPrimaries(f.colorSpace),pe=f.colorSpace===si||ee===ye?n.NONE:n.BROWSER_DEFAULT_WEBGL;t.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,f.flipY),t.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,f.premultiplyAlpha),t.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,pe)}t.pixelStorei(n.UNPACK_ALIGNMENT,f.unpackAlignment);let Q=p(f.image,!1,s.maxTextureSize);Q=K(f,Q);const ve=r.convert(f.format,f.colorSpace),Te=r.convert(f.type);let fe=T(f.internalFormat,ve,Te,f.normalized,f.colorSpace,f.isVideoTexture);R(q,f);let ue;const Be=f.mipmaps,$e=f.isVideoTexture!==!0,it=te.__version===void 0||ne===!0,P=V.dataReady,de=b(f,Q);if(f.isDepthTexture)fe=L(f.format===Ai,f.type),it&&($e?t.texStorage2D(n.TEXTURE_2D,1,fe,Q.width,Q.height):t.texImage2D(n.TEXTURE_2D,0,fe,Q.width,Q.height,0,ve,Te,null));else if(f.isDataTexture)if(Be.length>0){$e&&it&&t.texStorage2D(n.TEXTURE_2D,de,fe,Be[0].width,Be[0].height);for(let ee=0,ye=Be.length;ee<ye;ee++)ue=Be[ee],$e?P&&t.texSubImage2D(n.TEXTURE_2D,ee,0,0,ue.width,ue.height,ve,Te,ue.data):t.texImage2D(n.TEXTURE_2D,ee,fe,ue.width,ue.height,0,ve,Te,ue.data);f.generateMipmaps=!1}else $e?(it&&t.texStorage2D(n.TEXTURE_2D,de,fe,Q.width,Q.height),P&&re(f,Q,ve,Te)):t.texImage2D(n.TEXTURE_2D,0,fe,Q.width,Q.height,0,ve,Te,Q.data);else if(f.isCompressedTexture)if(f.isCompressedArrayTexture){$e&&it&&t.texStorage3D(n.TEXTURE_2D_ARRAY,de,fe,Be[0].width,Be[0].height,Q.depth);for(let ee=0,ye=Be.length;ee<ye;ee++)if(ue=Be[ee],f.format!==pn)if(ve!==null)if($e){if(P)if(f.layerUpdates.size>0){const pe=Ic(ue.width,ue.height,f.format,f.type);for(const ae of f.layerUpdates){const De=ue.data.subarray(ae*pe/ue.data.BYTES_PER_ELEMENT,(ae+1)*pe/ue.data.BYTES_PER_ELEMENT);t.compressedTexSubImage3D(n.TEXTURE_2D_ARRAY,ee,0,0,ae,ue.width,ue.height,1,ve,De)}f.clearLayerUpdates()}else t.compressedTexSubImage3D(n.TEXTURE_2D_ARRAY,ee,0,0,0,ue.width,ue.height,Q.depth,ve,ue.data)}else t.compressedTexImage3D(n.TEXTURE_2D_ARRAY,ee,fe,ue.width,ue.height,Q.depth,0,ue.data,0,0);else Ne("WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()");else $e?P&&t.texSubImage3D(n.TEXTURE_2D_ARRAY,ee,0,0,0,ue.width,ue.height,Q.depth,ve,Te,ue.data):t.texImage3D(n.TEXTURE_2D_ARRAY,ee,fe,ue.width,ue.height,Q.depth,0,ve,Te,ue.data)}else{$e&&it&&t.texStorage2D(n.TEXTURE_2D,de,fe,Be[0].width,Be[0].height);for(let ee=0,ye=Be.length;ee<ye;ee++)ue=Be[ee],f.format!==pn?ve!==null?$e?P&&t.compressedTexSubImage2D(n.TEXTURE_2D,ee,0,0,ue.width,ue.height,ve,ue.data):t.compressedTexImage2D(n.TEXTURE_2D,ee,fe,ue.width,ue.height,0,ue.data):Ne("WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()"):$e?P&&t.texSubImage2D(n.TEXTURE_2D,ee,0,0,ue.width,ue.height,ve,Te,ue.data):t.texImage2D(n.TEXTURE_2D,ee,fe,ue.width,ue.height,0,ve,Te,ue.data)}else if(f.isDataArrayTexture)if($e){if(it&&t.texStorage3D(n.TEXTURE_2D_ARRAY,de,fe,Q.width,Q.height,Q.depth),P)if(f.layerUpdates.size>0){const ee=Ic(Q.width,Q.height,f.format,f.type);for(const ye of f.layerUpdates){const pe=Q.data.subarray(ye*ee/Q.data.BYTES_PER_ELEMENT,(ye+1)*ee/Q.data.BYTES_PER_ELEMENT);t.texSubImage3D(n.TEXTURE_2D_ARRAY,0,0,0,ye,Q.width,Q.height,1,ve,Te,pe)}f.clearLayerUpdates()}else t.texSubImage3D(n.TEXTURE_2D_ARRAY,0,0,0,0,Q.width,Q.height,Q.depth,ve,Te,Q.data)}else t.texImage3D(n.TEXTURE_2D_ARRAY,0,fe,Q.width,Q.height,Q.depth,0,ve,Te,Q.data);else if(f.isData3DTexture)$e?(it&&t.texStorage3D(n.TEXTURE_3D,de,fe,Q.width,Q.height,Q.depth),P&&t.texSubImage3D(n.TEXTURE_3D,0,0,0,0,Q.width,Q.height,Q.depth,ve,Te,Q.data)):t.texImage3D(n.TEXTURE_3D,0,fe,Q.width,Q.height,Q.depth,0,ve,Te,Q.data);else if(f.isFramebufferTexture){if(it)if($e)t.texStorage2D(n.TEXTURE_2D,de,fe,Q.width,Q.height);else{let ee=Q.width,ye=Q.height;for(let pe=0;pe<de;pe++)t.texImage2D(n.TEXTURE_2D,pe,fe,ee,ye,0,ve,Te,null),ee>>=1,ye>>=1}}else if(f.isHTMLTexture){if("texElementImage2D"in n){const ee=n.canvas;if(ee.hasAttribute("layoutsubtree")||ee.setAttribute("layoutsubtree","true"),Q.parentNode!==ee){ee.appendChild(Q),g.add(f),ee.onpaint=We=>{const vt=We.changedElements;for(const ot of g)vt.includes(ot.image)&&(ot.needsUpdate=!0)},ee.requestPaint();return}const ye=0,pe=n.RGBA,ae=n.RGBA,De=n.UNSIGNED_BYTE;n.texElementImage2D(n.TEXTURE_2D,ye,pe,ae,De,Q),n.texParameteri(n.TEXTURE_2D,n.TEXTURE_MIN_FILTER,n.LINEAR),n.texParameteri(n.TEXTURE_2D,n.TEXTURE_WRAP_S,n.CLAMP_TO_EDGE),n.texParameteri(n.TEXTURE_2D,n.TEXTURE_WRAP_T,n.CLAMP_TO_EDGE)}}else if(Be.length>0){if($e&&it){const ee=Re(Be[0]);t.texStorage2D(n.TEXTURE_2D,de,fe,ee.width,ee.height)}for(let ee=0,ye=Be.length;ee<ye;ee++)ue=Be[ee],$e?P&&t.texSubImage2D(n.TEXTURE_2D,ee,0,0,ve,Te,ue):t.texImage2D(n.TEXTURE_2D,ee,fe,ve,Te,ue);f.generateMipmaps=!1}else if($e){if(it){const ee=Re(Q);t.texStorage2D(n.TEXTURE_2D,de,fe,ee.width,ee.height)}P&&t.texSubImage2D(n.TEXTURE_2D,0,0,0,ve,Te,Q)}else t.texImage2D(n.TEXTURE_2D,0,fe,ve,Te,Q);d(f)&&y(q),te.__version=V.version,f.onUpdate&&f.onUpdate(f)}v.__version=f.version}function Se(v,f,D){if(f.image.length!==6)return;const q=U(v,f),ne=f.source;t.bindTexture(n.TEXTURE_CUBE_MAP,v.__webglTexture,n.TEXTURE0+D);const V=i.get(ne);if(ne.version!==V.__version||q===!0){t.activeTexture(n.TEXTURE0+D);const te=Ze.getPrimaries(Ze.workingColorSpace),Y=f.colorSpace===si?null:Ze.getPrimaries(f.colorSpace),Q=f.colorSpace===si||te===Y?n.NONE:n.BROWSER_DEFAULT_WEBGL;t.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,f.flipY),t.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,f.premultiplyAlpha),t.pixelStorei(n.UNPACK_ALIGNMENT,f.unpackAlignment),t.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,Q);const ve=f.isCompressedTexture||f.image[0].isCompressedTexture,Te=f.image[0]&&f.image[0].isDataTexture,fe=[];for(let ae=0;ae<6;ae++)!ve&&!Te?fe[ae]=p(f.image[ae],!0,s.maxCubemapSize):fe[ae]=Te?f.image[ae].image:f.image[ae],fe[ae]=K(f,fe[ae]);const ue=fe[0],Be=r.convert(f.format,f.colorSpace),$e=r.convert(f.type),it=T(f.internalFormat,Be,$e,f.normalized,f.colorSpace),P=f.isVideoTexture!==!0,de=V.__version===void 0||q===!0,ee=ne.dataReady;let ye=b(f,ue);R(n.TEXTURE_CUBE_MAP,f);let pe;if(ve){P&&de&&t.texStorage2D(n.TEXTURE_CUBE_MAP,ye,it,ue.width,ue.height);for(let ae=0;ae<6;ae++){pe=fe[ae].mipmaps;for(let De=0;De<pe.length;De++){const We=pe[De];f.format!==pn?Be!==null?P?ee&&t.compressedTexSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,De,0,0,We.width,We.height,Be,We.data):t.compressedTexImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,De,it,We.width,We.height,0,We.data):Ne("WebGLRenderer: Attempt to load unsupported compressed texture format in .setTextureCube()"):P?ee&&t.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,De,0,0,We.width,We.height,Be,$e,We.data):t.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,De,it,We.width,We.height,0,Be,$e,We.data)}}}else{if(pe=f.mipmaps,P&&de){pe.length>0&&ye++;const ae=Re(fe[0]);t.texStorage2D(n.TEXTURE_CUBE_MAP,ye,it,ae.width,ae.height)}for(let ae=0;ae<6;ae++)if(Te){P?ee&&t.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,0,0,0,fe[ae].width,fe[ae].height,Be,$e,fe[ae].data):t.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,0,it,fe[ae].width,fe[ae].height,0,Be,$e,fe[ae].data);for(let De=0;De<pe.length;De++){const vt=pe[De].image[ae].image;P?ee&&t.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,De+1,0,0,vt.width,vt.height,Be,$e,vt.data):t.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,De+1,it,vt.width,vt.height,0,Be,$e,vt.data)}}else{P?ee&&t.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,0,0,0,Be,$e,fe[ae]):t.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,0,it,Be,$e,fe[ae]);for(let De=0;De<pe.length;De++){const We=pe[De];P?ee&&t.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,De+1,0,0,Be,$e,We.image[ae]):t.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ae,De+1,it,Be,$e,We.image[ae])}}}d(f)&&y(n.TEXTURE_CUBE_MAP),V.__version=ne.version,f.onUpdate&&f.onUpdate(f)}v.__version=f.version}function Ce(v,f,D,q,ne,V){const te=r.convert(D.format,D.colorSpace),Y=r.convert(D.type),Q=T(D.internalFormat,te,Y,D.normalized,D.colorSpace),ve=i.get(f),Te=i.get(D);if(Te.__renderTarget=f,!ve.__hasExternalTextures){const fe=Math.max(1,f.width>>V),ue=Math.max(1,f.height>>V);ne===n.TEXTURE_3D||ne===n.TEXTURE_2D_ARRAY?t.texImage3D(ne,V,Q,fe,ue,f.depth,0,te,Y,null):t.texImage2D(ne,V,Q,fe,ue,0,te,Y,null)}t.bindFramebuffer(n.FRAMEBUFFER,v),le(f)?o.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,q,ne,Te.__webglTexture,0,ce(f)):(ne===n.TEXTURE_2D||ne>=n.TEXTURE_CUBE_MAP_POSITIVE_X&&ne<=n.TEXTURE_CUBE_MAP_NEGATIVE_Z)&&n.framebufferTexture2D(n.FRAMEBUFFER,q,ne,Te.__webglTexture,V),t.bindFramebuffer(n.FRAMEBUFFER,null)}function Ke(v,f,D){if(n.bindRenderbuffer(n.RENDERBUFFER,v),f.depthBuffer){const q=f.depthTexture,ne=q&&q.isDepthTexture?q.type:null,V=L(f.stencilBuffer,ne),te=f.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT;le(f)?o.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,ce(f),V,f.width,f.height):D?n.renderbufferStorageMultisample(n.RENDERBUFFER,ce(f),V,f.width,f.height):n.renderbufferStorage(n.RENDERBUFFER,V,f.width,f.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,te,n.RENDERBUFFER,v)}else{const q=f.textures;for(let ne=0;ne<q.length;ne++){const V=q[ne],te=r.convert(V.format,V.colorSpace),Y=r.convert(V.type),Q=T(V.internalFormat,te,Y,V.normalized,V.colorSpace);le(f)?o.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,ce(f),Q,f.width,f.height):D?n.renderbufferStorageMultisample(n.RENDERBUFFER,ce(f),Q,f.width,f.height):n.renderbufferStorage(n.RENDERBUFFER,Q,f.width,f.height)}}n.bindRenderbuffer(n.RENDERBUFFER,null)}function Fe(v,f,D){const q=f.isWebGLCubeRenderTarget===!0;if(t.bindFramebuffer(n.FRAMEBUFFER,v),!(f.depthTexture&&f.depthTexture.isDepthTexture))throw new Error("renderTarget.depthTexture must be an instance of THREE.DepthTexture");const ne=i.get(f.depthTexture);if(ne.__renderTarget=f,(!ne.__webglTexture||f.depthTexture.image.width!==f.width||f.depthTexture.image.height!==f.height)&&(f.depthTexture.image.width=f.width,f.depthTexture.image.height=f.height,f.depthTexture.needsUpdate=!0),q){if(ne.__webglInit===void 0&&(ne.__webglInit=!0,f.depthTexture.addEventListener("dispose",I)),ne.__webglTexture===void 0){ne.__webglTexture=n.createTexture(),t.bindTexture(n.TEXTURE_CUBE_MAP,ne.__webglTexture),R(n.TEXTURE_CUBE_MAP,f.depthTexture);const ve=r.convert(f.depthTexture.format),Te=r.convert(f.depthTexture.type);let fe;f.depthTexture.format===Wn?fe=n.DEPTH_COMPONENT24:f.depthTexture.format===Ai&&(fe=n.DEPTH24_STENCIL8);for(let ue=0;ue<6;ue++)n.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+ue,0,fe,f.width,f.height,0,ve,Te,null)}}else se(f.depthTexture,0);const V=ne.__webglTexture,te=ce(f),Y=q?n.TEXTURE_CUBE_MAP_POSITIVE_X+D:n.TEXTURE_2D,Q=f.depthTexture.format===Ai?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT;if(f.depthTexture.format===Wn)le(f)?o.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,Q,Y,V,0,te):n.framebufferTexture2D(n.FRAMEBUFFER,Q,Y,V,0);else if(f.depthTexture.format===Ai)le(f)?o.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,Q,Y,V,0,te):n.framebufferTexture2D(n.FRAMEBUFFER,Q,Y,V,0);else throw new Error("Unknown depthTexture format")}function tt(v){const f=i.get(v),D=v.isWebGLCubeRenderTarget===!0;if(f.__boundDepthTexture!==v.depthTexture){const q=v.depthTexture;if(f.__depthDisposeCallback&&f.__depthDisposeCallback(),q){const ne=()=>{delete f.__boundDepthTexture,delete f.__depthDisposeCallback,q.removeEventListener("dispose",ne)};q.addEventListener("dispose",ne),f.__depthDisposeCallback=ne}f.__boundDepthTexture=q}if(v.depthTexture&&!f.__autoAllocateDepthBuffer)if(D)for(let q=0;q<6;q++)Fe(f.__webglFramebuffer[q],v,q);else{const q=v.texture.mipmaps;q&&q.length>0?Fe(f.__webglFramebuffer[0],v,0):Fe(f.__webglFramebuffer,v,0)}else if(D){f.__webglDepthbuffer=[];for(let q=0;q<6;q++)if(t.bindFramebuffer(n.FRAMEBUFFER,f.__webglFramebuffer[q]),f.__webglDepthbuffer[q]===void 0)f.__webglDepthbuffer[q]=n.createRenderbuffer(),Ke(f.__webglDepthbuffer[q],v,!1);else{const ne=v.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,V=f.__webglDepthbuffer[q];n.bindRenderbuffer(n.RENDERBUFFER,V),n.framebufferRenderbuffer(n.FRAMEBUFFER,ne,n.RENDERBUFFER,V)}}else{const q=v.texture.mipmaps;if(q&&q.length>0?t.bindFramebuffer(n.FRAMEBUFFER,f.__webglFramebuffer[0]):t.bindFramebuffer(n.FRAMEBUFFER,f.__webglFramebuffer),f.__webglDepthbuffer===void 0)f.__webglDepthbuffer=n.createRenderbuffer(),Ke(f.__webglDepthbuffer,v,!1);else{const ne=v.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,V=f.__webglDepthbuffer;n.bindRenderbuffer(n.RENDERBUFFER,V),n.framebufferRenderbuffer(n.FRAMEBUFFER,ne,n.RENDERBUFFER,V)}}t.bindFramebuffer(n.FRAMEBUFFER,null)}function rt(v,f,D){const q=i.get(v);f!==void 0&&Ce(q.__webglFramebuffer,v,v.texture,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,0),D!==void 0&&tt(v)}function Ve(v){const f=v.texture,D=i.get(v),q=i.get(f);v.addEventListener("dispose",M);const ne=v.textures,V=v.isWebGLCubeRenderTarget===!0,te=ne.length>1;if(te||(q.__webglTexture===void 0&&(q.__webglTexture=n.createTexture()),q.__version=f.version,a.memory.textures++),V){D.__webglFramebuffer=[];for(let Y=0;Y<6;Y++)if(f.mipmaps&&f.mipmaps.length>0){D.__webglFramebuffer[Y]=[];for(let Q=0;Q<f.mipmaps.length;Q++)D.__webglFramebuffer[Y][Q]=n.createFramebuffer()}else D.__webglFramebuffer[Y]=n.createFramebuffer()}else{if(f.mipmaps&&f.mipmaps.length>0){D.__webglFramebuffer=[];for(let Y=0;Y<f.mipmaps.length;Y++)D.__webglFramebuffer[Y]=n.createFramebuffer()}else D.__webglFramebuffer=n.createFramebuffer();if(te)for(let Y=0,Q=ne.length;Y<Q;Y++){const ve=i.get(ne[Y]);ve.__webglTexture===void 0&&(ve.__webglTexture=n.createTexture(),a.memory.textures++)}if(v.samples>0&&le(v)===!1){D.__webglMultisampledFramebuffer=n.createFramebuffer(),D.__webglColorRenderbuffer=[],t.bindFramebuffer(n.FRAMEBUFFER,D.__webglMultisampledFramebuffer);for(let Y=0;Y<ne.length;Y++){const Q=ne[Y];D.__webglColorRenderbuffer[Y]=n.createRenderbuffer(),n.bindRenderbuffer(n.RENDERBUFFER,D.__webglColorRenderbuffer[Y]);const ve=r.convert(Q.format,Q.colorSpace),Te=r.convert(Q.type),fe=T(Q.internalFormat,ve,Te,Q.normalized,Q.colorSpace,v.isXRRenderTarget===!0),ue=ce(v);n.renderbufferStorageMultisample(n.RENDERBUFFER,ue,fe,v.width,v.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+Y,n.RENDERBUFFER,D.__webglColorRenderbuffer[Y])}n.bindRenderbuffer(n.RENDERBUFFER,null),v.depthBuffer&&(D.__webglDepthRenderbuffer=n.createRenderbuffer(),Ke(D.__webglDepthRenderbuffer,v,!0)),t.bindFramebuffer(n.FRAMEBUFFER,null)}}if(V){t.bindTexture(n.TEXTURE_CUBE_MAP,q.__webglTexture),R(n.TEXTURE_CUBE_MAP,f);for(let Y=0;Y<6;Y++)if(f.mipmaps&&f.mipmaps.length>0)for(let Q=0;Q<f.mipmaps.length;Q++)Ce(D.__webglFramebuffer[Y][Q],v,f,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+Y,Q);else Ce(D.__webglFramebuffer[Y],v,f,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+Y,0);d(f)&&y(n.TEXTURE_CUBE_MAP),t.unbindTexture()}else if(te){for(let Y=0,Q=ne.length;Y<Q;Y++){const ve=ne[Y],Te=i.get(ve);let fe=n.TEXTURE_2D;(v.isWebGL3DRenderTarget||v.isWebGLArrayRenderTarget)&&(fe=v.isWebGL3DRenderTarget?n.TEXTURE_3D:n.TEXTURE_2D_ARRAY),t.bindTexture(fe,Te.__webglTexture),R(fe,ve),Ce(D.__webglFramebuffer,v,ve,n.COLOR_ATTACHMENT0+Y,fe,0),d(ve)&&y(fe)}t.unbindTexture()}else{let Y=n.TEXTURE_2D;if((v.isWebGL3DRenderTarget||v.isWebGLArrayRenderTarget)&&(Y=v.isWebGL3DRenderTarget?n.TEXTURE_3D:n.TEXTURE_2D_ARRAY),t.bindTexture(Y,q.__webglTexture),R(Y,f),f.mipmaps&&f.mipmaps.length>0)for(let Q=0;Q<f.mipmaps.length;Q++)Ce(D.__webglFramebuffer[Q],v,f,n.COLOR_ATTACHMENT0,Y,Q);else Ce(D.__webglFramebuffer,v,f,n.COLOR_ATTACHMENT0,Y,0);d(f)&&y(Y),t.unbindTexture()}v.depthBuffer&&tt(v)}function ft(v){const f=v.textures;for(let D=0,q=f.length;D<q;D++){const ne=f[D];if(d(ne)){const V=A(v),te=i.get(ne).__webglTexture;t.bindTexture(V,te),y(V),t.unbindTexture()}}}const G=[],j=[];function E(v){if(v.samples>0){if(le(v)===!1){const f=v.textures,D=v.width,q=v.height;let ne=n.COLOR_BUFFER_BIT;const V=v.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,te=i.get(v),Y=f.length>1;if(Y)for(let ve=0;ve<f.length;ve++)t.bindFramebuffer(n.FRAMEBUFFER,te.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+ve,n.RENDERBUFFER,null),t.bindFramebuffer(n.FRAMEBUFFER,te.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+ve,n.TEXTURE_2D,null,0);t.bindFramebuffer(n.READ_FRAMEBUFFER,te.__webglMultisampledFramebuffer);const Q=v.texture.mipmaps;Q&&Q.length>0?t.bindFramebuffer(n.DRAW_FRAMEBUFFER,te.__webglFramebuffer[0]):t.bindFramebuffer(n.DRAW_FRAMEBUFFER,te.__webglFramebuffer);for(let ve=0;ve<f.length;ve++){if(v.resolveDepthBuffer&&(v.depthBuffer&&(ne|=n.DEPTH_BUFFER_BIT),v.stencilBuffer&&v.resolveStencilBuffer&&(ne|=n.STENCIL_BUFFER_BIT)),Y){n.framebufferRenderbuffer(n.READ_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.RENDERBUFFER,te.__webglColorRenderbuffer[ve]);const Te=i.get(f[ve]).__webglTexture;n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,Te,0)}n.blitFramebuffer(0,0,D,q,0,0,D,q,ne,n.NEAREST),c===!0&&(G.length=0,j.length=0,G.push(n.COLOR_ATTACHMENT0+ve),v.depthBuffer&&v.resolveDepthBuffer===!1&&(G.push(V),j.push(V),n.invalidateFramebuffer(n.DRAW_FRAMEBUFFER,j)),n.invalidateFramebuffer(n.READ_FRAMEBUFFER,G))}if(t.bindFramebuffer(n.READ_FRAMEBUFFER,null),t.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),Y)for(let ve=0;ve<f.length;ve++){t.bindFramebuffer(n.FRAMEBUFFER,te.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+ve,n.RENDERBUFFER,te.__webglColorRenderbuffer[ve]);const Te=i.get(f[ve]).__webglTexture;t.bindFramebuffer(n.FRAMEBUFFER,te.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+ve,n.TEXTURE_2D,Te,0)}t.bindFramebuffer(n.DRAW_FRAMEBUFFER,te.__webglMultisampledFramebuffer)}else if(v.depthBuffer&&v.resolveDepthBuffer===!1&&c){const f=v.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT;n.invalidateFramebuffer(n.DRAW_FRAMEBUFFER,[f])}}}function ce(v){return Math.min(s.maxSamples,v.samples)}function le(v){const f=i.get(v);return v.samples>0&&e.has("WEBGL_multisampled_render_to_texture")===!0&&f.__useRenderToTexture!==!1}function me(v){const f=a.render.frame;u.get(v)!==f&&(u.set(v,f),v.update())}function K(v,f){const D=v.colorSpace,q=v.format,ne=v.type;return v.isCompressedTexture===!0||v.isVideoTexture===!0||D!==ia&&D!==si&&(Ze.getTransfer(D)===at?(q!==pn||ne!==an)&&Ne("WebGLTextures: sRGB encoded textures have to use RGBAFormat and UnsignedByteType."):Qe("WebGLTextures: Unsupported texture color space:",D)),f}function Re(v){return typeof HTMLImageElement<"u"&&v instanceof HTMLImageElement?(l.width=v.naturalWidth||v.width,l.height=v.naturalHeight||v.height):typeof VideoFrame<"u"&&v instanceof VideoFrame?(l.width=v.displayWidth,l.height=v.displayHeight):(l.width=v.width,l.height=v.height),l}this.allocateTextureUnit=W,this.resetTextureUnits=Z,this.getTextureUnits=J,this.setTextureUnits=O,this.setTexture2D=se,this.setTexture2DArray=oe,this.setTexture3D=he,this.setTextureCube=Ee,this.rebindTextures=rt,this.setupRenderTarget=Ve,this.updateRenderTargetMipmap=ft,this.updateMultisampleRenderTarget=E,this.setupDepthRenderbuffer=tt,this.setupFrameBufferTexture=Ce,this.useMultisampledRTT=le,this.isReversedDepthBuffer=function(){return t.buffers.depth.getReversed()}}function a0(n,e){function t(i,s=si){let r;const a=Ze.getTransfer(s);if(i===an)return n.UNSIGNED_BYTE;if(i===_l)return n.UNSIGNED_SHORT_4_4_4_4;if(i===xl)return n.UNSIGNED_SHORT_5_5_5_1;if(i===Ah)return n.UNSIGNED_INT_5_9_9_9_REV;if(i===wh)return n.UNSIGNED_INT_10F_11F_11F_REV;if(i===bh)return n.BYTE;if(i===Th)return n.SHORT;if(i===ks)return n.UNSIGNED_SHORT;if(i===gl)return n.INT;if(i===Ln)return n.UNSIGNED_INT;if(i===An)return n.FLOAT;if(i===Vn)return n.HALF_FLOAT;if(i===Rh)return n.ALPHA;if(i===Ch)return n.RGB;if(i===pn)return n.RGBA;if(i===Wn)return n.DEPTH_COMPONENT;if(i===Ai)return n.DEPTH_STENCIL;if(i===Ph)return n.RED;if(i===vl)return n.RED_INTEGER;if(i===Pi)return n.RG;if(i===Ml)return n.RG_INTEGER;if(i===Sl)return n.RGBA_INTEGER;if(i===kr||i===Hr||i===Gr||i===Vr)if(a===at)if(r=e.get("WEBGL_compressed_texture_s3tc_srgb"),r!==null){if(i===kr)return r.COMPRESSED_SRGB_S3TC_DXT1_EXT;if(i===Hr)return r.COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT;if(i===Gr)return r.COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT;if(i===Vr)return r.COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT}else return null;else if(r=e.get("WEBGL_compressed_texture_s3tc"),r!==null){if(i===kr)return r.COMPRESSED_RGB_S3TC_DXT1_EXT;if(i===Hr)return r.COMPRESSED_RGBA_S3TC_DXT1_EXT;if(i===Gr)return r.COMPRESSED_RGBA_S3TC_DXT3_EXT;if(i===Vr)return r.COMPRESSED_RGBA_S3TC_DXT5_EXT}else return null;if(i===Mo||i===So||i===yo||i===Eo)if(r=e.get("WEBGL_compressed_texture_pvrtc"),r!==null){if(i===Mo)return r.COMPRESSED_RGB_PVRTC_4BPPV1_IMG;if(i===So)return r.COMPRESSED_RGB_PVRTC_2BPPV1_IMG;if(i===yo)return r.COMPRESSED_RGBA_PVRTC_4BPPV1_IMG;if(i===Eo)return r.COMPRESSED_RGBA_PVRTC_2BPPV1_IMG}else return null;if(i===bo||i===To||i===Ao||i===wo||i===Ro||i===ta||i===Co)if(r=e.get("WEBGL_compressed_texture_etc"),r!==null){if(i===bo||i===To)return a===at?r.COMPRESSED_SRGB8_ETC2:r.COMPRESSED_RGB8_ETC2;if(i===Ao)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ETC2_EAC:r.COMPRESSED_RGBA8_ETC2_EAC;if(i===wo)return r.COMPRESSED_R11_EAC;if(i===Ro)return r.COMPRESSED_SIGNED_R11_EAC;if(i===ta)return r.COMPRESSED_RG11_EAC;if(i===Co)return r.COMPRESSED_SIGNED_RG11_EAC}else return null;if(i===Po||i===Lo||i===Do||i===Io||i===Uo||i===No||i===Fo||i===Oo||i===Bo||i===zo||i===ko||i===Ho||i===Go||i===Vo)if(r=e.get("WEBGL_compressed_texture_astc"),r!==null){if(i===Po)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR:r.COMPRESSED_RGBA_ASTC_4x4_KHR;if(i===Lo)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR:r.COMPRESSED_RGBA_ASTC_5x4_KHR;if(i===Do)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR:r.COMPRESSED_RGBA_ASTC_5x5_KHR;if(i===Io)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR:r.COMPRESSED_RGBA_ASTC_6x5_KHR;if(i===Uo)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR:r.COMPRESSED_RGBA_ASTC_6x6_KHR;if(i===No)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR:r.COMPRESSED_RGBA_ASTC_8x5_KHR;if(i===Fo)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR:r.COMPRESSED_RGBA_ASTC_8x6_KHR;if(i===Oo)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR:r.COMPRESSED_RGBA_ASTC_8x8_KHR;if(i===Bo)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR:r.COMPRESSED_RGBA_ASTC_10x5_KHR;if(i===zo)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR:r.COMPRESSED_RGBA_ASTC_10x6_KHR;if(i===ko)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR:r.COMPRESSED_RGBA_ASTC_10x8_KHR;if(i===Ho)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR:r.COMPRESSED_RGBA_ASTC_10x10_KHR;if(i===Go)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR:r.COMPRESSED_RGBA_ASTC_12x10_KHR;if(i===Vo)return a===at?r.COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR:r.COMPRESSED_RGBA_ASTC_12x12_KHR}else return null;if(i===Wo||i===Xo||i===$o)if(r=e.get("EXT_texture_compression_bptc"),r!==null){if(i===Wo)return a===at?r.COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT:r.COMPRESSED_RGBA_BPTC_UNORM_EXT;if(i===Xo)return r.COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT;if(i===$o)return r.COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT}else return null;if(i===Yo||i===qo||i===na||i===jo)if(r=e.get("EXT_texture_compression_rgtc"),r!==null){if(i===Yo)return r.COMPRESSED_RED_RGTC1_EXT;if(i===qo)return r.COMPRESSED_SIGNED_RED_RGTC1_EXT;if(i===na)return r.COMPRESSED_RED_GREEN_RGTC2_EXT;if(i===jo)return r.COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT}else return null;return i===Hs?n.UNSIGNED_INT_24_8:n[i]!==void 0?n[i]:null}return{convert:t}}const o0=`
void main() {

	gl_Position = vec4( position, 1.0 );

}`,l0=`
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

}`;class c0{constructor(){this.texture=null,this.mesh=null,this.depthNear=0,this.depthFar=0}init(e,t){if(this.texture===null){const i=new Wh(e.texture);(e.depthNear!==t.depthNear||e.depthFar!==t.depthFar)&&(this.depthNear=e.depthNear,this.depthFar=e.depthFar),this.texture=i}}getMesh(e){if(this.texture!==null&&this.mesh===null){const t=e.cameras[0].viewport,i=new gn({vertexShader:o0,fragmentShader:l0,uniforms:{depthColor:{value:this.texture},depthWidth:{value:t.z},depthHeight:{value:t.w}}});this.mesh=new Xn(new xa(20,20),i)}return this.mesh}reset(){this.texture=null,this.mesh=null}getDepthTexture(){return this.texture}}class h0 extends fi{constructor(e,t){super();const i=this;let s=null,r=1,a=null,o="local-floor",c=1,l=null,u=null,g=null,h=null,m=null,_=null;const S=typeof XRWebGLBinding<"u",p=new c0,d={},y=t.getContextAttributes();let A=null,T=null;const L=[],b=[],I=new Oe;let M=null;const w=new rn;w.viewport=new Et;const B=new rn;B.viewport=new Et;const C=[w,B],z=new vf;let Z=null,J=null;this.cameraAutoUpdate=!0,this.enabled=!1,this.isPresenting=!1,this.getController=function(U){let ie=L[U];return ie===void 0&&(ie=new Ua,L[U]=ie),ie.getTargetRaySpace()},this.getControllerGrip=function(U){let ie=L[U];return ie===void 0&&(ie=new Ua,L[U]=ie),ie.getGripSpace()},this.getHand=function(U){let ie=L[U];return ie===void 0&&(ie=new Ua,L[U]=ie),ie.getHandSpace()};function O(U){const ie=b.indexOf(U.inputSource);if(ie===-1)return;const re=L[ie];re!==void 0&&(re.update(U.inputSource,U.frame,l||a),re.dispatchEvent({type:U.type,data:U.inputSource}))}function W(){s.removeEventListener("select",O),s.removeEventListener("selectstart",O),s.removeEventListener("selectend",O),s.removeEventListener("squeeze",O),s.removeEventListener("squeezestart",O),s.removeEventListener("squeezeend",O),s.removeEventListener("end",W),s.removeEventListener("inputsourceschange",$);for(let U=0;U<L.length;U++){const ie=b[U];ie!==null&&(b[U]=null,L[U].disconnect(ie))}Z=null,J=null,p.reset();for(const U in d)delete d[U];e.setRenderTarget(A),m=null,h=null,g=null,s=null,T=null,R.stop(),i.isPresenting=!1,e.setPixelRatio(M),e.setSize(I.width,I.height,!1),i.dispatchEvent({type:"sessionend"})}this.setFramebufferScaleFactor=function(U){r=U,i.isPresenting===!0&&Ne("WebXRManager: Cannot change framebuffer scale while presenting.")},this.setReferenceSpaceType=function(U){o=U,i.isPresenting===!0&&Ne("WebXRManager: Cannot change reference space type while presenting.")},this.getReferenceSpace=function(){return l||a},this.setReferenceSpace=function(U){l=U},this.getBaseLayer=function(){return h!==null?h:m},this.getBinding=function(){return g===null&&S&&(g=new XRWebGLBinding(s,t)),g},this.getFrame=function(){return _},this.getSession=function(){return s},this.setSession=async function(U){if(s=U,s!==null){if(A=e.getRenderTarget(),s.addEventListener("select",O),s.addEventListener("selectstart",O),s.addEventListener("selectend",O),s.addEventListener("squeeze",O),s.addEventListener("squeezestart",O),s.addEventListener("squeezeend",O),s.addEventListener("end",W),s.addEventListener("inputsourceschange",$),y.xrCompatible!==!0&&await t.makeXRCompatible(),M=e.getPixelRatio(),e.getSize(I),S&&"createProjectionLayer"in XRWebGLBinding.prototype){let re=null,Me=null,Se=null;y.depth&&(Se=y.stencil?t.DEPTH24_STENCIL8:t.DEPTH_COMPONENT24,re=y.stencil?Ai:Wn,Me=y.stencil?Hs:Ln);const Ce={colorFormat:t.RGBA8,depthFormat:Se,scaleFactor:r};g=this.getBinding(),h=g.createProjectionLayer(Ce),s.updateRenderState({layers:[h]}),e.setPixelRatio(1),e.setSize(h.textureWidth,h.textureHeight,!1),T=new Pn(h.textureWidth,h.textureHeight,{format:pn,type:an,depthTexture:new gs(h.textureWidth,h.textureHeight,Me,void 0,void 0,void 0,void 0,void 0,void 0,re),stencilBuffer:y.stencil,colorSpace:e.outputColorSpace,samples:y.antialias?4:0,resolveDepthBuffer:h.ignoreDepthValues===!1,resolveStencilBuffer:h.ignoreDepthValues===!1})}else{const re={antialias:y.antialias,alpha:!0,depth:y.depth,stencil:y.stencil,framebufferScaleFactor:r};m=new XRWebGLLayer(s,t,re),s.updateRenderState({baseLayer:m}),e.setPixelRatio(1),e.setSize(m.framebufferWidth,m.framebufferHeight,!1),T=new Pn(m.framebufferWidth,m.framebufferHeight,{format:pn,type:an,colorSpace:e.outputColorSpace,stencilBuffer:y.stencil,resolveDepthBuffer:m.ignoreDepthValues===!1,resolveStencilBuffer:m.ignoreDepthValues===!1})}T.isXRRenderTarget=!0,this.setFoveation(c),l=null,a=await s.requestReferenceSpace(o),R.setContext(s),R.start(),i.isPresenting=!0,i.dispatchEvent({type:"sessionstart"})}},this.getEnvironmentBlendMode=function(){if(s!==null)return s.environmentBlendMode},this.getDepthTexture=function(){return p.getDepthTexture()};function $(U){for(let ie=0;ie<U.removed.length;ie++){const re=U.removed[ie],Me=b.indexOf(re);Me>=0&&(b[Me]=null,L[Me].disconnect(re))}for(let ie=0;ie<U.added.length;ie++){const re=U.added[ie];let Me=b.indexOf(re);if(Me===-1){for(let Ce=0;Ce<L.length;Ce++)if(Ce>=b.length){b.push(re),Me=Ce;break}else if(b[Ce]===null){b[Ce]=re,Me=Ce;break}if(Me===-1)break}const Se=L[Me];Se&&Se.connect(re)}}const se=new N,oe=new N;function he(U,ie,re){se.setFromMatrixPosition(ie.matrixWorld),oe.setFromMatrixPosition(re.matrixWorld);const Me=se.distanceTo(oe),Se=ie.projectionMatrix.elements,Ce=re.projectionMatrix.elements,Ke=Se[14]/(Se[10]-1),Fe=Se[14]/(Se[10]+1),tt=(Se[9]+1)/Se[5],rt=(Se[9]-1)/Se[5],Ve=(Se[8]-1)/Se[0],ft=(Ce[8]+1)/Ce[0],G=Ke*Ve,j=Ke*ft,E=Me/(-Ve+ft),ce=E*-Ve;if(ie.matrixWorld.decompose(U.position,U.quaternion,U.scale),U.translateX(ce),U.translateZ(E),U.matrixWorld.compose(U.position,U.quaternion,U.scale),U.matrixWorldInverse.copy(U.matrixWorld).invert(),Se[10]===-1)U.projectionMatrix.copy(ie.projectionMatrix),U.projectionMatrixInverse.copy(ie.projectionMatrixInverse);else{const le=Ke+E,me=Fe+E,K=G-ce,Re=j+(Me-ce),v=tt*Fe/me*le,f=rt*Fe/me*le;U.projectionMatrix.makePerspective(K,Re,v,f,le,me),U.projectionMatrixInverse.copy(U.projectionMatrix).invert()}}function Ee(U,ie){ie===null?U.matrixWorld.copy(U.matrix):U.matrixWorld.multiplyMatrices(ie.matrixWorld,U.matrix),U.matrixWorldInverse.copy(U.matrixWorld).invert()}this.updateCamera=function(U){if(s===null)return;let ie=U.near,re=U.far;p.texture!==null&&(p.depthNear>0&&(ie=p.depthNear),p.depthFar>0&&(re=p.depthFar)),z.near=B.near=w.near=ie,z.far=B.far=w.far=re,(Z!==z.near||J!==z.far)&&(s.updateRenderState({depthNear:z.near,depthFar:z.far}),Z=z.near,J=z.far),z.layers.mask=U.layers.mask|6,w.layers.mask=z.layers.mask&-5,B.layers.mask=z.layers.mask&-3;const Me=U.parent,Se=z.cameras;Ee(z,Me);for(let Ce=0;Ce<Se.length;Ce++)Ee(Se[Ce],Me);Se.length===2?he(z,w,B):z.projectionMatrix.copy(w.projectionMatrix),Pe(U,z,Me)};function Pe(U,ie,re){re===null?U.matrix.copy(ie.matrixWorld):(U.matrix.copy(re.matrixWorld),U.matrix.invert(),U.matrix.multiply(ie.matrixWorld)),U.matrix.decompose(U.position,U.quaternion,U.scale),U.updateMatrixWorld(!0),U.projectionMatrix.copy(ie.projectionMatrix),U.projectionMatrixInverse.copy(ie.projectionMatrixInverse),U.isPerspectiveCamera&&(U.fov=Jo*2*Math.atan(1/U.projectionMatrix.elements[5]),U.zoom=1)}this.getCamera=function(){return z},this.getFoveation=function(){if(!(h===null&&m===null))return c},this.setFoveation=function(U){c=U,h!==null&&(h.fixedFoveation=U),m!==null&&m.fixedFoveation!==void 0&&(m.fixedFoveation=U)},this.hasDepthSensing=function(){return p.texture!==null},this.getDepthSensingMesh=function(){return p.getMesh(z)},this.getCameraTexture=function(U){return d[U]};let Ge=null;function Je(U,ie){if(u=ie.getViewerPose(l||a),_=ie,u!==null){const re=u.views;m!==null&&(e.setRenderTargetFramebuffer(T,m.framebuffer),e.setRenderTarget(T));let Me=!1;re.length!==z.cameras.length&&(z.cameras.length=0,Me=!0);for(let Fe=0;Fe<re.length;Fe++){const tt=re[Fe];let rt=null;if(m!==null)rt=m.getViewport(tt);else{const ft=g.getViewSubImage(h,tt);rt=ft.viewport,Fe===0&&(e.setRenderTargetTextures(T,ft.colorTexture,ft.depthStencilTexture),e.setRenderTarget(T))}let Ve=C[Fe];Ve===void 0&&(Ve=new rn,Ve.layers.enable(Fe),Ve.viewport=new Et,C[Fe]=Ve),Ve.matrix.fromArray(tt.transform.matrix),Ve.matrix.decompose(Ve.position,Ve.quaternion,Ve.scale),Ve.projectionMatrix.fromArray(tt.projectionMatrix),Ve.projectionMatrixInverse.copy(Ve.projectionMatrix).invert(),Ve.viewport.set(rt.x,rt.y,rt.width,rt.height),Fe===0&&(z.matrix.copy(Ve.matrix),z.matrix.decompose(z.position,z.quaternion,z.scale)),Me===!0&&z.cameras.push(Ve)}const Se=s.enabledFeatures;if(Se&&Se.includes("depth-sensing")&&s.depthUsage=="gpu-optimized"&&S){g=i.getBinding();const Fe=g.getDepthInformation(re[0]);Fe&&Fe.isValid&&Fe.texture&&p.init(Fe,s.renderState)}if(Se&&Se.includes("camera-access")&&S){e.state.unbindTexture(),g=i.getBinding();for(let Fe=0;Fe<re.length;Fe++){const tt=re[Fe].camera;if(tt){let rt=d[tt];rt||(rt=new Wh,d[tt]=rt);const Ve=g.getCameraImage(tt);rt.sourceTexture=Ve}}}}for(let re=0;re<L.length;re++){const Me=b[re],Se=L[re];Me!==null&&Se!==void 0&&Se.update(Me,ie,l||a)}Ge&&Ge(U,ie),ie.detectedPlanes&&i.dispatchEvent({type:"planesdetected",data:ie}),_=null}const R=new jh;R.setAnimationLoop(Je),this.setAnimationLoop=function(U){Ge=U},this.dispose=function(){}}}const u0=new gt,nu=new He;nu.set(-1,0,0,0,1,0,0,0,1);function d0(n,e){function t(p,d){p.matrixAutoUpdate===!0&&p.updateMatrix(),d.value.copy(p.matrix)}function i(p,d){d.color.getRGB(p.fogColor.value,Xh(n)),d.isFog?(p.fogNear.value=d.near,p.fogFar.value=d.far):d.isFogExp2&&(p.fogDensity.value=d.density)}function s(p,d,y,A,T){d.isNodeMaterial?d.uniformsNeedUpdate=!1:d.isMeshBasicMaterial?r(p,d):d.isMeshLambertMaterial?(r(p,d),d.envMap&&(p.envMapIntensity.value=d.envMapIntensity)):d.isMeshToonMaterial?(r(p,d),g(p,d)):d.isMeshPhongMaterial?(r(p,d),u(p,d),d.envMap&&(p.envMapIntensity.value=d.envMapIntensity)):d.isMeshStandardMaterial?(r(p,d),h(p,d),d.isMeshPhysicalMaterial&&m(p,d,T)):d.isMeshMatcapMaterial?(r(p,d),_(p,d)):d.isMeshDepthMaterial?r(p,d):d.isMeshDistanceMaterial?(r(p,d),S(p,d)):d.isMeshNormalMaterial?r(p,d):d.isLineBasicMaterial?(a(p,d),d.isLineDashedMaterial&&o(p,d)):d.isPointsMaterial?c(p,d,y,A):d.isSpriteMaterial?l(p,d):d.isShadowMaterial?(p.color.value.copy(d.color),p.opacity.value=d.opacity):d.isShaderMaterial&&(d.uniformsNeedUpdate=!1)}function r(p,d){p.opacity.value=d.opacity,d.color&&p.diffuse.value.copy(d.color),d.emissive&&p.emissive.value.copy(d.emissive).multiplyScalar(d.emissiveIntensity),d.map&&(p.map.value=d.map,t(d.map,p.mapTransform)),d.alphaMap&&(p.alphaMap.value=d.alphaMap,t(d.alphaMap,p.alphaMapTransform)),d.bumpMap&&(p.bumpMap.value=d.bumpMap,t(d.bumpMap,p.bumpMapTransform),p.bumpScale.value=d.bumpScale,d.side===Kt&&(p.bumpScale.value*=-1)),d.normalMap&&(p.normalMap.value=d.normalMap,t(d.normalMap,p.normalMapTransform),p.normalScale.value.copy(d.normalScale),d.side===Kt&&p.normalScale.value.negate()),d.displacementMap&&(p.displacementMap.value=d.displacementMap,t(d.displacementMap,p.displacementMapTransform),p.displacementScale.value=d.displacementScale,p.displacementBias.value=d.displacementBias),d.emissiveMap&&(p.emissiveMap.value=d.emissiveMap,t(d.emissiveMap,p.emissiveMapTransform)),d.specularMap&&(p.specularMap.value=d.specularMap,t(d.specularMap,p.specularMapTransform)),d.alphaTest>0&&(p.alphaTest.value=d.alphaTest);const y=e.get(d),A=y.envMap,T=y.envMapRotation;A&&(p.envMap.value=A,p.envMapRotation.value.setFromMatrix4(u0.makeRotationFromEuler(T)).transpose(),A.isCubeTexture&&A.isRenderTargetTexture===!1&&p.envMapRotation.value.premultiply(nu),p.reflectivity.value=d.reflectivity,p.ior.value=d.ior,p.refractionRatio.value=d.refractionRatio),d.lightMap&&(p.lightMap.value=d.lightMap,p.lightMapIntensity.value=d.lightMapIntensity,t(d.lightMap,p.lightMapTransform)),d.aoMap&&(p.aoMap.value=d.aoMap,p.aoMapIntensity.value=d.aoMapIntensity,t(d.aoMap,p.aoMapTransform))}function a(p,d){p.diffuse.value.copy(d.color),p.opacity.value=d.opacity,d.map&&(p.map.value=d.map,t(d.map,p.mapTransform))}function o(p,d){p.dashSize.value=d.dashSize,p.totalSize.value=d.dashSize+d.gapSize,p.scale.value=d.scale}function c(p,d,y,A){p.diffuse.value.copy(d.color),p.opacity.value=d.opacity,p.size.value=d.size*y,p.scale.value=A*.5,d.map&&(p.map.value=d.map,t(d.map,p.uvTransform)),d.alphaMap&&(p.alphaMap.value=d.alphaMap,t(d.alphaMap,p.alphaMapTransform)),d.alphaTest>0&&(p.alphaTest.value=d.alphaTest)}function l(p,d){p.diffuse.value.copy(d.color),p.opacity.value=d.opacity,p.rotation.value=d.rotation,d.map&&(p.map.value=d.map,t(d.map,p.mapTransform)),d.alphaMap&&(p.alphaMap.value=d.alphaMap,t(d.alphaMap,p.alphaMapTransform)),d.alphaTest>0&&(p.alphaTest.value=d.alphaTest)}function u(p,d){p.specular.value.copy(d.specular),p.shininess.value=Math.max(d.shininess,1e-4)}function g(p,d){d.gradientMap&&(p.gradientMap.value=d.gradientMap)}function h(p,d){p.metalness.value=d.metalness,d.metalnessMap&&(p.metalnessMap.value=d.metalnessMap,t(d.metalnessMap,p.metalnessMapTransform)),p.roughness.value=d.roughness,d.roughnessMap&&(p.roughnessMap.value=d.roughnessMap,t(d.roughnessMap,p.roughnessMapTransform)),d.envMap&&(p.envMapIntensity.value=d.envMapIntensity)}function m(p,d,y){p.ior.value=d.ior,d.sheen>0&&(p.sheenColor.value.copy(d.sheenColor).multiplyScalar(d.sheen),p.sheenRoughness.value=d.sheenRoughness,d.sheenColorMap&&(p.sheenColorMap.value=d.sheenColorMap,t(d.sheenColorMap,p.sheenColorMapTransform)),d.sheenRoughnessMap&&(p.sheenRoughnessMap.value=d.sheenRoughnessMap,t(d.sheenRoughnessMap,p.sheenRoughnessMapTransform))),d.clearcoat>0&&(p.clearcoat.value=d.clearcoat,p.clearcoatRoughness.value=d.clearcoatRoughness,d.clearcoatMap&&(p.clearcoatMap.value=d.clearcoatMap,t(d.clearcoatMap,p.clearcoatMapTransform)),d.clearcoatRoughnessMap&&(p.clearcoatRoughnessMap.value=d.clearcoatRoughnessMap,t(d.clearcoatRoughnessMap,p.clearcoatRoughnessMapTransform)),d.clearcoatNormalMap&&(p.clearcoatNormalMap.value=d.clearcoatNormalMap,t(d.clearcoatNormalMap,p.clearcoatNormalMapTransform),p.clearcoatNormalScale.value.copy(d.clearcoatNormalScale),d.side===Kt&&p.clearcoatNormalScale.value.negate())),d.dispersion>0&&(p.dispersion.value=d.dispersion),d.iridescence>0&&(p.iridescence.value=d.iridescence,p.iridescenceIOR.value=d.iridescenceIOR,p.iridescenceThicknessMinimum.value=d.iridescenceThicknessRange[0],p.iridescenceThicknessMaximum.value=d.iridescenceThicknessRange[1],d.iridescenceMap&&(p.iridescenceMap.value=d.iridescenceMap,t(d.iridescenceMap,p.iridescenceMapTransform)),d.iridescenceThicknessMap&&(p.iridescenceThicknessMap.value=d.iridescenceThicknessMap,t(d.iridescenceThicknessMap,p.iridescenceThicknessMapTransform))),d.transmission>0&&(p.transmission.value=d.transmission,p.transmissionSamplerMap.value=y.texture,p.transmissionSamplerSize.value.set(y.width,y.height),d.transmissionMap&&(p.transmissionMap.value=d.transmissionMap,t(d.transmissionMap,p.transmissionMapTransform)),p.thickness.value=d.thickness,d.thicknessMap&&(p.thicknessMap.value=d.thicknessMap,t(d.thicknessMap,p.thicknessMapTransform)),p.attenuationDistance.value=d.attenuationDistance,p.attenuationColor.value.copy(d.attenuationColor)),d.anisotropy>0&&(p.anisotropyVector.value.set(d.anisotropy*Math.cos(d.anisotropyRotation),d.anisotropy*Math.sin(d.anisotropyRotation)),d.anisotropyMap&&(p.anisotropyMap.value=d.anisotropyMap,t(d.anisotropyMap,p.anisotropyMapTransform))),p.specularIntensity.value=d.specularIntensity,p.specularColor.value.copy(d.specularColor),d.specularColorMap&&(p.specularColorMap.value=d.specularColorMap,t(d.specularColorMap,p.specularColorMapTransform)),d.specularIntensityMap&&(p.specularIntensityMap.value=d.specularIntensityMap,t(d.specularIntensityMap,p.specularIntensityMapTransform))}function _(p,d){d.matcap&&(p.matcap.value=d.matcap)}function S(p,d){const y=e.get(d).light;p.referencePosition.value.setFromMatrixPosition(y.matrixWorld),p.nearDistance.value=y.shadow.camera.near,p.farDistance.value=y.shadow.camera.far}return{refreshFogUniforms:i,refreshMaterialUniforms:s}}function f0(n,e,t,i){let s={},r={},a=[];const o=n.getParameter(n.MAX_UNIFORM_BUFFER_BINDINGS);function c(y,A){const T=A.program;i.uniformBlockBinding(y,T)}function l(y,A){let T=s[y.id];T===void 0&&(_(y),T=u(y),s[y.id]=T,y.addEventListener("dispose",p));const L=A.program;i.updateUBOMapping(y,L);const b=e.render.frame;r[y.id]!==b&&(h(y),r[y.id]=b)}function u(y){const A=g();y.__bindingPointIndex=A;const T=n.createBuffer(),L=y.__size,b=y.usage;return n.bindBuffer(n.UNIFORM_BUFFER,T),n.bufferData(n.UNIFORM_BUFFER,L,b),n.bindBuffer(n.UNIFORM_BUFFER,null),n.bindBufferBase(n.UNIFORM_BUFFER,A,T),T}function g(){for(let y=0;y<o;y++)if(a.indexOf(y)===-1)return a.push(y),y;return Qe("WebGLRenderer: Maximum number of simultaneously usable uniforms groups reached."),0}function h(y){const A=s[y.id],T=y.uniforms,L=y.__cache;n.bindBuffer(n.UNIFORM_BUFFER,A);for(let b=0,I=T.length;b<I;b++){const M=Array.isArray(T[b])?T[b]:[T[b]];for(let w=0,B=M.length;w<B;w++){const C=M[w];if(m(C,b,w,L)===!0){const z=C.__offset,Z=Array.isArray(C.value)?C.value:[C.value];let J=0;for(let O=0;O<Z.length;O++){const W=Z[O],$=S(W);typeof W=="number"||typeof W=="boolean"?(C.__data[0]=W,n.bufferSubData(n.UNIFORM_BUFFER,z+J,C.__data)):W.isMatrix3?(C.__data[0]=W.elements[0],C.__data[1]=W.elements[1],C.__data[2]=W.elements[2],C.__data[3]=0,C.__data[4]=W.elements[3],C.__data[5]=W.elements[4],C.__data[6]=W.elements[5],C.__data[7]=0,C.__data[8]=W.elements[6],C.__data[9]=W.elements[7],C.__data[10]=W.elements[8],C.__data[11]=0):ArrayBuffer.isView(W)?C.__data.set(new W.constructor(W.buffer,W.byteOffset,C.__data.length)):(W.toArray(C.__data,J),J+=$.storage/Float32Array.BYTES_PER_ELEMENT)}n.bufferSubData(n.UNIFORM_BUFFER,z,C.__data)}}}n.bindBuffer(n.UNIFORM_BUFFER,null)}function m(y,A,T,L){const b=y.value,I=A+"_"+T;if(L[I]===void 0)return typeof b=="number"||typeof b=="boolean"?L[I]=b:ArrayBuffer.isView(b)?L[I]=b.slice():L[I]=b.clone(),!0;{const M=L[I];if(typeof b=="number"||typeof b=="boolean"){if(M!==b)return L[I]=b,!0}else{if(ArrayBuffer.isView(b))return!0;if(M.equals(b)===!1)return M.copy(b),!0}}return!1}function _(y){const A=y.uniforms;let T=0;const L=16;for(let I=0,M=A.length;I<M;I++){const w=Array.isArray(A[I])?A[I]:[A[I]];for(let B=0,C=w.length;B<C;B++){const z=w[B],Z=Array.isArray(z.value)?z.value:[z.value];for(let J=0,O=Z.length;J<O;J++){const W=Z[J],$=S(W),se=T%L,oe=se%$.boundary,he=se+oe;T+=oe,he!==0&&L-he<$.storage&&(T+=L-he),z.__data=new Float32Array($.storage/Float32Array.BYTES_PER_ELEMENT),z.__offset=T,T+=$.storage}}}const b=T%L;return b>0&&(T+=L-b),y.__size=T,y.__cache={},this}function S(y){const A={boundary:0,storage:0};return typeof y=="number"||typeof y=="boolean"?(A.boundary=4,A.storage=4):y.isVector2?(A.boundary=8,A.storage=8):y.isVector3||y.isColor?(A.boundary=16,A.storage=12):y.isVector4?(A.boundary=16,A.storage=16):y.isMatrix3?(A.boundary=48,A.storage=48):y.isMatrix4?(A.boundary=64,A.storage=64):y.isTexture?Ne("WebGLRenderer: Texture samplers can not be part of an uniforms group."):ArrayBuffer.isView(y)?(A.boundary=16,A.storage=y.byteLength):Ne("WebGLRenderer: Unsupported uniform value type.",y),A}function p(y){const A=y.target;A.removeEventListener("dispose",p);const T=a.indexOf(A.__bindingPointIndex);a.splice(T,1),n.deleteBuffer(s[A.id]),delete s[A.id],delete r[A.id]}function d(){for(const y in s)n.deleteBuffer(s[y]);a=[],s={},r={}}return{bind:c,update:l,dispose:d}}const p0=new Uint16Array([12469,15057,12620,14925,13266,14620,13807,14376,14323,13990,14545,13625,14713,13328,14840,12882,14931,12528,14996,12233,15039,11829,15066,11525,15080,11295,15085,10976,15082,10705,15073,10495,13880,14564,13898,14542,13977,14430,14158,14124,14393,13732,14556,13410,14702,12996,14814,12596,14891,12291,14937,11834,14957,11489,14958,11194,14943,10803,14921,10506,14893,10278,14858,9960,14484,14039,14487,14025,14499,13941,14524,13740,14574,13468,14654,13106,14743,12678,14818,12344,14867,11893,14889,11509,14893,11180,14881,10751,14852,10428,14812,10128,14765,9754,14712,9466,14764,13480,14764,13475,14766,13440,14766,13347,14769,13070,14786,12713,14816,12387,14844,11957,14860,11549,14868,11215,14855,10751,14825,10403,14782,10044,14729,9651,14666,9352,14599,9029,14967,12835,14966,12831,14963,12804,14954,12723,14936,12564,14917,12347,14900,11958,14886,11569,14878,11247,14859,10765,14828,10401,14784,10011,14727,9600,14660,9289,14586,8893,14508,8533,15111,12234,15110,12234,15104,12216,15092,12156,15067,12010,15028,11776,14981,11500,14942,11205,14902,10752,14861,10393,14812,9991,14752,9570,14682,9252,14603,8808,14519,8445,14431,8145,15209,11449,15208,11451,15202,11451,15190,11438,15163,11384,15117,11274,15055,10979,14994,10648,14932,10343,14871,9936,14803,9532,14729,9218,14645,8742,14556,8381,14461,8020,14365,7603,15273,10603,15272,10607,15267,10619,15256,10631,15231,10614,15182,10535,15118,10389,15042,10167,14963,9787,14883,9447,14800,9115,14710,8665,14615,8318,14514,7911,14411,7507,14279,7198,15314,9675,15313,9683,15309,9712,15298,9759,15277,9797,15229,9773,15166,9668,15084,9487,14995,9274,14898,8910,14800,8539,14697,8234,14590,7790,14479,7409,14367,7067,14178,6621,15337,8619,15337,8631,15333,8677,15325,8769,15305,8871,15264,8940,15202,8909,15119,8775,15022,8565,14916,8328,14804,8009,14688,7614,14569,7287,14448,6888,14321,6483,14088,6171,15350,7402,15350,7419,15347,7480,15340,7613,15322,7804,15287,7973,15229,8057,15148,8012,15046,7846,14933,7611,14810,7357,14682,7069,14552,6656,14421,6316,14251,5948,14007,5528,15356,5942,15356,5977,15353,6119,15348,6294,15332,6551,15302,6824,15249,7044,15171,7122,15070,7050,14949,6861,14818,6611,14679,6349,14538,6067,14398,5651,14189,5311,13935,4958,15359,4123,15359,4153,15356,4296,15353,4646,15338,5160,15311,5508,15263,5829,15188,6042,15088,6094,14966,6001,14826,5796,14678,5543,14527,5287,14377,4985,14133,4586,13869,4257,15360,1563,15360,1642,15358,2076,15354,2636,15341,3350,15317,4019,15273,4429,15203,4732,15105,4911,14981,4932,14836,4818,14679,4621,14517,4386,14359,4156,14083,3795,13808,3437,15360,122,15360,137,15358,285,15355,636,15344,1274,15322,2177,15281,2765,15215,3223,15120,3451,14995,3569,14846,3567,14681,3466,14511,3305,14344,3121,14037,2800,13753,2467,15360,0,15360,1,15359,21,15355,89,15346,253,15325,479,15287,796,15225,1148,15133,1492,15008,1749,14856,1882,14685,1886,14506,1783,14324,1608,13996,1398,13702,1183]);let yn=null;function m0(){return yn===null&&(yn=new rf(p0,16,16,Pi,Vn),yn.name="DFG_LUT",yn.minFilter=Bt,yn.magFilter=Bt,yn.wrapS=kn,yn.wrapT=kn,yn.generateMipmaps=!1,yn.needsUpdate=!0),yn}class g0{constructor(e={}){const{canvas:t=Dd(),context:i=null,depth:s=!0,stencil:r=!1,alpha:a=!1,antialias:o=!1,premultipliedAlpha:c=!0,preserveDrawingBuffer:l=!1,powerPreference:u="default",failIfMajorPerformanceCaveat:g=!1,reversedDepthBuffer:h=!1,outputBufferType:m=an}=e;this.isWebGLRenderer=!0;let _;if(i!==null){if(typeof WebGLRenderingContext<"u"&&i instanceof WebGLRenderingContext)throw new Error("THREE.WebGLRenderer: WebGL 1 is not supported since r163.");_=i.getContextAttributes().alpha}else _=a;const S=m,p=new Set([Sl,Ml,vl]),d=new Set([an,Ln,ks,Hs,_l,xl]),y=new Uint32Array(4),A=new Int32Array(4),T=new N;let L=null,b=null;const I=[],M=[];let w=null;this.domElement=t,this.debug={checkShaderErrors:!0,onShaderError:null},this.autoClear=!0,this.autoClearColor=!0,this.autoClearDepth=!0,this.autoClearStencil=!0,this.sortObjects=!0,this.clippingPlanes=[],this.localClippingEnabled=!1,this.toneMapping=Cn,this.toneMappingExposure=1,this.transmissionResolutionScale=1;const B=this;let C=!1,z=null;this._outputColorSpace=sn;let Z=0,J=0,O=null,W=-1,$=null;const se=new Et,oe=new Et;let he=null;const Ee=new ke(0);let Pe=0,Ge=t.width,Je=t.height,R=1,U=null,ie=null;const re=new Et(0,0,Ge,Je),Me=new Et(0,0,Ge,Je);let Se=!1;const Ce=new kh;let Ke=!1,Fe=!1;const tt=new gt,rt=new N,Ve=new Et,ft={background:null,fog:null,environment:null,overrideMaterial:null,isScene:!0};let G=!1;function j(){return O===null?R:1}let E=i;function ce(x,F){return t.getContext(x,F)}try{const x={alpha:!0,depth:s,stencil:r,antialias:o,premultipliedAlpha:c,preserveDrawingBuffer:l,powerPreference:u,failIfMajorPerformanceCaveat:g};if("setAttribute"in t&&t.setAttribute("data-engine",`three.js r${ml}`),t.addEventListener("webglcontextlost",ae,!1),t.addEventListener("webglcontextrestored",De,!1),t.addEventListener("webglcontextcreationerror",We,!1),E===null){const F="webgl2";if(E=ce(F,x),E===null)throw ce(F)?new Error("Error creating WebGL context with your selected attributes."):new Error("Error creating WebGL context.")}}catch(x){throw Qe("WebGLRenderer: "+x.message),x}let le,me,K,Re,v,f,D,q,ne,V,te,Y,Q,ve,Te,fe,ue,Be,$e,it,P,de,ee;function ye(){le=new mg(E),le.init(),P=new a0(E,le),me=new og(E,le,e,P),K=new s0(E,le),me.reversedDepthBuffer&&h&&K.buffers.depth.setReversed(!0),Re=new xg(E),v=new W_,f=new r0(E,le,K,v,me,P,Re),D=new pg(B),q=new yf(E),de=new rg(E,q),ne=new gg(E,q,Re,de),V=new Mg(E,ne,q,de,Re),Be=new vg(E,me,f),Te=new lg(v),te=new V_(B,D,le,me,de,Te),Y=new d0(B,v),Q=new $_,ve=new J_(le),ue=new sg(B,D,K,V,_,c),fe=new i0(B,V,me),ee=new f0(E,Re,me,K),$e=new ag(E,le,Re),it=new _g(E,le,Re),Re.programs=te.programs,B.capabilities=me,B.extensions=le,B.properties=v,B.renderLists=Q,B.shadowMap=fe,B.state=K,B.info=Re}ye(),S!==an&&(w=new yg(S,t.width,t.height,s,r));const pe=new h0(B,E);this.xr=pe,this.getContext=function(){return E},this.getContextAttributes=function(){return E.getContextAttributes()},this.forceContextLoss=function(){const x=le.get("WEBGL_lose_context");x&&x.loseContext()},this.forceContextRestore=function(){const x=le.get("WEBGL_lose_context");x&&x.restoreContext()},this.getPixelRatio=function(){return R},this.setPixelRatio=function(x){x!==void 0&&(R=x,this.setSize(Ge,Je,!1))},this.getSize=function(x){return x.set(Ge,Je)},this.setSize=function(x,F,X=!0){if(pe.isPresenting){Ne("WebGLRenderer: Can't change size while VR device is presenting.");return}Ge=x,Je=F,t.width=Math.floor(x*R),t.height=Math.floor(F*R),X===!0&&(t.style.width=x+"px",t.style.height=F+"px"),w!==null&&w.setSize(t.width,t.height),this.setViewport(0,0,x,F)},this.getDrawingBufferSize=function(x){return x.set(Ge*R,Je*R).floor()},this.setDrawingBufferSize=function(x,F,X){Ge=x,Je=F,R=X,t.width=Math.floor(x*X),t.height=Math.floor(F*X),this.setViewport(0,0,x,F)},this.setEffects=function(x){if(S===an){Qe("THREE.WebGLRenderer: setEffects() requires outputBufferType set to HalfFloatType or FloatType.");return}if(x){for(let F=0;F<x.length;F++)if(x[F].isOutputPass===!0){Ne("THREE.WebGLRenderer: OutputPass is not needed in setEffects(). Tone mapping and color space conversion are applied automatically.");break}}w.setEffects(x||[])},this.getCurrentViewport=function(x){return x.copy(se)},this.getViewport=function(x){return x.copy(re)},this.setViewport=function(x,F,X,k){x.isVector4?re.set(x.x,x.y,x.z,x.w):re.set(x,F,X,k),K.viewport(se.copy(re).multiplyScalar(R).round())},this.getScissor=function(x){return x.copy(Me)},this.setScissor=function(x,F,X,k){x.isVector4?Me.set(x.x,x.y,x.z,x.w):Me.set(x,F,X,k),K.scissor(oe.copy(Me).multiplyScalar(R).round())},this.getScissorTest=function(){return Se},this.setScissorTest=function(x){K.setScissorTest(Se=x)},this.setOpaqueSort=function(x){U=x},this.setTransparentSort=function(x){ie=x},this.getClearColor=function(x){return x.copy(ue.getClearColor())},this.setClearColor=function(){ue.setClearColor(...arguments)},this.getClearAlpha=function(){return ue.getClearAlpha()},this.setClearAlpha=function(){ue.setClearAlpha(...arguments)},this.clear=function(x=!0,F=!0,X=!0){let k=0;if(x){let H=!1;if(O!==null){const xe=O.texture.format;H=p.has(xe)}if(H){const xe=O.texture.type,Ae=d.has(xe),_e=ue.getClearColor(),Le=ue.getClearAlpha(),Ie=_e.r,Xe=_e.g,qe=_e.b;Ae?(y[0]=Ie,y[1]=Xe,y[2]=qe,y[3]=Le,E.clearBufferuiv(E.COLOR,0,y)):(A[0]=Ie,A[1]=Xe,A[2]=qe,A[3]=Le,E.clearBufferiv(E.COLOR,0,A))}else k|=E.COLOR_BUFFER_BIT}F&&(k|=E.DEPTH_BUFFER_BIT,this.state.buffers.depth.setMask(!0)),X&&(k|=E.STENCIL_BUFFER_BIT,this.state.buffers.stencil.setMask(4294967295)),k!==0&&E.clear(k)},this.clearColor=function(){this.clear(!0,!1,!1)},this.clearDepth=function(){this.clear(!1,!0,!1)},this.clearStencil=function(){this.clear(!1,!1,!0)},this.setNodesHandler=function(x){x.setRenderer(this),z=x},this.dispose=function(){t.removeEventListener("webglcontextlost",ae,!1),t.removeEventListener("webglcontextrestored",De,!1),t.removeEventListener("webglcontextcreationerror",We,!1),ue.dispose(),Q.dispose(),ve.dispose(),v.dispose(),D.dispose(),V.dispose(),de.dispose(),ee.dispose(),te.dispose(),pe.dispose(),pe.removeEventListener("sessionstart",$l),pe.removeEventListener("sessionend",Yl),mi.stop()};function ae(x){x.preventDefault(),oa("WebGLRenderer: Context Lost."),C=!0}function De(){oa("WebGLRenderer: Context Restored."),C=!1;const x=Re.autoReset,F=fe.enabled,X=fe.autoUpdate,k=fe.needsUpdate,H=fe.type;ye(),Re.autoReset=x,fe.enabled=F,fe.autoUpdate=X,fe.needsUpdate=k,fe.type=H}function We(x){Qe("WebGLRenderer: A WebGL context could not be created. Reason: ",x.statusMessage)}function vt(x){const F=x.target;F.removeEventListener("dispose",vt),ot(F)}function ot(x){Dn(x),v.remove(x)}function Dn(x){const F=v.get(x).programs;F!==void 0&&(F.forEach(function(X){te.releaseProgram(X)}),x.isShaderMaterial&&te.releaseShaderCache(x))}this.renderBufferDirect=function(x,F,X,k,H,xe){F===null&&(F=ft);const Ae=H.isMesh&&H.matrixWorld.determinant()<0,_e=Xu(x,F,X,k,H);K.setMaterial(k,Ae);let Le=X.index,Ie=1;if(k.wireframe===!0){if(Le=ne.getWireframeAttribute(X),Le===void 0)return;Ie=2}const Xe=X.drawRange,qe=X.attributes.position;let Ue=Xe.start*Ie,lt=(Xe.start+Xe.count)*Ie;xe!==null&&(Ue=Math.max(Ue,xe.start*Ie),lt=Math.min(lt,(xe.start+xe.count)*Ie)),Le!==null?(Ue=Math.max(Ue,0),lt=Math.min(lt,Le.count)):qe!=null&&(Ue=Math.max(Ue,0),lt=Math.min(lt,qe.count));const Mt=lt-Ue;if(Mt<0||Mt===1/0)return;de.setup(H,k,_e,X,Le);let _t,ut=$e;if(Le!==null&&(_t=q.get(Le),ut=it,ut.setIndex(_t)),H.isMesh)k.wireframe===!0?(K.setLineWidth(k.wireframeLinewidth*j()),ut.setMode(E.LINES)):ut.setMode(E.TRIANGLES);else if(H.isLine){let Nt=k.linewidth;Nt===void 0&&(Nt=1),K.setLineWidth(Nt*j()),H.isLineSegments?ut.setMode(E.LINES):H.isLineLoop?ut.setMode(E.LINE_LOOP):ut.setMode(E.LINE_STRIP)}else H.isPoints?ut.setMode(E.POINTS):H.isSprite&&ut.setMode(E.TRIANGLES);if(H.isBatchedMesh)if(le.get("WEBGL_multi_draw"))ut.renderMultiDraw(H._multiDrawStarts,H._multiDrawCounts,H._multiDrawCount);else{const Nt=H._multiDrawStarts,be=H._multiDrawCounts,Zt=H._multiDrawCount,nt=Le?q.get(Le).bytesPerElement:1,tn=v.get(k).currentProgram.getUniforms();for(let Mn=0;Mn<Zt;Mn++)tn.setValue(E,"_gl_DrawID",Mn),ut.render(Nt[Mn]/nt,be[Mn])}else if(H.isInstancedMesh)ut.renderInstances(Ue,Mt,H.count);else if(X.isInstancedBufferGeometry){const Nt=X._maxInstanceCount!==void 0?X._maxInstanceCount:1/0,be=Math.min(X.instanceCount,Nt);ut.renderInstances(Ue,Mt,be)}else ut.render(Ue,Mt)};function vn(x,F,X){x.transparent===!0&&x.side===Bn&&x.forceSinglePass===!1?(x.side=Kt,x.needsUpdate=!0,er(x,F,X),x.side=ci,x.needsUpdate=!0,er(x,F,X),x.side=Bn):er(x,F,X)}this.compile=function(x,F,X=null){X===null&&(X=x),b=ve.get(X),b.init(F),M.push(b),X.traverseVisible(function(H){H.isLight&&H.layers.test(F.layers)&&(b.pushLight(H),H.castShadow&&b.pushShadow(H))}),x!==X&&x.traverseVisible(function(H){H.isLight&&H.layers.test(F.layers)&&(b.pushLight(H),H.castShadow&&b.pushShadow(H))}),b.setupLights();const k=new Set;return x.traverse(function(H){if(!(H.isMesh||H.isPoints||H.isLine||H.isSprite))return;const xe=H.material;if(xe)if(Array.isArray(xe))for(let Ae=0;Ae<xe.length;Ae++){const _e=xe[Ae];vn(_e,X,H),k.add(_e)}else vn(xe,X,H),k.add(xe)}),b=M.pop(),k},this.compileAsync=function(x,F,X=null){const k=this.compile(x,F,X);return new Promise(H=>{function xe(){if(k.forEach(function(Ae){v.get(Ae).currentProgram.isReady()&&k.delete(Ae)}),k.size===0){H(x);return}setTimeout(xe,10)}le.get("KHR_parallel_shader_compile")!==null?xe():setTimeout(xe,10)})};let Ea=null;function Vu(x){Ea&&Ea(x)}function $l(){mi.stop()}function Yl(){mi.start()}const mi=new jh;mi.setAnimationLoop(Vu),typeof self<"u"&&mi.setContext(self),this.setAnimationLoop=function(x){Ea=x,pe.setAnimationLoop(x),x===null?mi.stop():mi.start()},pe.addEventListener("sessionstart",$l),pe.addEventListener("sessionend",Yl),this.render=function(x,F){if(F!==void 0&&F.isCamera!==!0){Qe("WebGLRenderer.render: camera is not an instance of THREE.Camera.");return}if(C===!0)return;z!==null&&z.renderStart(x,F);const X=pe.enabled===!0&&pe.isPresenting===!0,k=w!==null&&(O===null||X)&&w.begin(B,O);if(x.matrixWorldAutoUpdate===!0&&x.updateMatrixWorld(),F.parent===null&&F.matrixWorldAutoUpdate===!0&&F.updateMatrixWorld(),pe.enabled===!0&&pe.isPresenting===!0&&(w===null||w.isCompositing()===!1)&&(pe.cameraAutoUpdate===!0&&pe.updateCamera(F),F=pe.getCamera()),x.isScene===!0&&x.onBeforeRender(B,x,F,O),b=ve.get(x,M.length),b.init(F),b.state.textureUnits=f.getTextureUnits(),M.push(b),tt.multiplyMatrices(F.projectionMatrix,F.matrixWorldInverse),Ce.setFromProjectionMatrix(tt,wn,F.reversedDepth),Fe=this.localClippingEnabled,Ke=Te.init(this.clippingPlanes,Fe),L=Q.get(x,I.length),L.init(),I.push(L),pe.enabled===!0&&pe.isPresenting===!0){const Ae=B.xr.getDepthSensingMesh();Ae!==null&&ba(Ae,F,-1/0,B.sortObjects)}ba(x,F,0,B.sortObjects),L.finish(),B.sortObjects===!0&&L.sort(U,ie),G=pe.enabled===!1||pe.isPresenting===!1||pe.hasDepthSensing()===!1,G&&ue.addToRenderList(L,x),this.info.render.frame++,Ke===!0&&Te.beginShadows();const H=b.state.shadowsArray;if(fe.render(H,x,F),Ke===!0&&Te.endShadows(),this.info.autoReset===!0&&this.info.reset(),(k&&w.hasRenderPass())===!1){const Ae=L.opaque,_e=L.transmissive;if(b.setupLights(),F.isArrayCamera){const Le=F.cameras;if(_e.length>0)for(let Ie=0,Xe=Le.length;Ie<Xe;Ie++){const qe=Le[Ie];jl(Ae,_e,x,qe)}G&&ue.render(x);for(let Ie=0,Xe=Le.length;Ie<Xe;Ie++){const qe=Le[Ie];ql(L,x,qe,qe.viewport)}}else _e.length>0&&jl(Ae,_e,x,F),G&&ue.render(x),ql(L,x,F)}O!==null&&J===0&&(f.updateMultisampleRenderTarget(O),f.updateRenderTargetMipmap(O)),k&&w.end(B),x.isScene===!0&&x.onAfterRender(B,x,F),de.resetDefaultState(),W=-1,$=null,M.pop(),M.length>0?(b=M[M.length-1],f.setTextureUnits(b.state.textureUnits),Ke===!0&&Te.setGlobalState(B.clippingPlanes,b.state.camera)):b=null,I.pop(),I.length>0?L=I[I.length-1]:L=null,z!==null&&z.renderEnd()};function ba(x,F,X,k){if(x.visible===!1)return;if(x.layers.test(F.layers)){if(x.isGroup)X=x.renderOrder;else if(x.isLOD)x.autoUpdate===!0&&x.update(F);else if(x.isLightProbeGrid)b.pushLightProbeGrid(x);else if(x.isLight)b.pushLight(x),x.castShadow&&b.pushShadow(x);else if(x.isSprite){if(!x.frustumCulled||Ce.intersectsSprite(x)){k&&Ve.setFromMatrixPosition(x.matrixWorld).applyMatrix4(tt);const Ae=V.update(x),_e=x.material;_e.visible&&L.push(x,Ae,_e,X,Ve.z,null)}}else if((x.isMesh||x.isLine||x.isPoints)&&(!x.frustumCulled||Ce.intersectsObject(x))){const Ae=V.update(x),_e=x.material;if(k&&(x.boundingSphere!==void 0?(x.boundingSphere===null&&x.computeBoundingSphere(),Ve.copy(x.boundingSphere.center)):(Ae.boundingSphere===null&&Ae.computeBoundingSphere(),Ve.copy(Ae.boundingSphere.center)),Ve.applyMatrix4(x.matrixWorld).applyMatrix4(tt)),Array.isArray(_e)){const Le=Ae.groups;for(let Ie=0,Xe=Le.length;Ie<Xe;Ie++){const qe=Le[Ie],Ue=_e[qe.materialIndex];Ue&&Ue.visible&&L.push(x,Ae,Ue,X,Ve.z,qe)}}else _e.visible&&L.push(x,Ae,_e,X,Ve.z,null)}}const xe=x.children;for(let Ae=0,_e=xe.length;Ae<_e;Ae++)ba(xe[Ae],F,X,k)}function ql(x,F,X,k){const{opaque:H,transmissive:xe,transparent:Ae}=x;b.setupLightsView(X),Ke===!0&&Te.setGlobalState(B.clippingPlanes,X),k&&K.viewport(se.copy(k)),H.length>0&&Qs(H,F,X),xe.length>0&&Qs(xe,F,X),Ae.length>0&&Qs(Ae,F,X),K.buffers.depth.setTest(!0),K.buffers.depth.setMask(!0),K.buffers.color.setMask(!0),K.setPolygonOffset(!1)}function jl(x,F,X,k){if((X.isScene===!0?X.overrideMaterial:null)!==null)return;if(b.state.transmissionRenderTarget[k.id]===void 0){const Ue=le.has("EXT_color_buffer_half_float")||le.has("EXT_color_buffer_float");b.state.transmissionRenderTarget[k.id]=new Pn(1,1,{generateMipmaps:!0,type:Ue?Vn:an,minFilter:Ti,samples:Math.max(4,me.samples),stencilBuffer:r,resolveDepthBuffer:!1,resolveStencilBuffer:!1,colorSpace:Ze.workingColorSpace})}const xe=b.state.transmissionRenderTarget[k.id],Ae=k.viewport||se;xe.setSize(Ae.z*B.transmissionResolutionScale,Ae.w*B.transmissionResolutionScale);const _e=B.getRenderTarget(),Le=B.getActiveCubeFace(),Ie=B.getActiveMipmapLevel();B.setRenderTarget(xe),B.getClearColor(Ee),Pe=B.getClearAlpha(),Pe<1&&B.setClearColor(16777215,.5),B.clear(),G&&ue.render(X);const Xe=B.toneMapping;B.toneMapping=Cn;const qe=k.viewport;if(k.viewport!==void 0&&(k.viewport=void 0),b.setupLightsView(k),Ke===!0&&Te.setGlobalState(B.clippingPlanes,k),Qs(x,X,k),f.updateMultisampleRenderTarget(xe),f.updateRenderTargetMipmap(xe),le.has("WEBGL_multisampled_render_to_texture")===!1){let Ue=!1;for(let lt=0,Mt=F.length;lt<Mt;lt++){const _t=F[lt],{object:ut,geometry:Nt,material:be,group:Zt}=_t;if(be.side===Bn&&ut.layers.test(k.layers)){const nt=be.side;be.side=Kt,be.needsUpdate=!0,Kl(ut,X,k,Nt,be,Zt),be.side=nt,be.needsUpdate=!0,Ue=!0}}Ue===!0&&(f.updateMultisampleRenderTarget(xe),f.updateRenderTargetMipmap(xe))}B.setRenderTarget(_e,Le,Ie),B.setClearColor(Ee,Pe),qe!==void 0&&(k.viewport=qe),B.toneMapping=Xe}function Qs(x,F,X){const k=F.isScene===!0?F.overrideMaterial:null;for(let H=0,xe=x.length;H<xe;H++){const Ae=x[H],{object:_e,geometry:Le,group:Ie}=Ae;let Xe=Ae.material;Xe.allowOverride===!0&&k!==null&&(Xe=k),_e.layers.test(X.layers)&&Kl(_e,F,X,Le,Xe,Ie)}}function Kl(x,F,X,k,H,xe){x.onBeforeRender(B,F,X,k,H,xe),x.modelViewMatrix.multiplyMatrices(X.matrixWorldInverse,x.matrixWorld),x.normalMatrix.getNormalMatrix(x.modelViewMatrix),H.onBeforeRender(B,F,X,k,x,xe),H.transparent===!0&&H.side===Bn&&H.forceSinglePass===!1?(H.side=Kt,H.needsUpdate=!0,B.renderBufferDirect(X,F,k,H,x,xe),H.side=ci,H.needsUpdate=!0,B.renderBufferDirect(X,F,k,H,x,xe),H.side=Bn):B.renderBufferDirect(X,F,k,H,x,xe),x.onAfterRender(B,F,X,k,H,xe)}function er(x,F,X){F.isScene!==!0&&(F=ft);const k=v.get(x),H=b.state.lights,xe=b.state.shadowsArray,Ae=H.state.version,_e=te.getParameters(x,H.state,xe,F,X,b.state.lightProbeGridArray),Le=te.getProgramCacheKey(_e);let Ie=k.programs;k.environment=x.isMeshStandardMaterial||x.isMeshLambertMaterial||x.isMeshPhongMaterial?F.environment:null,k.fog=F.fog;const Xe=x.isMeshStandardMaterial||x.isMeshLambertMaterial&&!x.envMap||x.isMeshPhongMaterial&&!x.envMap;k.envMap=D.get(x.envMap||k.environment,Xe),k.envMapRotation=k.environment!==null&&x.envMap===null?F.environmentRotation:x.envMapRotation,Ie===void 0&&(x.addEventListener("dispose",vt),Ie=new Map,k.programs=Ie);let qe=Ie.get(Le);if(qe!==void 0){if(k.currentProgram===qe&&k.lightsStateVersion===Ae)return Jl(x,_e),qe}else _e.uniforms=te.getUniforms(x),z!==null&&x.isNodeMaterial&&z.build(x,X,_e),x.onBeforeCompile(_e,B),qe=te.acquireProgram(_e,Le),Ie.set(Le,qe),k.uniforms=_e.uniforms;const Ue=k.uniforms;return(!x.isShaderMaterial&&!x.isRawShaderMaterial||x.clipping===!0)&&(Ue.clippingPlanes=Te.uniform),Jl(x,_e),k.needsLights=Yu(x),k.lightsStateVersion=Ae,k.needsLights&&(Ue.ambientLightColor.value=H.state.ambient,Ue.lightProbe.value=H.state.probe,Ue.directionalLights.value=H.state.directional,Ue.directionalLightShadows.value=H.state.directionalShadow,Ue.spotLights.value=H.state.spot,Ue.spotLightShadows.value=H.state.spotShadow,Ue.rectAreaLights.value=H.state.rectArea,Ue.ltc_1.value=H.state.rectAreaLTC1,Ue.ltc_2.value=H.state.rectAreaLTC2,Ue.pointLights.value=H.state.point,Ue.pointLightShadows.value=H.state.pointShadow,Ue.hemisphereLights.value=H.state.hemi,Ue.directionalShadowMatrix.value=H.state.directionalShadowMatrix,Ue.spotLightMatrix.value=H.state.spotLightMatrix,Ue.spotLightMap.value=H.state.spotLightMap,Ue.pointShadowMatrix.value=H.state.pointShadowMatrix),k.lightProbeGrid=b.state.lightProbeGridArray.length>0,k.currentProgram=qe,k.uniformsList=null,qe}function Zl(x){if(x.uniformsList===null){const F=x.currentProgram.getUniforms();x.uniformsList=Xr.seqWithValue(F.seq,x.uniforms)}return x.uniformsList}function Jl(x,F){const X=v.get(x);X.outputColorSpace=F.outputColorSpace,X.batching=F.batching,X.batchingColor=F.batchingColor,X.instancing=F.instancing,X.instancingColor=F.instancingColor,X.instancingMorph=F.instancingMorph,X.skinning=F.skinning,X.morphTargets=F.morphTargets,X.morphNormals=F.morphNormals,X.morphColors=F.morphColors,X.morphTargetsCount=F.morphTargetsCount,X.numClippingPlanes=F.numClippingPlanes,X.numIntersection=F.numClipIntersection,X.vertexAlphas=F.vertexAlphas,X.vertexTangents=F.vertexTangents,X.toneMapping=F.toneMapping}function Wu(x,F){if(x.length===0)return null;if(x.length===1)return x[0].texture!==null?x[0]:null;T.setFromMatrixPosition(F.matrixWorld);for(let X=0,k=x.length;X<k;X++){const H=x[X];if(H.texture!==null&&H.boundingBox.containsPoint(T))return H}return null}function Xu(x,F,X,k,H){F.isScene!==!0&&(F=ft),f.resetTextureUnits();const xe=F.fog,Ae=k.isMeshStandardMaterial||k.isMeshLambertMaterial||k.isMeshPhongMaterial?F.environment:null,_e=O===null?B.outputColorSpace:O.isXRRenderTarget===!0?O.texture.colorSpace:Ze.workingColorSpace,Le=k.isMeshStandardMaterial||k.isMeshLambertMaterial&&!k.envMap||k.isMeshPhongMaterial&&!k.envMap,Ie=D.get(k.envMap||Ae,Le),Xe=k.vertexColors===!0&&!!X.attributes.color&&X.attributes.color.itemSize===4,qe=!!X.attributes.tangent&&(!!k.normalMap||k.anisotropy>0),Ue=!!X.morphAttributes.position,lt=!!X.morphAttributes.normal,Mt=!!X.morphAttributes.color;let _t=Cn;k.toneMapped&&(O===null||O.isXRRenderTarget===!0)&&(_t=B.toneMapping);const ut=X.morphAttributes.position||X.morphAttributes.normal||X.morphAttributes.color,Nt=ut!==void 0?ut.length:0,be=v.get(k),Zt=b.state.lights;if(Ke===!0&&(Fe===!0||x!==$)){const pt=x===$&&k.id===W;Te.setState(k,x,pt)}let nt=!1;k.version===be.__version?(be.needsLights&&be.lightsStateVersion!==Zt.state.version||be.outputColorSpace!==_e||H.isBatchedMesh&&be.batching===!1||!H.isBatchedMesh&&be.batching===!0||H.isBatchedMesh&&be.batchingColor===!0&&H.colorTexture===null||H.isBatchedMesh&&be.batchingColor===!1&&H.colorTexture!==null||H.isInstancedMesh&&be.instancing===!1||!H.isInstancedMesh&&be.instancing===!0||H.isSkinnedMesh&&be.skinning===!1||!H.isSkinnedMesh&&be.skinning===!0||H.isInstancedMesh&&be.instancingColor===!0&&H.instanceColor===null||H.isInstancedMesh&&be.instancingColor===!1&&H.instanceColor!==null||H.isInstancedMesh&&be.instancingMorph===!0&&H.morphTexture===null||H.isInstancedMesh&&be.instancingMorph===!1&&H.morphTexture!==null||be.envMap!==Ie||k.fog===!0&&be.fog!==xe||be.numClippingPlanes!==void 0&&(be.numClippingPlanes!==Te.numPlanes||be.numIntersection!==Te.numIntersection)||be.vertexAlphas!==Xe||be.vertexTangents!==qe||be.morphTargets!==Ue||be.morphNormals!==lt||be.morphColors!==Mt||be.toneMapping!==_t||be.morphTargetsCount!==Nt||!!be.lightProbeGrid!=b.state.lightProbeGridArray.length>0)&&(nt=!0):(nt=!0,be.__version=k.version);let tn=be.currentProgram;nt===!0&&(tn=er(k,F,H),z&&k.isNodeMaterial&&z.onUpdateProgram(k,tn,be));let Mn=!1,qn=!1,Oi=!1;const dt=tn.getUniforms(),St=be.uniforms;if(K.useProgram(tn.program)&&(Mn=!0,qn=!0,Oi=!0),k.id!==W&&(W=k.id,qn=!0),be.needsLights){const pt=Wu(b.state.lightProbeGridArray,H);be.lightProbeGrid!==pt&&(be.lightProbeGrid=pt,qn=!0)}if(Mn||$!==x){K.buffers.depth.getReversed()&&x.reversedDepth!==!0&&(x._reversedDepth=!0,x.updateProjectionMatrix()),dt.setValue(E,"projectionMatrix",x.projectionMatrix),dt.setValue(E,"viewMatrix",x.matrixWorldInverse);const Kn=dt.map.cameraPosition;Kn!==void 0&&Kn.setValue(E,rt.setFromMatrixPosition(x.matrixWorld)),me.logarithmicDepthBuffer&&dt.setValue(E,"logDepthBufFC",2/(Math.log(x.far+1)/Math.LN2)),(k.isMeshPhongMaterial||k.isMeshToonMaterial||k.isMeshLambertMaterial||k.isMeshBasicMaterial||k.isMeshStandardMaterial||k.isShaderMaterial)&&dt.setValue(E,"isOrthographic",x.isOrthographicCamera===!0),$!==x&&($=x,qn=!0,Oi=!0)}if(be.needsLights&&(Zt.state.directionalShadowMap.length>0&&dt.setValue(E,"directionalShadowMap",Zt.state.directionalShadowMap,f),Zt.state.spotShadowMap.length>0&&dt.setValue(E,"spotShadowMap",Zt.state.spotShadowMap,f),Zt.state.pointShadowMap.length>0&&dt.setValue(E,"pointShadowMap",Zt.state.pointShadowMap,f)),H.isSkinnedMesh){dt.setOptional(E,H,"bindMatrix"),dt.setOptional(E,H,"bindMatrixInverse");const pt=H.skeleton;pt&&(pt.boneTexture===null&&pt.computeBoneTexture(),dt.setValue(E,"boneTexture",pt.boneTexture,f))}H.isBatchedMesh&&(dt.setOptional(E,H,"batchingTexture"),dt.setValue(E,"batchingTexture",H._matricesTexture,f),dt.setOptional(E,H,"batchingIdTexture"),dt.setValue(E,"batchingIdTexture",H._indirectTexture,f),dt.setOptional(E,H,"batchingColorTexture"),H._colorsTexture!==null&&dt.setValue(E,"batchingColorTexture",H._colorsTexture,f));const jn=X.morphAttributes;if((jn.position!==void 0||jn.normal!==void 0||jn.color!==void 0)&&Be.update(H,X,tn),(qn||be.receiveShadow!==H.receiveShadow)&&(be.receiveShadow=H.receiveShadow,dt.setValue(E,"receiveShadow",H.receiveShadow)),(k.isMeshStandardMaterial||k.isMeshLambertMaterial||k.isMeshPhongMaterial)&&k.envMap===null&&F.environment!==null&&(St.envMapIntensity.value=F.environmentIntensity),St.dfgLUT!==void 0&&(St.dfgLUT.value=m0()),qn){if(dt.setValue(E,"toneMappingExposure",B.toneMappingExposure),be.needsLights&&$u(St,Oi),xe&&k.fog===!0&&Y.refreshFogUniforms(St,xe),Y.refreshMaterialUniforms(St,k,R,Je,b.state.transmissionRenderTarget[x.id]),be.needsLights&&be.lightProbeGrid){const pt=be.lightProbeGrid;St.probesSH.value=pt.texture,St.probesMin.value.copy(pt.boundingBox.min),St.probesMax.value.copy(pt.boundingBox.max),St.probesResolution.value.copy(pt.resolution)}Xr.upload(E,Zl(be),St,f)}if(k.isShaderMaterial&&k.uniformsNeedUpdate===!0&&(Xr.upload(E,Zl(be),St,f),k.uniformsNeedUpdate=!1),k.isSpriteMaterial&&dt.setValue(E,"center",H.center),dt.setValue(E,"modelViewMatrix",H.modelViewMatrix),dt.setValue(E,"normalMatrix",H.normalMatrix),dt.setValue(E,"modelMatrix",H.matrixWorld),k.uniformsGroups!==void 0){const pt=k.uniformsGroups;for(let Kn=0,Bi=pt.length;Kn<Bi;Kn++){const Ql=pt[Kn];ee.update(Ql,tn),ee.bind(Ql,tn)}}return tn}function $u(x,F){x.ambientLightColor.needsUpdate=F,x.lightProbe.needsUpdate=F,x.directionalLights.needsUpdate=F,x.directionalLightShadows.needsUpdate=F,x.pointLights.needsUpdate=F,x.pointLightShadows.needsUpdate=F,x.spotLights.needsUpdate=F,x.spotLightShadows.needsUpdate=F,x.rectAreaLights.needsUpdate=F,x.hemisphereLights.needsUpdate=F}function Yu(x){return x.isMeshLambertMaterial||x.isMeshToonMaterial||x.isMeshPhongMaterial||x.isMeshStandardMaterial||x.isShadowMaterial||x.isShaderMaterial&&x.lights===!0}this.getActiveCubeFace=function(){return Z},this.getActiveMipmapLevel=function(){return J},this.getRenderTarget=function(){return O},this.setRenderTargetTextures=function(x,F,X){const k=v.get(x);k.__autoAllocateDepthBuffer=x.resolveDepthBuffer===!1,k.__autoAllocateDepthBuffer===!1&&(k.__useRenderToTexture=!1),v.get(x.texture).__webglTexture=F,v.get(x.depthTexture).__webglTexture=k.__autoAllocateDepthBuffer?void 0:X,k.__hasExternalTextures=!0},this.setRenderTargetFramebuffer=function(x,F){const X=v.get(x);X.__webglFramebuffer=F,X.__useDefaultFramebuffer=F===void 0};const qu=E.createFramebuffer();this.setRenderTarget=function(x,F=0,X=0){O=x,Z=F,J=X;let k=null,H=!1,xe=!1;if(x){const _e=v.get(x);if(_e.__useDefaultFramebuffer!==void 0){K.bindFramebuffer(E.FRAMEBUFFER,_e.__webglFramebuffer),se.copy(x.viewport),oe.copy(x.scissor),he=x.scissorTest,K.viewport(se),K.scissor(oe),K.setScissorTest(he),W=-1;return}else if(_e.__webglFramebuffer===void 0)f.setupRenderTarget(x);else if(_e.__hasExternalTextures)f.rebindTextures(x,v.get(x.texture).__webglTexture,v.get(x.depthTexture).__webglTexture);else if(x.depthBuffer){const Xe=x.depthTexture;if(_e.__boundDepthTexture!==Xe){if(Xe!==null&&v.has(Xe)&&(x.width!==Xe.image.width||x.height!==Xe.image.height))throw new Error("WebGLRenderTarget: Attached DepthTexture is initialized to the incorrect size.");f.setupDepthRenderbuffer(x)}}const Le=x.texture;(Le.isData3DTexture||Le.isDataArrayTexture||Le.isCompressedArrayTexture)&&(xe=!0);const Ie=v.get(x).__webglFramebuffer;x.isWebGLCubeRenderTarget?(Array.isArray(Ie[F])?k=Ie[F][X]:k=Ie[F],H=!0):x.samples>0&&f.useMultisampledRTT(x)===!1?k=v.get(x).__webglMultisampledFramebuffer:Array.isArray(Ie)?k=Ie[X]:k=Ie,se.copy(x.viewport),oe.copy(x.scissor),he=x.scissorTest}else se.copy(re).multiplyScalar(R).floor(),oe.copy(Me).multiplyScalar(R).floor(),he=Se;if(X!==0&&(k=qu),K.bindFramebuffer(E.FRAMEBUFFER,k)&&K.drawBuffers(x,k),K.viewport(se),K.scissor(oe),K.setScissorTest(he),H){const _e=v.get(x.texture);E.framebufferTexture2D(E.FRAMEBUFFER,E.COLOR_ATTACHMENT0,E.TEXTURE_CUBE_MAP_POSITIVE_X+F,_e.__webglTexture,X)}else if(xe){const _e=F;for(let Le=0;Le<x.textures.length;Le++){const Ie=v.get(x.textures[Le]);E.framebufferTextureLayer(E.FRAMEBUFFER,E.COLOR_ATTACHMENT0+Le,Ie.__webglTexture,X,_e)}}else if(x!==null&&X!==0){const _e=v.get(x.texture);E.framebufferTexture2D(E.FRAMEBUFFER,E.COLOR_ATTACHMENT0,E.TEXTURE_2D,_e.__webglTexture,X)}W=-1},this.readRenderTargetPixels=function(x,F,X,k,H,xe,Ae,_e=0){if(!(x&&x.isWebGLRenderTarget)){Qe("WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.");return}let Le=v.get(x).__webglFramebuffer;if(x.isWebGLCubeRenderTarget&&Ae!==void 0&&(Le=Le[Ae]),Le){K.bindFramebuffer(E.FRAMEBUFFER,Le);try{const Ie=x.textures[_e],Xe=Ie.format,qe=Ie.type;if(x.textures.length>1&&E.readBuffer(E.COLOR_ATTACHMENT0+_e),!me.textureFormatReadable(Xe)){Qe("WebGLRenderer.readRenderTargetPixels: renderTarget is not in RGBA or implementation defined format.");return}if(!me.textureTypeReadable(qe)){Qe("WebGLRenderer.readRenderTargetPixels: renderTarget is not in UnsignedByteType or implementation defined type.");return}F>=0&&F<=x.width-k&&X>=0&&X<=x.height-H&&E.readPixels(F,X,k,H,P.convert(Xe),P.convert(qe),xe)}finally{const Ie=O!==null?v.get(O).__webglFramebuffer:null;K.bindFramebuffer(E.FRAMEBUFFER,Ie)}}},this.readRenderTargetPixelsAsync=async function(x,F,X,k,H,xe,Ae,_e=0){if(!(x&&x.isWebGLRenderTarget))throw new Error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.");let Le=v.get(x).__webglFramebuffer;if(x.isWebGLCubeRenderTarget&&Ae!==void 0&&(Le=Le[Ae]),Le)if(F>=0&&F<=x.width-k&&X>=0&&X<=x.height-H){K.bindFramebuffer(E.FRAMEBUFFER,Le);const Ie=x.textures[_e],Xe=Ie.format,qe=Ie.type;if(x.textures.length>1&&E.readBuffer(E.COLOR_ATTACHMENT0+_e),!me.textureFormatReadable(Xe))throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in RGBA or implementation defined format.");if(!me.textureTypeReadable(qe))throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in UnsignedByteType or implementation defined type.");const Ue=E.createBuffer();E.bindBuffer(E.PIXEL_PACK_BUFFER,Ue),E.bufferData(E.PIXEL_PACK_BUFFER,xe.byteLength,E.STREAM_READ),E.readPixels(F,X,k,H,P.convert(Xe),P.convert(qe),0);const lt=O!==null?v.get(O).__webglFramebuffer:null;K.bindFramebuffer(E.FRAMEBUFFER,lt);const Mt=E.fenceSync(E.SYNC_GPU_COMMANDS_COMPLETE,0);return E.flush(),await Id(E,Mt,4),E.bindBuffer(E.PIXEL_PACK_BUFFER,Ue),E.getBufferSubData(E.PIXEL_PACK_BUFFER,0,xe),E.deleteBuffer(Ue),E.deleteSync(Mt),xe}else throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: requested read bounds are out of range.")},this.copyFramebufferToTexture=function(x,F=null,X=0){const k=Math.pow(2,-X),H=Math.floor(x.image.width*k),xe=Math.floor(x.image.height*k),Ae=F!==null?F.x:0,_e=F!==null?F.y:0;f.setTexture2D(x,0),E.copyTexSubImage2D(E.TEXTURE_2D,X,0,0,Ae,_e,H,xe),K.unbindTexture()};const ju=E.createFramebuffer(),Ku=E.createFramebuffer();this.copyTextureToTexture=function(x,F,X=null,k=null,H=0,xe=0){let Ae,_e,Le,Ie,Xe,qe,Ue,lt,Mt;const _t=x.isCompressedTexture?x.mipmaps[xe]:x.image;if(X!==null)Ae=X.max.x-X.min.x,_e=X.max.y-X.min.y,Le=X.isBox3?X.max.z-X.min.z:1,Ie=X.min.x,Xe=X.min.y,qe=X.isBox3?X.min.z:0;else{const St=Math.pow(2,-H);Ae=Math.floor(_t.width*St),_e=Math.floor(_t.height*St),x.isDataArrayTexture?Le=_t.depth:x.isData3DTexture?Le=Math.floor(_t.depth*St):Le=1,Ie=0,Xe=0,qe=0}k!==null?(Ue=k.x,lt=k.y,Mt=k.z):(Ue=0,lt=0,Mt=0);const ut=P.convert(F.format),Nt=P.convert(F.type);let be;F.isData3DTexture?(f.setTexture3D(F,0),be=E.TEXTURE_3D):F.isDataArrayTexture||F.isCompressedArrayTexture?(f.setTexture2DArray(F,0),be=E.TEXTURE_2D_ARRAY):(f.setTexture2D(F,0),be=E.TEXTURE_2D),K.activeTexture(E.TEXTURE0),K.pixelStorei(E.UNPACK_FLIP_Y_WEBGL,F.flipY),K.pixelStorei(E.UNPACK_PREMULTIPLY_ALPHA_WEBGL,F.premultiplyAlpha),K.pixelStorei(E.UNPACK_ALIGNMENT,F.unpackAlignment);const Zt=K.getParameter(E.UNPACK_ROW_LENGTH),nt=K.getParameter(E.UNPACK_IMAGE_HEIGHT),tn=K.getParameter(E.UNPACK_SKIP_PIXELS),Mn=K.getParameter(E.UNPACK_SKIP_ROWS),qn=K.getParameter(E.UNPACK_SKIP_IMAGES);K.pixelStorei(E.UNPACK_ROW_LENGTH,_t.width),K.pixelStorei(E.UNPACK_IMAGE_HEIGHT,_t.height),K.pixelStorei(E.UNPACK_SKIP_PIXELS,Ie),K.pixelStorei(E.UNPACK_SKIP_ROWS,Xe),K.pixelStorei(E.UNPACK_SKIP_IMAGES,qe);const Oi=x.isDataArrayTexture||x.isData3DTexture,dt=F.isDataArrayTexture||F.isData3DTexture;if(x.isDepthTexture){const St=v.get(x),jn=v.get(F),pt=v.get(St.__renderTarget),Kn=v.get(jn.__renderTarget);K.bindFramebuffer(E.READ_FRAMEBUFFER,pt.__webglFramebuffer),K.bindFramebuffer(E.DRAW_FRAMEBUFFER,Kn.__webglFramebuffer);for(let Bi=0;Bi<Le;Bi++)Oi&&(E.framebufferTextureLayer(E.READ_FRAMEBUFFER,E.COLOR_ATTACHMENT0,v.get(x).__webglTexture,H,qe+Bi),E.framebufferTextureLayer(E.DRAW_FRAMEBUFFER,E.COLOR_ATTACHMENT0,v.get(F).__webglTexture,xe,Mt+Bi)),E.blitFramebuffer(Ie,Xe,Ae,_e,Ue,lt,Ae,_e,E.DEPTH_BUFFER_BIT,E.NEAREST);K.bindFramebuffer(E.READ_FRAMEBUFFER,null),K.bindFramebuffer(E.DRAW_FRAMEBUFFER,null)}else if(H!==0||x.isRenderTargetTexture||v.has(x)){const St=v.get(x),jn=v.get(F);K.bindFramebuffer(E.READ_FRAMEBUFFER,ju),K.bindFramebuffer(E.DRAW_FRAMEBUFFER,Ku);for(let pt=0;pt<Le;pt++)Oi?E.framebufferTextureLayer(E.READ_FRAMEBUFFER,E.COLOR_ATTACHMENT0,St.__webglTexture,H,qe+pt):E.framebufferTexture2D(E.READ_FRAMEBUFFER,E.COLOR_ATTACHMENT0,E.TEXTURE_2D,St.__webglTexture,H),dt?E.framebufferTextureLayer(E.DRAW_FRAMEBUFFER,E.COLOR_ATTACHMENT0,jn.__webglTexture,xe,Mt+pt):E.framebufferTexture2D(E.DRAW_FRAMEBUFFER,E.COLOR_ATTACHMENT0,E.TEXTURE_2D,jn.__webglTexture,xe),H!==0?E.blitFramebuffer(Ie,Xe,Ae,_e,Ue,lt,Ae,_e,E.COLOR_BUFFER_BIT,E.NEAREST):dt?E.copyTexSubImage3D(be,xe,Ue,lt,Mt+pt,Ie,Xe,Ae,_e):E.copyTexSubImage2D(be,xe,Ue,lt,Ie,Xe,Ae,_e);K.bindFramebuffer(E.READ_FRAMEBUFFER,null),K.bindFramebuffer(E.DRAW_FRAMEBUFFER,null)}else dt?x.isDataTexture||x.isData3DTexture?E.texSubImage3D(be,xe,Ue,lt,Mt,Ae,_e,Le,ut,Nt,_t.data):F.isCompressedArrayTexture?E.compressedTexSubImage3D(be,xe,Ue,lt,Mt,Ae,_e,Le,ut,_t.data):E.texSubImage3D(be,xe,Ue,lt,Mt,Ae,_e,Le,ut,Nt,_t):x.isDataTexture?E.texSubImage2D(E.TEXTURE_2D,xe,Ue,lt,Ae,_e,ut,Nt,_t.data):x.isCompressedTexture?E.compressedTexSubImage2D(E.TEXTURE_2D,xe,Ue,lt,_t.width,_t.height,ut,_t.data):E.texSubImage2D(E.TEXTURE_2D,xe,Ue,lt,Ae,_e,ut,Nt,_t);K.pixelStorei(E.UNPACK_ROW_LENGTH,Zt),K.pixelStorei(E.UNPACK_IMAGE_HEIGHT,nt),K.pixelStorei(E.UNPACK_SKIP_PIXELS,tn),K.pixelStorei(E.UNPACK_SKIP_ROWS,Mn),K.pixelStorei(E.UNPACK_SKIP_IMAGES,qn),xe===0&&F.generateMipmaps&&E.generateMipmap(be),K.unbindTexture()},this.initRenderTarget=function(x){v.get(x).__webglFramebuffer===void 0&&f.setupRenderTarget(x)},this.initTexture=function(x){x.isCubeTexture?f.setTextureCube(x,0):x.isData3DTexture?f.setTexture3D(x,0):x.isDataArrayTexture||x.isCompressedArrayTexture?f.setTexture2DArray(x,0):f.setTexture2D(x,0),K.unbindTexture()},this.resetState=function(){Z=0,J=0,O=null,K.reset(),de.reset()},typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}get coordinateSystem(){return wn}get outputColorSpace(){return this._outputColorSpace}set outputColorSpace(e){this._outputColorSpace=e;const t=this.getContext();t.drawingBufferColorSpace=Ze._getDrawingBufferColorSpace(e),t.unpackColorSpace=Ze._getUnpackColorSpace()}}const ih={type:"change"},Cl={type:"start"},iu={type:"end"},Lr=new _a,sh=new ii,_0=Math.cos(70*Fd.DEG2RAD),wt=new N,Yt=2*Math.PI,ht={NONE:-1,ROTATE:0,DOLLY:1,PAN:2,TOUCH_ROTATE:3,TOUCH_PAN:4,TOUCH_DOLLY_PAN:5,TOUCH_DOLLY_ROTATE:6},ro=1e-6;class x0 extends Mf{constructor(e,t=null){super(e,t),this.state=ht.NONE,this.target=new N,this.cursor=new N,this.minDistance=0,this.maxDistance=1/0,this.minZoom=0,this.maxZoom=1/0,this.minTargetRadius=0,this.maxTargetRadius=1/0,this.minPolarAngle=0,this.maxPolarAngle=Math.PI,this.minAzimuthAngle=-1/0,this.maxAzimuthAngle=1/0,this.enableDamping=!1,this.dampingFactor=.05,this.enableZoom=!0,this.zoomSpeed=1,this.enableRotate=!0,this.rotateSpeed=1,this.keyRotateSpeed=1,this.enablePan=!0,this.panSpeed=1,this.screenSpacePanning=!0,this.keyPanSpeed=7,this.zoomToCursor=!1,this.autoRotate=!1,this.autoRotateSpeed=2,this.keys={LEFT:"ArrowLeft",UP:"ArrowUp",RIGHT:"ArrowRight",BOTTOM:"ArrowDown"},this.mouseButtons={LEFT:cs.ROTATE,MIDDLE:cs.DOLLY,RIGHT:cs.PAN},this.touches={ONE:ss.ROTATE,TWO:ss.DOLLY_PAN},this.target0=this.target.clone(),this.position0=this.object.position.clone(),this.zoom0=this.object.zoom,this._cursorStyle="auto",this._domElementKeyEvents=null,this._lastPosition=new N,this._lastQuaternion=new hi,this._lastTargetPosition=new N,this._quat=new hi().setFromUnitVectors(e.up,new N(0,1,0)),this._quatInverse=this._quat.clone().invert(),this._spherical=new Dc,this._sphericalDelta=new Dc,this._scale=1,this._panOffset=new N,this._rotateStart=new Oe,this._rotateEnd=new Oe,this._rotateDelta=new Oe,this._panStart=new Oe,this._panEnd=new Oe,this._panDelta=new Oe,this._dollyStart=new Oe,this._dollyEnd=new Oe,this._dollyDelta=new Oe,this._dollyDirection=new N,this._mouse=new Oe,this._performCursorZoom=!1,this._pointers=[],this._pointerPositions={},this._controlActive=!1,this._onPointerMove=M0.bind(this),this._onPointerDown=v0.bind(this),this._onPointerUp=S0.bind(this),this._onContextMenu=R0.bind(this),this._onMouseWheel=b0.bind(this),this._onKeyDown=T0.bind(this),this._onTouchStart=A0.bind(this),this._onTouchMove=w0.bind(this),this._onMouseDown=y0.bind(this),this._onMouseMove=E0.bind(this),this._interceptControlDown=C0.bind(this),this._interceptControlUp=P0.bind(this),this.domElement!==null&&this.connect(this.domElement),this.update()}set cursorStyle(e){this._cursorStyle=e,e==="grab"?this.domElement.style.cursor="grab":this.domElement.style.cursor="auto"}get cursorStyle(){return this._cursorStyle}connect(e){super.connect(e),this.domElement.addEventListener("pointerdown",this._onPointerDown),this.domElement.addEventListener("pointercancel",this._onPointerUp),this.domElement.addEventListener("contextmenu",this._onContextMenu),this.domElement.addEventListener("wheel",this._onMouseWheel,{passive:!1}),this.domElement.getRootNode().addEventListener("keydown",this._interceptControlDown,{passive:!0,capture:!0}),this.domElement.style.touchAction="none"}disconnect(){this.domElement.removeEventListener("pointerdown",this._onPointerDown),this.domElement.ownerDocument.removeEventListener("pointermove",this._onPointerMove),this.domElement.ownerDocument.removeEventListener("pointerup",this._onPointerUp),this.domElement.removeEventListener("pointercancel",this._onPointerUp),this.domElement.removeEventListener("wheel",this._onMouseWheel),this.domElement.removeEventListener("contextmenu",this._onContextMenu),this.stopListenToKeyEvents(),this.domElement.getRootNode().removeEventListener("keydown",this._interceptControlDown,{capture:!0}),this.domElement.style.touchAction=""}dispose(){this.disconnect()}getPolarAngle(){return this._spherical.phi}getAzimuthalAngle(){return this._spherical.theta}getDistance(){return this.object.position.distanceTo(this.target)}listenToKeyEvents(e){e.addEventListener("keydown",this._onKeyDown),this._domElementKeyEvents=e}stopListenToKeyEvents(){this._domElementKeyEvents!==null&&(this._domElementKeyEvents.removeEventListener("keydown",this._onKeyDown),this._domElementKeyEvents=null)}saveState(){this.target0.copy(this.target),this.position0.copy(this.object.position),this.zoom0=this.object.zoom}reset(){this.target.copy(this.target0),this.object.position.copy(this.position0),this.object.zoom=this.zoom0,this.object.updateProjectionMatrix(),this.dispatchEvent(ih),this.update(),this.state=ht.NONE}pan(e,t){this._pan(e,t),this.update()}dollyIn(e){this._dollyIn(e),this.update()}dollyOut(e){this._dollyOut(e),this.update()}rotateLeft(e){this._rotateLeft(e),this.update()}rotateUp(e){this._rotateUp(e),this.update()}update(e=null){const t=this.object.position;wt.copy(t).sub(this.target),wt.applyQuaternion(this._quat),this._spherical.setFromVector3(wt),this.autoRotate&&this.state===ht.NONE&&this._rotateLeft(this._getAutoRotationAngle(e)),this.enableDamping?(this._spherical.theta+=this._sphericalDelta.theta*this.dampingFactor,this._spherical.phi+=this._sphericalDelta.phi*this.dampingFactor):(this._spherical.theta+=this._sphericalDelta.theta,this._spherical.phi+=this._sphericalDelta.phi);let i=this.minAzimuthAngle,s=this.maxAzimuthAngle;isFinite(i)&&isFinite(s)&&(i<-Math.PI?i+=Yt:i>Math.PI&&(i-=Yt),s<-Math.PI?s+=Yt:s>Math.PI&&(s-=Yt),i<=s?this._spherical.theta=Math.max(i,Math.min(s,this._spherical.theta)):this._spherical.theta=this._spherical.theta>(i+s)/2?Math.max(i,this._spherical.theta):Math.min(s,this._spherical.theta)),this._spherical.phi=Math.max(this.minPolarAngle,Math.min(this.maxPolarAngle,this._spherical.phi)),this._spherical.makeSafe(),this.enableDamping===!0?this.target.addScaledVector(this._panOffset,this.dampingFactor):this.target.add(this._panOffset),this.target.sub(this.cursor),this.target.clampLength(this.minTargetRadius,this.maxTargetRadius),this.target.add(this.cursor);let r=!1;if(this.zoomToCursor&&this._performCursorZoom||this.object.isOrthographicCamera)this._spherical.radius=this._clampDistance(this._spherical.radius);else{const a=this._spherical.radius;this._spherical.radius=this._clampDistance(this._spherical.radius*this._scale),r=a!=this._spherical.radius}if(wt.setFromSpherical(this._spherical),wt.applyQuaternion(this._quatInverse),t.copy(this.target).add(wt),this.object.lookAt(this.target),this.enableDamping===!0?(this._sphericalDelta.theta*=1-this.dampingFactor,this._sphericalDelta.phi*=1-this.dampingFactor,this._panOffset.multiplyScalar(1-this.dampingFactor)):(this._sphericalDelta.set(0,0,0),this._panOffset.set(0,0,0)),this.zoomToCursor&&this._performCursorZoom){let a=null;if(this.object.isPerspectiveCamera){const o=wt.length();a=this._clampDistance(o*this._scale);const c=o-a;this.object.position.addScaledVector(this._dollyDirection,c),this.object.updateMatrixWorld(),r=!!c}else if(this.object.isOrthographicCamera){const o=new N(this._mouse.x,this._mouse.y,0);o.unproject(this.object);const c=this.object.zoom;this.object.zoom=Math.max(this.minZoom,Math.min(this.maxZoom,this.object.zoom/this._scale)),this.object.updateProjectionMatrix(),r=c!==this.object.zoom;const l=new N(this._mouse.x,this._mouse.y,0);l.unproject(this.object),this.object.position.sub(l).add(o),this.object.updateMatrixWorld(),a=wt.length()}else console.warn("WARNING: OrbitControls.js encountered an unknown camera type - zoom to cursor disabled."),this.zoomToCursor=!1;a!==null&&(this.screenSpacePanning?this.target.set(0,0,-1).transformDirection(this.object.matrix).multiplyScalar(a).add(this.object.position):(Lr.origin.copy(this.object.position),Lr.direction.set(0,0,-1).transformDirection(this.object.matrix),Math.abs(this.object.up.dot(Lr.direction))<_0?this.object.lookAt(this.target):(sh.setFromNormalAndCoplanarPoint(this.object.up,this.target),Lr.intersectPlane(sh,this.target))))}else if(this.object.isOrthographicCamera){const a=this.object.zoom;this.object.zoom=Math.max(this.minZoom,Math.min(this.maxZoom,this.object.zoom/this._scale)),a!==this.object.zoom&&(this.object.updateProjectionMatrix(),r=!0)}return this._scale=1,this._performCursorZoom=!1,r||this._lastPosition.distanceToSquared(this.object.position)>ro||8*(1-this._lastQuaternion.dot(this.object.quaternion))>ro||this._lastTargetPosition.distanceToSquared(this.target)>ro?(this.dispatchEvent(ih),this._lastPosition.copy(this.object.position),this._lastQuaternion.copy(this.object.quaternion),this._lastTargetPosition.copy(this.target),!0):!1}_getAutoRotationAngle(e){return e!==null?Yt/60*this.autoRotateSpeed*e:Yt/60/60*this.autoRotateSpeed}_getZoomScale(e){const t=Math.abs(e*.01);return Math.pow(.95,this.zoomSpeed*t)}_rotateLeft(e){this._sphericalDelta.theta-=e}_rotateUp(e){this._sphericalDelta.phi-=e}_panLeft(e,t){wt.setFromMatrixColumn(t,0),wt.multiplyScalar(-e),this._panOffset.add(wt)}_panUp(e,t){this.screenSpacePanning===!0?wt.setFromMatrixColumn(t,1):(wt.setFromMatrixColumn(t,0),wt.crossVectors(this.object.up,wt)),wt.multiplyScalar(e),this._panOffset.add(wt)}_pan(e,t){const i=this.domElement;if(this.object.isPerspectiveCamera){const s=this.object.position;wt.copy(s).sub(this.target);let r=wt.length();r*=Math.tan(this.object.fov/2*Math.PI/180),this._panLeft(2*e*r/i.clientHeight,this.object.matrix),this._panUp(2*t*r/i.clientHeight,this.object.matrix)}else this.object.isOrthographicCamera?(this._panLeft(e*(this.object.right-this.object.left)/this.object.zoom/i.clientWidth,this.object.matrix),this._panUp(t*(this.object.top-this.object.bottom)/this.object.zoom/i.clientHeight,this.object.matrix)):(console.warn("WARNING: OrbitControls.js encountered an unknown camera type - pan disabled."),this.enablePan=!1)}_dollyOut(e){this.object.isPerspectiveCamera||this.object.isOrthographicCamera?this._scale/=e:(console.warn("WARNING: OrbitControls.js encountered an unknown camera type - dolly/zoom disabled."),this.enableZoom=!1)}_dollyIn(e){this.object.isPerspectiveCamera||this.object.isOrthographicCamera?this._scale*=e:(console.warn("WARNING: OrbitControls.js encountered an unknown camera type - dolly/zoom disabled."),this.enableZoom=!1)}_updateZoomParameters(e,t){if(!this.zoomToCursor)return;this._performCursorZoom=!0;const i=this.domElement.getBoundingClientRect(),s=e-i.left,r=t-i.top,a=i.width,o=i.height;this._mouse.x=s/a*2-1,this._mouse.y=-(r/o)*2+1,this._dollyDirection.set(this._mouse.x,this._mouse.y,1).unproject(this.object).sub(this.object.position).normalize()}_clampDistance(e){return Math.max(this.minDistance,Math.min(this.maxDistance,e))}_handleMouseDownRotate(e){this._rotateStart.set(e.clientX,e.clientY)}_handleMouseDownDolly(e){this._updateZoomParameters(e.clientX,e.clientX),this._dollyStart.set(e.clientX,e.clientY)}_handleMouseDownPan(e){this._panStart.set(e.clientX,e.clientY)}_handleMouseMoveRotate(e){this._rotateEnd.set(e.clientX,e.clientY),this._rotateDelta.subVectors(this._rotateEnd,this._rotateStart).multiplyScalar(this.rotateSpeed);const t=this.domElement;this._rotateLeft(Yt*this._rotateDelta.x/t.clientHeight),this._rotateUp(Yt*this._rotateDelta.y/t.clientHeight),this._rotateStart.copy(this._rotateEnd),this.update()}_handleMouseMoveDolly(e){this._dollyEnd.set(e.clientX,e.clientY),this._dollyDelta.subVectors(this._dollyEnd,this._dollyStart),this._dollyDelta.y>0?this._dollyOut(this._getZoomScale(this._dollyDelta.y)):this._dollyDelta.y<0&&this._dollyIn(this._getZoomScale(this._dollyDelta.y)),this._dollyStart.copy(this._dollyEnd),this.update()}_handleMouseMovePan(e){this._panEnd.set(e.clientX,e.clientY),this._panDelta.subVectors(this._panEnd,this._panStart).multiplyScalar(this.panSpeed),this._pan(this._panDelta.x,this._panDelta.y),this._panStart.copy(this._panEnd),this.update()}_handleMouseWheel(e){this._updateZoomParameters(e.clientX,e.clientY),e.deltaY<0?this._dollyIn(this._getZoomScale(e.deltaY)):e.deltaY>0&&this._dollyOut(this._getZoomScale(e.deltaY)),this.update()}_handleKeyDown(e){let t=!1;switch(e.code){case this.keys.UP:e.ctrlKey||e.metaKey||e.shiftKey?this.enableRotate&&this._rotateUp(Yt*this.keyRotateSpeed/this.domElement.clientHeight):this.enablePan&&this._pan(0,this.keyPanSpeed),t=!0;break;case this.keys.BOTTOM:e.ctrlKey||e.metaKey||e.shiftKey?this.enableRotate&&this._rotateUp(-Yt*this.keyRotateSpeed/this.domElement.clientHeight):this.enablePan&&this._pan(0,-this.keyPanSpeed),t=!0;break;case this.keys.LEFT:e.ctrlKey||e.metaKey||e.shiftKey?this.enableRotate&&this._rotateLeft(Yt*this.keyRotateSpeed/this.domElement.clientHeight):this.enablePan&&this._pan(this.keyPanSpeed,0),t=!0;break;case this.keys.RIGHT:e.ctrlKey||e.metaKey||e.shiftKey?this.enableRotate&&this._rotateLeft(-Yt*this.keyRotateSpeed/this.domElement.clientHeight):this.enablePan&&this._pan(-this.keyPanSpeed,0),t=!0;break}t&&(e.preventDefault(),this.update())}_handleTouchStartRotate(e){if(this._pointers.length===1)this._rotateStart.set(e.pageX,e.pageY);else{const t=this._getSecondPointerPosition(e),i=.5*(e.pageX+t.x),s=.5*(e.pageY+t.y);this._rotateStart.set(i,s)}}_handleTouchStartPan(e){if(this._pointers.length===1)this._panStart.set(e.pageX,e.pageY);else{const t=this._getSecondPointerPosition(e),i=.5*(e.pageX+t.x),s=.5*(e.pageY+t.y);this._panStart.set(i,s)}}_handleTouchStartDolly(e){const t=this._getSecondPointerPosition(e),i=e.pageX-t.x,s=e.pageY-t.y,r=Math.sqrt(i*i+s*s);this._dollyStart.set(0,r)}_handleTouchStartDollyPan(e){this.enableZoom&&this._handleTouchStartDolly(e),this.enablePan&&this._handleTouchStartPan(e)}_handleTouchStartDollyRotate(e){this.enableZoom&&this._handleTouchStartDolly(e),this.enableRotate&&this._handleTouchStartRotate(e)}_handleTouchMoveRotate(e){if(this._pointers.length==1)this._rotateEnd.set(e.pageX,e.pageY);else{const i=this._getSecondPointerPosition(e),s=.5*(e.pageX+i.x),r=.5*(e.pageY+i.y);this._rotateEnd.set(s,r)}this._rotateDelta.subVectors(this._rotateEnd,this._rotateStart).multiplyScalar(this.rotateSpeed);const t=this.domElement;this._rotateLeft(Yt*this._rotateDelta.x/t.clientHeight),this._rotateUp(Yt*this._rotateDelta.y/t.clientHeight),this._rotateStart.copy(this._rotateEnd)}_handleTouchMovePan(e){if(this._pointers.length===1)this._panEnd.set(e.pageX,e.pageY);else{const t=this._getSecondPointerPosition(e),i=.5*(e.pageX+t.x),s=.5*(e.pageY+t.y);this._panEnd.set(i,s)}this._panDelta.subVectors(this._panEnd,this._panStart).multiplyScalar(this.panSpeed),this._pan(this._panDelta.x,this._panDelta.y),this._panStart.copy(this._panEnd)}_handleTouchMoveDolly(e){const t=this._getSecondPointerPosition(e),i=e.pageX-t.x,s=e.pageY-t.y,r=Math.sqrt(i*i+s*s);this._dollyEnd.set(0,r),this._dollyDelta.set(0,Math.pow(this._dollyEnd.y/this._dollyStart.y,this.zoomSpeed)),this._dollyOut(this._dollyDelta.y),this._dollyStart.copy(this._dollyEnd);const a=(e.pageX+t.x)*.5,o=(e.pageY+t.y)*.5;this._updateZoomParameters(a,o)}_handleTouchMoveDollyPan(e){this.enableZoom&&this._handleTouchMoveDolly(e),this.enablePan&&this._handleTouchMovePan(e)}_handleTouchMoveDollyRotate(e){this.enableZoom&&this._handleTouchMoveDolly(e),this.enableRotate&&this._handleTouchMoveRotate(e)}_addPointer(e){this._pointers.push(e.pointerId)}_removePointer(e){delete this._pointerPositions[e.pointerId];for(let t=0;t<this._pointers.length;t++)if(this._pointers[t]==e.pointerId){this._pointers.splice(t,1);return}}_isTrackingPointer(e){for(let t=0;t<this._pointers.length;t++)if(this._pointers[t]==e.pointerId)return!0;return!1}_trackPointer(e){let t=this._pointerPositions[e.pointerId];t===void 0&&(t=new Oe,this._pointerPositions[e.pointerId]=t),t.set(e.pageX,e.pageY)}_getSecondPointerPosition(e){const t=e.pointerId===this._pointers[0]?this._pointers[1]:this._pointers[0];return this._pointerPositions[t]}_customWheelEvent(e){const t=e.deltaMode,i={clientX:e.clientX,clientY:e.clientY,deltaY:e.deltaY};switch(t){case 1:i.deltaY*=16;break;case 2:i.deltaY*=100;break}return e.ctrlKey&&!this._controlActive&&(i.deltaY*=10),i}}function v0(n){this.enabled!==!1&&(this._pointers.length===0&&(this.domElement.setPointerCapture(n.pointerId),this.domElement.ownerDocument.addEventListener("pointermove",this._onPointerMove),this.domElement.ownerDocument.addEventListener("pointerup",this._onPointerUp)),!this._isTrackingPointer(n)&&(this._addPointer(n),n.pointerType==="touch"?this._onTouchStart(n):this._onMouseDown(n),this._cursorStyle==="grab"&&(this.domElement.style.cursor="grabbing")))}function M0(n){this.enabled!==!1&&(n.pointerType==="touch"?this._onTouchMove(n):this._onMouseMove(n))}function S0(n){switch(this._removePointer(n),this._pointers.length){case 0:this.domElement.releasePointerCapture(n.pointerId),this.domElement.ownerDocument.removeEventListener("pointermove",this._onPointerMove),this.domElement.ownerDocument.removeEventListener("pointerup",this._onPointerUp),this.dispatchEvent(iu),this.state=ht.NONE,this._cursorStyle==="grab"&&(this.domElement.style.cursor="grab");break;case 1:const e=this._pointers[0],t=this._pointerPositions[e];this._onTouchStart({pointerId:e,pageX:t.x,pageY:t.y});break}}function y0(n){let e;switch(n.button){case 0:e=this.mouseButtons.LEFT;break;case 1:e=this.mouseButtons.MIDDLE;break;case 2:e=this.mouseButtons.RIGHT;break;default:e=-1}switch(e){case cs.DOLLY:if(this.enableZoom===!1)return;this._handleMouseDownDolly(n),this.state=ht.DOLLY;break;case cs.ROTATE:if(n.ctrlKey||n.metaKey||n.shiftKey){if(this.enablePan===!1)return;this._handleMouseDownPan(n),this.state=ht.PAN}else{if(this.enableRotate===!1)return;this._handleMouseDownRotate(n),this.state=ht.ROTATE}break;case cs.PAN:if(n.ctrlKey||n.metaKey||n.shiftKey){if(this.enableRotate===!1)return;this._handleMouseDownRotate(n),this.state=ht.ROTATE}else{if(this.enablePan===!1)return;this._handleMouseDownPan(n),this.state=ht.PAN}break;default:this.state=ht.NONE}this.state!==ht.NONE&&this.dispatchEvent(Cl)}function E0(n){switch(this.state){case ht.ROTATE:if(this.enableRotate===!1)return;this._handleMouseMoveRotate(n);break;case ht.DOLLY:if(this.enableZoom===!1)return;this._handleMouseMoveDolly(n);break;case ht.PAN:if(this.enablePan===!1)return;this._handleMouseMovePan(n);break}}function b0(n){this.enabled===!1||this.enableZoom===!1||this.state!==ht.NONE||(n.preventDefault(),this.dispatchEvent(Cl),this._handleMouseWheel(this._customWheelEvent(n)),this.dispatchEvent(iu))}function T0(n){this.enabled!==!1&&this._handleKeyDown(n)}function A0(n){switch(this._trackPointer(n),this._pointers.length){case 1:switch(this.touches.ONE){case ss.ROTATE:if(this.enableRotate===!1)return;this._handleTouchStartRotate(n),this.state=ht.TOUCH_ROTATE;break;case ss.PAN:if(this.enablePan===!1)return;this._handleTouchStartPan(n),this.state=ht.TOUCH_PAN;break;default:this.state=ht.NONE}break;case 2:switch(this.touches.TWO){case ss.DOLLY_PAN:if(this.enableZoom===!1&&this.enablePan===!1)return;this._handleTouchStartDollyPan(n),this.state=ht.TOUCH_DOLLY_PAN;break;case ss.DOLLY_ROTATE:if(this.enableZoom===!1&&this.enableRotate===!1)return;this._handleTouchStartDollyRotate(n),this.state=ht.TOUCH_DOLLY_ROTATE;break;default:this.state=ht.NONE}break;default:this.state=ht.NONE}this.state!==ht.NONE&&this.dispatchEvent(Cl)}function w0(n){switch(this._trackPointer(n),this.state){case ht.TOUCH_ROTATE:if(this.enableRotate===!1)return;this._handleTouchMoveRotate(n),this.update();break;case ht.TOUCH_PAN:if(this.enablePan===!1)return;this._handleTouchMovePan(n),this.update();break;case ht.TOUCH_DOLLY_PAN:if(this.enableZoom===!1&&this.enablePan===!1)return;this._handleTouchMoveDollyPan(n),this.update();break;case ht.TOUCH_DOLLY_ROTATE:if(this.enableZoom===!1&&this.enableRotate===!1)return;this._handleTouchMoveDollyRotate(n),this.update();break;default:this.state=ht.NONE}}function R0(n){this.enabled!==!1&&n.preventDefault()}function C0(n){n.key==="Control"&&(this._controlActive=!0,this.domElement.getRootNode().addEventListener("keyup",this._interceptControlUp,{passive:!0,capture:!0}))}function P0(n){n.key==="Control"&&(this._controlActive=!1,this.domElement.getRootNode().removeEventListener("keyup",this._interceptControlUp,{passive:!0,capture:!0}))}const Dr=new Map;async function su(n){for(const e of new Set(n))!e||Dr.has(e)||Dr.set(e,await pl.record(e));return Object.fromEntries(n.filter(e=>Dr.get(e)).map(e=>[e,Dr.get(e)]))}const Ir={core:"#caa24a",hot:"#c8402e",warm:"#7e96cf",cold:"#6f7da0",frozen:"#8a8578"},rh={derived_from:["derived from","grounds this"],refers_to:["refers to","referenced here"],supersedes:["supersedes","superseded by"],verifies:["verifies","verified in"],causal:["led to","follows from"],part_of:["part of","contains"],temporal:["before","after"],contradicts:["contradicts","contradicts"],blocks:["blocks","blocked by"],targets:["targets","targeted by"]},L0=470,D0=300,ao=300,I0=120,qt=n=>document.getElementById(n),mt=n=>(n||"").replace(/[&<>]/g,e=>({"&":"&amp;","<":"&lt;",">":"&gt;"})[e]),Vt=n=>(n||"").replace(/-20\d\d-\d\d-\d\d$/,"").replace(/-[0-9a-f]{6}$/,"").replace(/-/g," "),U0=n=>{const e=new Map;n.forEach(i=>{if(!i)return;const s=Vt(i.name);e.set(s,(e.get(s)||0)+1)});const t={};return n.forEach(i=>{if(!i)return;const s=Vt(i.name),r=e.get(s)>1?(i.name.match(/-([0-9a-f]{6})$/)||[])[1]:null;t[i.id]=r?`${s} · ${r}`:s}),t},ah=(n,e,t)=>Math.min(t,Math.max(e,n));function N0(n,{fmtDateTime:e}){const t=qt("rworld"),i=qt("rtrack"),s=qt("rcanvas"),r=qt("manuscript"),a={},o={};n.nodes.filter(G=>G.type!=="chain").forEach(G=>{a[G.id]={...G},o[G.id]=[]});for(const G of n.edges)!a[G.src]||!a[G.dst]||(o[G.src].push({o:G.dst,t:G.type,dir:"out"}),o[G.dst].push({o:G.src,t:G.type,dir:"in"}));let c=null,l=[],u={},g=new Map,h=new Set,m=[],_=60,S=40,p=.62,d="trail";const y=G=>mt(G).replace(/\[\[([^\]]+)\]\]/g,(j,E)=>`<span class="rmark">${Vt(E)}</span>`).replace(/`([^`]+)`/g,"<code>$1</code>"),A=G=>(G||"").trim().split(/\n{2,}/).map(j=>`<p>${y(j.trim()).replace(/\n/g,"<br>")}</p>`).join(""),T=(G,j)=>{const E=(G||"").replace(/\s+/g," ").trim();return E.length>j?y(E.slice(0,j).replace(/\s\S*$/,""))+"…":y(E)},L=G=>G.created_secs?e(G.created_secs):"—",b=G=>{const j=document.createElement("div");return j.innerHTML=G.trim(),j.firstChild};async function I(G){let j;try{j=await su(G)}catch(ce){return window.dispatchEvent(new CustomEvent("kaeru-api-error",{detail:ce})),!1}let E=!1;for(const ce in j)a[ce]&&a[ce].body!==j[ce]&&(a[ce].body=j[ce],E=!0);return E}async function M(G){const j=n.chains.find(E=>E.id===G);return j?I(j.members||[]):!1}const w=G=>{const j=n.chains.find(E=>(E.members||[]).includes(G));return j?j.members:[G]},B=()=>{const G=[];for(const j in o)for(const E of o[j])E.dir==="out"&&E.t==="derived_from"&&G.push({from:E.o,to:j});return G};function C(){u={};const G=B(),j={};h=new Set(l.map(V=>V.id));for(let V=0;V<2;V++)for(const te of G)h.has(te.from)&&h.add(te.to),h.has(te.to)&&h.add(te.from);for(const V of G)h.has(V.from)&&h.has(V.to)&&(j[V.to]=j[V.to]||[]).push(V.from);for(let V=1;V<l.length;V++){const te=l[V].id;(j[te]||[]).length||(j[te]=[l[V-1].id])}const E={},ce=(V,te)=>{if(E[V]!=null)return E[V];if(te.has(V))return 0;te.add(V);const Y=(j[V]||[]).filter(Q=>h.has(Q));return E[V]=Y.length?Math.max(...Y.map(Q=>ce(Q,te)))+1:0,E[V]};[...h].forEach(V=>ce(V,new Set));const le={};[...h].forEach(V=>{(le[E[V]]=le[E[V]]||[]).push(V)});const me={},K=new Set;let Re=0;Object.keys(le).map(Number).sort((V,te)=>V-te).forEach(V=>{le[V].sort((te,Y)=>(a[te]?.created_secs||0)-(a[Y]?.created_secs||0)),le[V].forEach(te=>{const Y=(j[te]||[]).find(Q=>me[Q]!=null&&!K.has(Q));Y!=null?(me[te]=me[Y],K.add(Y)):me[te]=Re++})});const v=new Set(l.map(V=>V.id)),f=new Set([...v].map(V=>me[V]).filter(V=>V!=null)),D=[...new Set(Object.values(me))].sort((V,te)=>(f.has(te)?1:0)-(f.has(V)?1:0)||V-te),q={};D.forEach((V,te)=>{q[V]=te}),[...h].forEach(V=>{u[V]={x:I0+E[V]*L0,y:ao-70+q[me[V]]*D0,d:E[V],lane:q[me[V]]}});const ne=new Map;[...h].forEach(V=>{for(const te of o[V]||[]){if(h.has(te.o))continue;ne.has(te.o)||ne.set(te.o,{n:0,steps:[]});const Y=ne.get(te.o);Y.n++,Y.steps.push(V)}}),g=new Map([...ne].filter(([,V])=>V.n>1))}let z={};function Z(){if(t.querySelectorAll(".step,.sat,.lanetag,.bedlabel").forEach(j=>j.remove()),!l.length)return;C(),z=U0([...h].map(j=>a[j]).concat([...g.keys()].map(j=>a[j])));const G=new Set(l.map(j=>j.id));[...h].sort((j,E)=>u[j].d-u[E].d).forEach(j=>{const E=a[j];if(!E||!u[j])return;const{x:ce,y:le}=u[j],me=Ir[E.layer]||"var(--dim)",K=G.has(j),Re=l.findIndex(f=>f.id===j),v=b(`<button type="button" class="step${K?"":" aside"}" data-id="${j}"
          style="left:${ce}px;top:${le}px;--acc:${me}"
          aria-label="Read ${mt(z[j]||Vt(E.name))}">
        <span class="meta"><span class="num">${K?String(Re+1).padStart(2,"0"):"—"}</span>
          <span class="s">·</span><span class="t">${mt(E.type)}</span>
          <span class="s">·</span><span>${mt(E.layer)}</span></span>
        <span class="h2-like"></span>
        <h2>${mt(z[j]||Vt(E.name))}</h2>
        <p class="gist">${T(E.body,150)}</p>
        <span class="when">${L(E)}</span>
      </button>`);t.appendChild(v),v.onclick=()=>Ee(j)}),J(),requestAnimationFrame(R)}function J(){t.querySelectorAll(".sat,.lanetag,.bedlabel").forEach(le=>le.remove());const G={};Object.entries(u).forEach(([le,me])=>{const K=t.querySelector(`.step[data-id="${le}"]`);K&&(G[me.lane]=G[me.lane]||[]).push({id:le,p:me,el:K})});const j=64,E=150;let ce=ao-70;Object.keys(G).map(Number).sort((le,me)=>le-me).forEach(le=>{const me=G[le];me.some(({id:K})=>O(K).length)&&(ce+=E),me.forEach(({p:K,el:Re})=>{K.y=ce,Re.style.top=`${ce}px`}),ce+=Math.max(...me.map(({el:K})=>K.offsetHeight))+j}),Object.entries(u).forEach(([le,me])=>{const K=O(le);K.length&&W(K.slice(0,1),me.x,me.y-130,le)}),se(new Set(l.map(le=>le.id))),oe(),he()}const O=G=>(o[G]||[]).filter(j=>!h.has(j.o)&&!g.has(j.o)&&j.dir==="in"&&j.t==="refers_to");function W(G,j,E,ce){G.forEach((le,me)=>{const K=a[le.o];if(!K)return;const Re=Ir[K.layer]||"var(--dim)",v=(rh[le.t]||[le.t,le.t])[le.dir==="out"?0:1],f=b(`<button type="button" class="sat" data-id="${K.id}" data-of="${ce}"
          style="left:${j+8+me*310}px;top:${E}px;--acc:${Re}"
          aria-label="Read ${mt(z[K.id]||Vt(K.name))} — ${mt(v)}">
        <span class="rel">${mt(v)}</span><span class="h3">${mt(z[K.id]||Vt(K.name))}</span>
        <span class="ex">${T(K.body,110)}</span>
        <span class="tag">${mt(K.type)} · ${mt(K.tier)}</span></button>`);f.onclick=()=>tt(K.id),t.appendChild(f)})}const $=G=>{let j="";for(let E=G;;E=Math.floor(E/26)-1)if(j=String.fromCharCode(65+E%26)+j,E<26)return j};function se(G){const j={};Object.entries(u).forEach(([ce,le])=>{(!j[le.lane]||le.d<u[j[le.lane]].d)&&(j[le.lane]=ce)});const E=l.length&&u[l[0].id]?u[l[0].id].lane:0;Object.entries(j).forEach(([ce,le])=>{const me=u[le],K=+ce===E;t.appendChild(b(`<div class="lanetag${K?" main":""}" style="left:${me.x}px;top:${me.y-30}px">
        <span class="ln">${$(+ce)}</span> · ${K?"main trail":"side line"}</div>`))})}function oe(){if(!g.size)return;const G=Object.values(u).map(K=>K.x);let j=ao;Object.entries(u).forEach(([K,Re])=>{const v=t.querySelector(`.step[data-id="${K}"]`);j=Math.max(j,Re.y+(v?v.offsetHeight:300))});const E=j+90,ce=g.size,le=(Math.min(...G)+Math.max(...G)+540)/2,me=le-ce*310/2;t.appendChild(b(`<div class="bedlabel" style="left:${me}px;top:${E-30}px">
      shared ground — more than one line rests on this</div>`)),[...g].forEach(([K,Re],v)=>{const f=a[K];if(!f)return;const D=Ir[f.layer]||"var(--dim)",q=b(`<button type="button" class="sat bed" data-id="${K}" data-steps="${Re.steps.join(",")}"
          style="left:${me+v*310}px;top:${E}px;--acc:${D}"
          aria-label="Read ${mt(z[K]||Vt(f.name))} — shared ground for ${Re.n} steps">
        <span class="rel">shared ground · ${Re.n}×</span><span class="h3">${mt(z[K]||Vt(f.name))}</span>
        <span class="ex">${T(f.body,130)}</span>
        <span class="tag">${mt(f.type)} · ${mt(f.tier)}</span></button>`);q.onclick=()=>tt(K),t.appendChild(q)})}function he(){if(!l.length){i.innerHTML="";return}const G=getComputedStyle(document.documentElement),j=G.getPropertyValue("--seal").trim(),E=G.getPropertyValue("--line-2").trim(),ce=G.getPropertyValue("--line").trim(),le=Re=>{const v=t.querySelector(`.step[data-id="${Re}"]`);if(!v)return null;const f=parseFloat(v.style.left),D=parseFloat(v.style.top);return{x:f,y:D,w:v.offsetWidth,h:v.offsetHeight,cy:D+34}};let me="";const K={};Object.entries(u).forEach(([Re,v])=>{const f=t.querySelector(`.step[data-id="${Re}"]`);if(!f)return;const D=K[v.lane]=K[v.lane]||{y:v.y+34,min:1/0,max:-1/0};D.min=Math.min(D.min,v.x),D.max=Math.max(D.max,v.x+f.offsetWidth)}),Object.values(K).forEach(Re=>{isFinite(Re.min)&&(me+=`<line x1="${Re.min-60}" y1="${Re.y}" x2="${Re.max+60}" y2="${Re.y}" stroke="${E}" stroke-width="1" opacity=".5"/>`)}),B().forEach(Re=>{const v=le(Re.from),f=le(Re.to);if(!v||!f)return;const D=v.x+v.w,q=v.cy,ne=f.x-10,V=f.cy,te=Math.max(60,(ne-D)*.45);me+=`<path d="M ${D} ${q} C ${D+te} ${q}, ${ne-te} ${V}, ${ne} ${V}" fill="none" stroke="${j}" stroke-width="1.5" opacity=".6"/>`,me+=`<path d="M ${ne-6} ${V-4} l 6 4 l -6 4" fill="none" stroke="${j}" stroke-width="1.5" opacity=".6"/>`,me+=`<circle cx="${D}" cy="${q}" r="3.5" fill="${j}" opacity=".75"/>`}),t.querySelectorAll(".sat.bed").forEach(Re=>{const v=parseFloat(Re.style.left)+145,f=parseFloat(Re.style.top);(Re.dataset.steps||"").split(",").filter(Boolean).forEach(D=>{const q=le(D);if(!q)return;const ne=q.x+q.w/2,V=q.y+q.h;me+=`<path d="M ${ne} ${V} C ${ne} ${V+80}, ${v} ${f-80}, ${v} ${f}" fill="none" stroke="${ce}" stroke-width="1" stroke-dasharray="3 5"/>`})}),t.querySelectorAll(".sat:not(.bed)").forEach(Re=>{const v=le(Re.dataset.of);if(!v)return;const f=parseFloat(Re.style.left)+40,D=parseFloat(Re.style.top)+Re.offsetHeight;me+=`<path d="M ${f} ${D} L ${f} ${v.y}" stroke="${ce}" stroke-width="1" stroke-dasharray="3 4"/>`}),i.innerHTML=me}function Ee(G){Se("manuscript");const j=()=>{const E=r.querySelector(`section[data-id="${G}"]`);if(!E)return;r.querySelectorAll("section.here").forEach(le=>le.classList.remove("here")),E.classList.add("here");const ce=E.getBoundingClientRect().top-r.getBoundingClientRect().top;r.scrollTo({top:r.scrollTop+ce-96,behavior:Pe()?"auto":"smooth"})};j(),setTimeout(j,60)}const Pe=()=>matchMedia("(prefers-reduced-motion: reduce)").matches;function Ge(){if(!l.length){r.innerHTML="";return}let G=`<div class="ms"><div class="kind">${c.adhoc?"node":"reasoning chain"}</div>
      <h1>${mt(Vt(c.name))}</h1>
      <p class="lede">${mt(c.body||(c.adhoc?"A single node, read in full.":"A worked trail of reasoning, read end to end."))}</p>
      <div class="byline">${l.length} ${l.length===1?"step":"steps"} · ${L(l[0])}${l.length>1?" — "+L(l[l.length-1]):""}</div>`;const j=new Set(l.map(ce=>ce.id));[...l,...[...h].filter(ce=>!j.has(ce)).map(ce=>a[ce]).filter(Boolean).sort((ce,le)=>(ce.created_secs||0)-(le.created_secs||0))].forEach((ce,le)=>{const me=Ir[ce.layer]||"var(--dim)",K=j.has(ce.id);G+=`<section data-id="${ce.id}" style="--acc:${me}">
        <div class="snum">${K?`STEP ${String(le+1).padStart(2,"0")}`:"SIDE LINE"} · ${mt(ce.type)} · ${mt(ce.tier)}</div>
        <h2>${mt(z[ce.id]||Vt(ce.name))}</h2>
        <div class="when">asserted <b>${L(ce)}</b></div>
        <div class="prose">${A(ce.body||"—")}</div></section>`;const Re=(o[ce.id]||[]).filter(v=>!j.has(v.o)).slice(0,3);Re.length&&(G+=`<aside class="marg" style="--acc:${me}"><div class="mtitle">in the margin</div>`,Re.forEach(v=>{const f=a[v.o];if(!f)return;const D=(rh[v.t]||[v.t,v.t])[v.dir==="out"?0:1];G+=`<button type="button" class="mlink" data-to="${f.id}"
            aria-label="Read ${mt(Vt(f.name))} — ${mt(D)}">
            <span class="r">${v.dir==="out"?"↳ ":"↰ "}${mt(D)}</span>
            <span class="n">${mt(Vt(f.name))}</span></button>`}),G+="</aside>")}),r.innerHTML=G+"</div>",r.querySelectorAll(".mlink").forEach(ce=>{ce.onclick=()=>{tt(ce.dataset.to),Se("manuscript")}})}function Je(){t.style.transform=`translate(${_}px,${S}px) scale(${p})`;const G=28*p;s.style.backgroundSize=`${G}px ${G}px`,s.style.backgroundPosition=`${_}px ${S}px`;const j=qt("rzoom");j&&(j.textContent=Math.round(p*100)+"%")}function R(){U()}function U(){const G=[...t.children].filter(f=>f.tagName!=="svg"&&f.offsetWidth);if(!G.length)return;let j=1e9,E=1e9,ce=-1e9,le=-1e9;G.forEach(f=>{const D=parseFloat(f.style.left),q=parseFloat(f.style.top);j=Math.min(j,D),E=Math.min(E,q),ce=Math.max(ce,D+f.offsetWidth),le=Math.max(le,q+f.offsetHeight)});const me=90,K=76,v=innerHeight-K-60;p=ah(Math.min(innerWidth/(ce-j+me*2),v/(le-E+me*2)),.34,1),_=innerWidth/2-(j+(ce-j)/2)*p,S=K+v/2-(E+(le-E)/2)*p,Je()}let ie=!1,re=0,Me=0;s.addEventListener("pointerdown",G=>{G.target.closest("#rdock")||(ie=!0,re=G.clientX-_,Me=G.clientY-S,s.classList.add("grabbing"))}),addEventListener("pointermove",G=>{ie&&(_=G.clientX-re,S=G.clientY-Me,Je())}),addEventListener("pointerup",()=>{ie=!1,s.classList.remove("grabbing")}),s.addEventListener("wheel",G=>{G.preventDefault();const j=ah(p*Math.exp(-G.deltaY*.0012),.2,1.8),E=s.getBoundingClientRect(),ce=G.clientX-E.left,le=G.clientY-E.top;_=ce-(ce-_)*(j/p),S=le-(le-S)*(j/p),p=j,Je()},{passive:!1});function Se(G){const j=qt("rname");j&&c&&(j.textContent=Vt(c.name));const E=qt("rhint");E&&(E.textContent=G==="manuscript"?"reading the trail end to end":"click a step to read it"),d=G,r.classList.toggle("on",G==="manuscript"),s.style.display=G==="manuscript"?"none":"",qt("rTrail").classList.toggle("on",G==="trail"),qt("rMs").classList.toggle("on",G==="manuscript")}const Ce=qt("rPrev");Ce&&(Ce.onclick=rt),addEventListener("keydown",G=>{qt("reader").hidden||/^(INPUT|TEXTAREA)$/.test(G.target.tagName)||(G.key==="Backspace"||G.altKey&&G.key==="ArrowLeft")&&(G.preventDefault(),rt())}),qt("rTrail").onclick=()=>Se("trail"),qt("rMs").onclick=()=>Se("manuscript");const Ke=()=>c?c.adhoc?{node:l[0]&&l[0].id}:{chain:c.id}:null;function Fe(G){if(!G)return!1;if(G.chain){const j=n.chains.find(E=>E.id===G.chain);return j?ft(j,j.members):!1}return G.node&&a[G.node]?ft({name:a[G.node].name,adhoc:!0},[G.node]):!1}function tt(G,j=!0){if(!a[G])return!1;const E=Ke(),ce=n.chains.find(K=>(K.members||[]).includes(G)),le=ce?ce.members:[G],me=ce?ft(ce,le):ft({name:a[G].name,adhoc:!0},[G]);return me&&I(le).then(K=>{K&&c&&ft(c,le)}),me&&j&&E&&JSON.stringify(E)!==JSON.stringify(Ke())&&m.push(E),Ve(),me}function rt(){const G=m.pop();if(!G)return!1;const j=Fe(G);return Ve(),j}function Ve(){const G=qt("rPrev");if(!G)return;G.hidden=!m.length;const j=m[m.length-1],E=j?j.chain?(n.chains.find(ce=>ce.id===j.chain)||{}).name:(a[j.node]||{}).name:"";G.title=E?`Back to ${Vt(E)}`:""}function ft(G,j){return c=G,l=(j||[]).map(E=>a[E]).filter(Boolean).sort((E,ce)=>(E.created_secs||0)-(ce.created_secs||0)),l.length?(Z(),Ge(),Se(d),document.fonts&&document.fonts.status!=="loaded"&&document.fonts.ready.then(()=>{c&&l.length&&J()}),!0):!1}return{async show(G){const j=n.chains.find(E=>E.id===G)||n.chains.find(E=>E.name===G);return j&&!await M(j.id)&&await I(j.members),m=[],Ve(),j?ft(j,j.members):!1},async showNode(G){return await I(w(G)),m=[],Ve(),tt(G,!1)},back:rt,canBack:()=>m.length>0,chainId:()=>c&&c.id,redraw:he,hasChain:()=>!!c}}const Ur=[{key:"open",label:"Open",terminal:!1},{key:"in-progress",label:"In Progress",terminal:!1},{key:"done",label:"Done",terminal:!0}],F0=86400,$r=new Map;async function oh(n){if($r.has(n))return $r.get(n);let e=null;try{const i=(await pl.call("context",{initiative:n,view:"board",budget_chars:64e3})).data.sections.find(s=>s.name==="board");i?.details.columns?.length&&(e=i.details.columns)}catch{}return $r.set(n,e),e}const xt=n=>document.getElementById(n),yt=n=>(n||"").replace(/[&<>"']/g,e=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"})[e]),oo=n=>(n||"").replace(/-20\d\d-\d\d-\d\d$/,"").replace(/-[0-9a-f]{6}$/,"").replace(/-/g," "),lh=(n,e)=>(n.tags||[]).find(t=>t.startsWith(e)),ch=n=>n===1?"1 day":`${n} days`;function O0(n,{onOpenNode:e}){const t={},i={};n.nodes.forEach(R=>{t[R.id]=R});for(const R of n.edges)(i[R.src]=i[R.src]||[]).push(R.dst),(i[R.dst]=i[R.dst]||[]).push(R.src);const s=new Date().toISOString().slice(0,10),r=Date.now()/1e3,a=n.nodes.filter(R=>R.type==="task").map(R=>{const U=R.created_secs?Math.floor((r-R.created_secs)/F0):null,ie=(lh(R,"due:")||"").slice(4)||null,re=!!ie&&ie<s,Me=(i[R.id]||[]).filter(Fe=>t[Fe]),Se=Me.find(Fe=>t[Fe].type==="episode")||Me[0]||null,Ce=(lh(R,"status:")||"").slice(7),Ke=(R.body||"").trim()||oo(R.name);return{id:R.id,name:R.name,raw:Ce,age:U,due:ie,overdue:re,src:Se,island:Me.length===0,init:(R.initiatives||[])[0]||null,text:Ke,hay:(Ke+" "+oo(R.name)).toLowerCase(),rot:(U||0)+(Me.length===0?45:0)+(re?400:0)}});let o=Ur;const c=R=>o.some(U=>U.key===R.raw)?R.raw:o[0].key,l=R=>o.some(U=>U.key===R&&U.terminal),u=R=>$r.get(R.init)||Ur,g=R=>u(R).some(U=>U.key===R.raw&&U.terminal),h=R=>u(R).find(U=>U.terminal)?.key||"done";async function m(R){if(R!==p)return await oh(R)||Ur;const U=await Promise.all(S.map(([Se])=>oh(Se))),ie=new Map;for(const Se of[Ur,...U.filter(Boolean)])for(const Ce of Se)ie.has(Ce.key)||ie.set(Ce.key,Ce);const re=[...ie.values()],Me=re.findIndex(Se=>Se.key==="done");return Me>=0&&re.push(re.splice(Me,1)[0]),re}function _(){const R=d;m(R).then(U=>{d===R&&(o=U,se(),he())})}const S=[...a.reduce((R,U)=>R.set(U.init,(R.get(U.init)||0)+(U.raw==="done"?0:1)),new Map)].filter(([R])=>R).sort((R,U)=>U[1]-R[1]),p="*";let d=p,y=null,A=null,T="";const L=new Set,b=[{key:"overdue",label:"past due",f:R=>R.overdue},{key:"island",label:"no provenance",f:R=>R.island},{key:"old",label:"over a month",f:R=>R.age>30},{key:"owed",label:"not done",f:R=>!g(R)}],I=()=>d===p?a:a.filter(R=>R.init===d),M=R=>(!T||R.hay.includes(T))&&[...L].every(U=>b.find(ie=>ie.key===U).f(R)),w=()=>!!(T||L.size);function B(){xt("bsieves").innerHTML=b.map(R=>`<button type="button" class="sieve" data-sieve="${R.key}" aria-pressed="false">
         ${R.label} <span class="c"></span></button>`).join("")}function C(R){b.forEach(U=>{const ie=xt("bsieves").querySelector(`[data-sieve="${U.key}"]`);if(!ie)return;const re=L.has(U.key);ie.setAttribute("aria-pressed",String(re)),ie.classList.toggle("on",re),ie.querySelector(".c").textContent=R.filter(U.f).length}),xt("bClear").hidden=!w()}function z(){const R=[];return T&&R.push(`“${T}”`),L.forEach(U=>R.push(b.find(ie=>ie.key===U).label)),R.join(" + ")}function Z(){T="",L.clear(),xt("bFind").value="",se(),he()}const J=R=>{const U=R.replace(/\s+/g," ").trim(),ie=U.search(/[.!?](\s|$)/);return ie>24&&ie<150?U.slice(0,ie+1):U};function O(R){const U=[R.overdue?`<span class="due">due ${yt(R.due)}</span>`:R.due?`<span class="soft">due ${yt(R.due)}</span>`:"",R.island?'<span class="isle">no provenance</span>':""].filter(Boolean).join('<span class="sep">·</span>');return`<button type="button" class="card${R.overdue?" overdue":""}${y===R.id?" picked":""}"
        data-card="${yt(R.id)}" aria-pressed="${y===R.id}">
      <span class="what">${yt(J(R.text))}</span>
      <span class="meta">${d===p&&R.init?`<span class="owner">${yt(R.init)}</span>`:""}<span class="age">${R.age==null?"—":ch(R.age)}</span>${U?`<span class="sep">·</span>${U}`:""}</span>
    </button>`}const W=new Set(a.map(R=>R.raw));function $(R){return W.has(R.key)||R.key===o[0].key?`<p class="none">${R.label} is empty.</p>`:`<p class="none">Nothing has ever been in ${R.label}. The board reads;
      the vault writes — a card gets here from the chat:</p>
      <code class="cmd">task_update(task=&lt;uuid&gt;, status=${yt(R.key)})</code>`}function se(){const R=I(),U=R.filter(M);xt("bcols").innerHTML=o.map(Se=>{const Ce=U.filter(Ke=>c(Ke)===Se.key).sort((Ke,Fe)=>l(Se.key)?(Fe.age==null)-(Ke.age==null)||Ke.age-Fe.age:Fe.rot-Ke.rot);return`<section class="col" aria-labelledby="col-${Se.key}">
        <header class="colhead"><h2 id="col-${Se.key}">${Se.label}</h2><span class="n">${Ce.length}</span></header>
        <div class="stack">${Ce.length?Ce.map(O).join(""):$(Se)}</div>
      </section>`}).join(""),!U.length&&w()&&(xt("bcols").innerHTML=`<p class="nohits">No cards match ${yt(z())}.
        <button type="button" class="btn" id="bClear2">Clear filters</button></p>`);const ie=U.filter(Se=>!g(Se)).length,re=U.filter(Se=>Se.overdue&&!g(Se)).length,Me=w()?`${U.length} of ${R.length} cards · `:"";xt("bcount").textContent=U.length?`${Me}${ie} owed${re?`, ${re} past due`:""} · sorted by rot — deadline, then age, then isolation`:"",C(R),xt("bname").textContent=d===p?"all initiatives":d,xt("bsay").textContent=`${d===p?"all initiatives":d}: ${ie} open, ${U.length} cards total`}let oe=null;function he(){const R=a.find(Me=>Me.id===y),U=xt("bdetail");if(U.hidden=!R,!R)return;R.whole||(R.whole=!0,su([R.id]).then(Me=>{Me[R.id]&&Me[R.id]!==R.text&&(R.text=Me[R.id],y===R.id&&he())}));const ie=o.find(Me=>Me.key===c(R)),re=R.island?'<span class="isle">nothing links to this card yet</span>':`<button type="button" class="nodelink" data-open="${yt(R.src)}">${yt(oo(t[R.src]?t[R.src].name:""))}</button>`;U.tabIndex=-1,U.innerHTML=`
      <header id="bdhead">
        <span class="k">card</span><span class="colname">${ie?ie.label:c(R)}</span>
        <button type="button" class="btn x" data-close="1" aria-label="Close card">✕</button>
      </header>
      <div id="bdbody">
        <p class="dbody">${yt(R.text)}</p>
        <dl>
          <dt>initiative</dt><dd>${yt(R.init||"—")}</dd>
          <dt>open for</dt><dd>${R.age==null?"—":ch(R.age)}</dd>
          ${R.due?`<dt>due</dt><dd>${R.overdue?`<span class="due">${yt(R.due)} · past due</span>`:`<span class="soft">${yt(R.due)}</span>`}</dd>`:""}
          <dt>came out of</dt><dd>${re}</dd>
        </dl>
      </div>
      <div id="bdacts">
        <button type="button" class="btn" data-open="${yt(R.id)}">Open in reader</button>
        <button type="button" class="btn cp" data-copy="task_update(initiative=${yt(R.init)}, task=${yt(R.id)}, status=in-progress)">Copy “task_update”</button>
        <button type="button" class="btn cp" data-copy="task_update(initiative=${yt(R.init)}, task=${yt(R.id)}, status=${yt(h(R))})">Copy completion</button>
      </div>`}function Ee(){y=null,se(),he(),oe&&document.contains(oe)&&oe.focus(),oe=null}function Pe(R,U){if(y===R){Ee();return}y=R,oe=U||null,se(),he(),xt("bdetail").focus()}xt("bcols").addEventListener("click",R=>{const U=R.target.closest("[data-card]");U&&Pe(U.dataset.card,U)}),xt("bdetail").addEventListener("click",R=>{const U=R.target.closest("button");if(U){if(U.dataset.close){Ee();return}if(U.dataset.open){e(U.dataset.open);return}U.dataset.copy&&(navigator.clipboard?.writeText(U.dataset.copy),clearTimeout(A),xt("bdetail").querySelectorAll(".cp").forEach(ie=>ie.classList.remove("ok")),U.classList.add("ok"),xt("bsay").textContent=`Copied: ${U.dataset.copy}`,A=setTimeout(()=>U.classList.remove("ok"),1400))}}),addEventListener("keydown",R=>{R.key!=="Escape"||xt("board").hidden||!y||(R.stopImmediatePropagation(),R.preventDefault(),Ee())}),xt("bsieves").addEventListener("click",R=>{const U=R.target.closest("[data-sieve]");if(!U)return;const ie=U.dataset.sieve;L.has(ie)?L.delete(ie):L.add(ie),se(),he()}),xt("bFind").addEventListener("input",R=>{T=R.target.value.trim().toLowerCase(),se(),he()}),xt("bFind").addEventListener("keydown",R=>{R.key!=="Escape"||!R.target.value||(R.stopPropagation(),R.target.value="",T="",se(),he())}),xt("bClear").addEventListener("click",Z),xt("bcols").addEventListener("click",R=>{R.target.id==="bClear2"&&Z()});const Ge=xt("bInit");B();const Je=a.filter(R=>!g(R)).length;return Ge.innerHTML=`<option value="${p}">all initiatives (${Je} open)</option>`+S.map(([R,U])=>`<option value="${yt(R)}">${yt(R)} (${U} open)</option>`).join(""),Ge.addEventListener("change",()=>{d=Ge.value,y=null,se(),he(),_()}),{show(R){return(R===p||R&&S.some(([U])=>U===R))&&(d=R),d?(Ge.value=d,Ge._sync&&Ge._sync(),y=null,se(),he(),_(),!0):!1},initiative:()=>d}}const jt=n=>String(n??"").replace(/[&<>]/g,e=>({"&":"&amp;","<":"&lt;",">":"&gt;"})[e]),ua=n=>new Date(n*1e3).toISOString().slice(0,10),ru=n=>{const e=new Date(n*1e3);return e.toISOString().slice(0,10).replace(/-/g,"·")+" "+e.toISOString().slice(11,16)},we=n=>document.getElementById(n);function B0(n){let e=2166136261;for(let t=0;t<n.length;t++)e^=n.charCodeAt(t),e=Math.imul(e,16777619);return e>>>0}function au(n){let e=n>>>0;return()=>(e=e*1664525+1013904223>>>0,e/4294967296)}const z0={core:"#caa24a",hot:"#c8402e",warm:"#7e96cf",cold:"#6f7da0",frozen:"#566"},k0={derived_from:[.45,.78,.85],refers_to:[.55,.6,.82],causal:[.88,.66,.36],supersedes:[.85,.45,.45],verifies:[.42,.78,.6],falsifies:[.9,.45,.72],contradicts:[.9,.35,.35],part_of:[.66,.82,.45],temporal:[.6,.64,.78],consolidated_to:[.95,.78,.4],blocks:[.95,.58,.32],targets:[.4,.78,.95]},H0=[.4,.45,.55],ui={hub:{spread:0,size:24,mix:.12,a:1},core:{spread:75,size:22,mix:.12,a:1},hot:{spread:150,size:14,mix:.06,a:1},warm:{spread:240,size:10,mix:.02,a:.96},cold:{spread:300,size:7,mix:0,a:.72},frozen:{spread:345,size:5,mix:0,a:.52}},Pl={dark:{bg:2763311,fog:18e-5,sat:.95,light:.6,mix:new ke(16774111),edgeMul:1,edgeGround:[0,0,0],edgeOpacity:.1,spoke:[.15,.16,.21],hotEdge:[.95,.88,.66],chainEdge:[1,.83,.4],bridge:13601582,bridgeRest:.6,bridgeDim:.14,particle:15116348,star:10131084,starOp:.4,ring:15526111,dimTo:new ke(0),dimAmt:.8,hotLerp:new ke(16777215),chainCur:16777215,chainVisited:16767082,chainLerp:.2},light:{bg:16579576,fog:9e-5,sat:.85,light:.43,mix:new ke(2761499),edgeMul:.34,edgeGround:[.988,.984,.973],edgeOpacity:.4,spoke:[.84,.8,.72],hotEdge:[.42,.3,.12],chainEdge:[.55,.37,.06],bridge:9071396,bridgeRest:.7,bridgeDim:.18,particle:9071396,star:12103836,starOp:0,ring:3485994,dimTo:new ke(16579576),dimAmt:.72,hotLerp:new ke(1708554),chainCur:1513243,chainVisited:9067024,chainLerp:.28}};let ds=null;const ou=matchMedia("(prefers-reduced-motion: reduce)");let Gs=(()=>{try{return localStorage.getItem("kaeru-viz-theme")||"auto"}catch{return"auto"}})();const lu=matchMedia("(prefers-color-scheme: light)"),Ll=()=>Gs==="auto"?lu.matches?"light":"dark":Gs;let ze=Pl[Ll()];async function G0(){return Ju()||await pl.graph()}function V0(n){const e=document.getElementById("graph");e&&e.insertAdjacentHTML("beforeend",`<div style="position:absolute;inset:0;display:grid;place-items:center;color:#8b887e;font-family:'IBM Plex Mono',monospace;font-size:13px;letter-spacing:.04em;text-align:center;padding:24px;">${n}</div>`);const t=document.getElementById("stats");throw t&&(t.textContent="unavailable"),new Error("kaeru-viz: "+n)}const et=await G0();et&&Zu(et);(!et||!Array.isArray(et.nodes)||et.nodes.length===0)&&V0("no records available under this instance’s access settings");et.initiatives??=[];et.edges??=[];et.chains??=[];et.project_links??=[];et.meta??={};const Dl=et.nodes.filter(n=>n.type!=="chain"),W0=n=>n.isHub?n.hubInit:n.initiatives&&n.initiatives[0]||"∅",Is=et.initiatives.map(n=>n.name),da={},cu={},Yr=1050;Is.forEach((n,e)=>{da[n]=e*137.508%360/360;const t=Is.length>1?1-e/(Is.length-1)*2:0,i=Math.sqrt(Math.max(0,1-t*t)),s=e*2.39996;cu[n]=new N(Math.cos(s)*i*Yr,t*Yr*.82,Math.sin(s)*i*Yr)});const X0=new N(0,0,0),hu=n=>cu[n]||X0,$0=Dl.length/Math.max(1,Is.length),Il={};for(const n of et.initiatives)Il[n.name]=Math.min(2,Math.max(.7,Math.cbrt((n.node_count||1)/$0)));const st=[],rs=new Map;for(const n of Dl){const e=W0(n),t=hu(e),i=ui[n.layer]||ui.warm,s=Il[e]||1,r=au(B0(n.id)),a=()=>(r()+r()+r()-1.5)*.9,o=i.spread*s,c={...n,init:e,layer:n.layer||"warm",isHub:!1,pos:new N(t.x+a()*o,t.y+a()*o,t.z+a()*o)};st.push(c),rs.set(c.id,c)}const Y0=Object.fromEntries(et.initiatives.map(n=>[n.name,n.node_count]));for(const n of Is){const e={id:"__hub__"+n,name:n,isHub:!0,hubInit:n,layer:"hub",type:"project core",tier:"—",initiatives:[n],count:Y0[n]||1,pos:hu(n).clone()};st.push(e),rs.set(e.id,e)}const Ii=st.length,en=new Map(st.map((n,e)=>[n.id,e])),$n=[];for(const n of et.edges)en.has(n.src)&&en.has(n.dst)&&$n.push({a:en.get(n.src),b:en.get(n.dst),type:n.type,kind:"real"});for(const n of st){if(n.isHub)continue;const e="__hub__"+n.init;en.has(e)&&$n.push({a:en.get(n.id),b:en.get(e),kind:"spoke"})}const Zs=[],q0=[...et.project_links||[]].sort((n,e)=>(e.weight||0)-(n.weight||0)).slice(0,20);for(const n of q0){const e="__hub__"+n.a,t="__hub__"+n.b;en.has(e)&&en.has(t)&&Zs.push({a:en.get(e),b:en.get(t),w:n.weight})}const fa=st.map(()=>new Set);$n.forEach((n,e)=>{fa[n.a].add(e),fa[n.b].add(e)});const nl=we("graph"),_n=new Kd;_n.fog=new Tl(ze.bg,ze.fog);const $t=new rn(55,innerWidth/innerHeight,1,8e3),xn=new g0({antialias:!0});xn.setPixelRatio(Math.min(2,devicePixelRatio));xn.setSize(innerWidth,innerHeight);xn.setClearColor(ze.bg,1);nl.appendChild(xn.domElement);const It=new x0($t,xn.domElement);It.enableDamping=!0;It.dampingFactor=.08;It.enablePan=!1;It.autoRotate=!0;It.autoRotateSpeed=.4;It.minDistance=200;It.maxDistance=4e3;function j0(){const n=document.createElement("canvas");n.width=n.height=64;const e=n.getContext("2d"),t=e.createRadialGradient(32,32,0,32,32,32);return t.addColorStop(0,"rgba(255,255,255,1)"),t.addColorStop(.55,"rgba(255,255,255,1)"),t.addColorStop(.82,"rgba(255,255,255,0.5)"),t.addColorStop(1,"rgba(255,255,255,0)"),e.fillStyle=t,e.fillRect(0,0,64,64),new Vh(n)}function K0(){const n=document.createElement("canvas");n.width=n.height=128;const e=n.getContext("2d");e.strokeStyle="rgba(236,232,223,0.95)",e.lineWidth=4,e.beginPath(),e.arc(64,64,52,0,7),e.stroke();for(let t=0;t<4;t++){const i=t*Math.PI/2;e.beginPath(),e.moveTo(64+Math.cos(i)*52,64+Math.sin(i)*52),e.lineTo(64+Math.cos(i)*62,64+Math.sin(i)*62),e.stroke()}return new Vh(n)}const Ul=j0(),wi=new Float32Array(Ii*3),qr=new Float32Array(Ii*3),uu=new Float32Array(Ii),Nl=new Float32Array(Ii);st.forEach((n,e)=>{wi[e*3]=n.pos.x,wi[e*3+1]=n.pos.y,wi[e*3+2]=n.pos.z});const li=new Ut;li.setAttribute("position",new At(wi,3));li.setAttribute("aColor",new At(qr,3));li.setAttribute("aSize",new At(uu,1));li.setAttribute("aAlpha",new At(Nl,1));const Z0=new gn({uniforms:{map:{value:Ul}},transparent:!0,depthWrite:!1,vertexShader:`attribute vec3 aColor;attribute float aSize;attribute float aAlpha;
    varying vec3 vC;varying float vA;void main(){vC=aColor;vA=aAlpha;
    vec4 mv=modelViewMatrix*vec4(position,1.0);
    gl_PointSize=aSize*(1100.0/ -mv.z);gl_Position=projectionMatrix*mv;}`,fragmentShader:`uniform sampler2D map;varying vec3 vC;varying float vA;
    void main(){if(vA<0.01)discard;vec4 t=texture2D(map,gl_PointCoord);if(t.a<0.04)discard;gl_FragColor=vec4(vC,t.a*vA);}`}),du=new Rl(li,Z0);du.renderOrder=3;_n.add(du);const fu=new Float32Array($n.length*6),jr=new Float32Array($n.length*6);$n.forEach((n,e)=>{const t=st[n.a].pos,i=st[n.b].pos;let s=i.x,r=i.y,a=i.z;if(n.kind==="spoke"){const o=t.x-i.x,c=t.y-i.y,l=t.z-i.z,u=Math.hypot(o,c,l)||1,g=Math.min(.45,48*(Il[st[n.b].hubInit]||1)/u);s+=o*g,r+=c*g,a+=l*g}fu.set([t.x,t.y,t.z,s,r,a],e*6)});const Sa=new Ut;Sa.setAttribute("position",new At(fu,3));Sa.setAttribute("color",new At(jr,3));const pu=new Al({vertexColors:!0,transparent:!0,opacity:ze.edgeOpacity,depthWrite:!1}),mu=new Hh(Sa,pu);mu.renderOrder=0;_n.add(mu);const gu=new Ut,_u=new Float32Array(Zs.length*6);Zs.forEach((n,e)=>{const t=st[n.a].pos,i=st[n.b].pos;_u.set([t.x,t.y,t.z,i.x,i.y,i.z],e*6)});gu.setAttribute("position",new At(_u,3));const Fl=new Al({color:ze.bridge,transparent:!0,opacity:ze.bridgeRest,depthWrite:!1}),xu=new Hh(gu,Fl);xu.renderOrder=1;_n.add(xu);const Ol=[],Nr=au(99);for(let n=0;n<900;n++)Ol.push(new N(Nr()*2-1,Nr()*2-1,Nr()*2-1).normalize().multiplyScalar(2200+Nr()*1600));const vu=new Ut,Mu=new Float32Array(Ol.length*3);Ol.forEach((n,e)=>Mu.set([n.x,n.y,n.z],e*3));vu.setAttribute("position",new At(Mu,3));const il=new wl({size:3,map:Ul,color:ze.star,transparent:!0,opacity:ze.starOp,blending:ea,depthWrite:!1,sizeAttenuation:!0}),Su=new Rl(vu,il);Su.renderOrder=-2;_n.add(Su);const J0=Zs.flatMap(()=>[.2,.55,.85]),Bl=new Ut,Kr=new Float32Array(J0.length*3);Bl.setAttribute("position",new At(Kr,3));const zl=new Rl(Bl,new wl({size:5,map:Ul,color:ze.particle,transparent:!0,opacity:.8,blending:ea,depthWrite:!1,sizeAttenuation:!0}));zl.renderOrder=2;_n.add(zl);const ai=new nf(new Oh({map:K0(),color:ze.ring,transparent:!0,depthTest:!1,depthWrite:!1}));ai.visible=!1;ai.renderOrder=10;_n.add(ai);let Vs="initiative",yu=!0,bt=null;const Us=et.meta.earliest_secs??0,sl=et.meta.latest_secs??0;let Ns=1/0;const Wt=new Set;let Xt=-1,di=-1;const rl={operational:[.03,1,0],archival:[.53,.9,-.02]},Eu={core:[.12,1,.05],hot:[.03,1,0],warm:[.58,.92,0],cold:[.63,.55,-.04],frozen:[.09,.28,.14]},hh=n=>n<0?0:n>1?1:n,Fs=(n,e)=>{const[t,i,s]=n[e]||[.6,1,0];return new ke().setHSL(t,hh(ze.sat*i),hh(ze.light+s))},Q0=n=>new ke().setHSL(da[n]??.6,ze.sat,ze.light),uh=n=>{if(Vs==="tier")return Fs(rl,n.tier);if(Vs==="layer")return Fs(Eu,n.layer);const e=da[n.init]??da[n.hubInit],t=e==null?new ke("#9af"):new ke().setHSL(e,ze.sat,ze.light),i=ui[n.layer]||ui.warm;return t.lerp(ze.mix,i.mix)},Ws=n=>n.isHub?!0:(n.created_secs??0)<=Ns,Mi=new ke;function Yn(){const n=bt&&!Wt.size,e=Xt>=0?Xt:di,t=Xt>=0?Zr:Js;for(let i=0;i<Ii;i++){const s=st[i],r=ui[s.layer]||ui.warm;let a=r.size;yu&&(s.layer==="core"||s.layer==="hot")&&(a*=1.45);let o=r.a,c=!1,l=!1;Ws(s)?Wt.size?Wt.has(s.id)?(l=!0,s.__cur&&(l="cur"),a=Math.max(a,16)):c=!0:e>=0?i===e?(l=!0,a*=1.6):t.has(i)?l=!0:c=!0:n&&s.init!==bt&&s.hubInit!==bt&&(c=!0):o=0,Mi.copy(uh(s)),l==="cur"?Mi.set(ze.chainCur):l&&Wt.size?Mi.copy(s.__visited?new ke(ze.chainVisited):uh(s)).lerp(ze.hotLerp,ze.chainLerp):c&&Mi.lerp(ze.dimTo,ze.dimAmt),qr[i*3]=Mi.r,qr[i*3+1]=Mi.g,qr[i*3+2]=Mi.b,uu[i]=a,Nl[i]=o}li.attributes.aColor.needsUpdate=li.attributes.aSize.needsUpdate=li.attributes.aAlpha.needsUpdate=!0,Fl.opacity=Wt.size||Xt>=0||di>=0||bt?ze.bridgeDim:ze.bridgeRest,tx()}function ex(n){if(n.kind==="spoke")return ze.spoke;const e=k0[n.type]||H0;return ze.edgeMul===1?e:[e[0]*ze.edgeMul,e[1]*ze.edgeMul,e[2]*ze.edgeMul]}function tx(){const n=bt&&!Wt.size,e=Xt>=0?Xt:di;for(let t=0;t<$n.length;t++){const i=$n[t],s=st[i.a],r=st[i.b];let a=ex(i),o=1;if(!(Ws(s)&&Ws(r)))o=0;else if(Wt.size)o=Wt.has(s.id)&&Wt.has(r.id)&&i.kind!=="spoke"?1.4:.05,o>1&&(a=ze.chainEdge);else if(e>=0)fa[e].has(t)?(o=1.5,a=ze.hotEdge):o=.06;else if(n)o=(s.init===bt||s.hubInit===bt)&&(r.init===bt||r.hubInit===bt)?1:.05;else if(i.kind==="spoke"){const u=st[i.a].layer;o=u==="cold"||u==="frozen"?0:.85}const l=ze.edgeGround;for(let u=0;u<6;u+=3)jr[t*6+u]=l[0]+(a[0]-l[0])*o,jr[t*6+u+1]=l[1]+(a[1]-l[1])*o,jr[t*6+u+2]=l[2]+(a[2]-l[2])*o}Sa.attributes.color.needsUpdate=!0}const al=we("readout");function ol(n,e,t){const i=e?`step <b>${e}</b> / ${t} · knowledge chain`:`node · ${jt(n.type)}`,s=z0[n.layer]||"#caa24a",r=n.created_secs?ru(n.created_secs):"—",a=n.isHub?`<span class="tag k">${n.count} insights</span><span class="tag">project core</span>`:`<span class="tag k">${jt(n.tier)}</span><span class="tag">${jt(n.layer)}</span>${(n.initiatives||[]).map(o=>`<span class="tag">${jt(o)}</span>`).join("")}`;al.innerHTML=`
    <div class="bar" style="background:${s}"></div>
    <div class="body">
      <div class="kicker">${i}</div>
      <h2>${jt(n.name)}</h2>
      <div class="tags">${a}</div>
      <p>${jt(n.body||(n.redacted?"⟨body redacted⟩":n.isHub?"A project cluster — hover its nodes to explore.":"—"))}</p>
      ${n.isHub?"":`<div class="ts">asserted <b>${r}</b></div>`}
      ${n.isHub?"":`<button class="btn go rd" data-read="${jt(n.id)}">read this trail →</button>`}
    </div>`,al.classList.add("show")}const bu=()=>al.classList.remove("show");let Zr=new Set,Js=new Set;const Tn=(()=>{const n=document.createElement("div");return n.id="nodechip",document.body.appendChild(n),n})();Tn.style.cssText="position:fixed;z-index:11;pointer-events:none;transform:translate(-50%,-145%);padding:5px 9px;font-family:var(--serif,Georgia,serif);font-size:13px;white-space:nowrap;display:none";let Ri=null,ll=0,Fr=null,cl=0,kl=!1,is=null,dh=-1,fh=0;const nx=1.1,ix=120;addEventListener("pointermove",n=>{const e=performance.now();if(Fr){const t=Math.max(1,e-ll);cl=Math.hypot(n.clientX-Fr.x,n.clientY-Fr.y)/t}Fr={x:n.clientX,y:n.clientY},ll=e,Ri=!kl&&n.target===xn.domElement?{x:n.clientX,y:n.clientY}:null});addEventListener("pointerleave",()=>{Ri=null});addEventListener("pointerdown",n=>{kl=!0,is={x:n.clientX,y:n.clientY,onCanvas:n.target===xn.domElement}});addEventListener("pointerup",n=>{kl=!1,is&&is.onCanvas&&Math.hypot(n.clientX-is.x,n.clientY-is.y)<5&&(Ri={x:n.clientX,y:n.clientY},wu(Tu())),is=null});const Rn=new N;function Tu(){if(!Ri)return-1;let n=-1,e=22*22;for(let t=0;t<Ii;t++){if(Nl[t]<.05||(Rn.set(wi[t*3],wi[t*3+1],wi[t*3+2]).project($t),Rn.z>1))continue;const i=(Rn.x*.5+.5)*innerWidth,s=(-Rn.y*.5+.5)*innerHeight,r=i-Ri.x,a=s-Ri.y,o=st[t].isHub?2:1,c=(r*r+a*a)/(o*o);c<e&&(e=c,n=t)}return n}function Au(n){const e=new Set([n]);for(const t of fa[n]){const i=$n[t];e.add(i.a),e.add(i.b)}return e}function hl(){const n=Xt>=0?Xt:di;if(Wt.size||(n>=0?ol(st[n]):bu()),n>=0){const e=st[n];ai.position.copy(e.pos);const t=(ui[e.layer]||ui.warm).size*2.6;ai.scale.set(t,t,1),ai.visible=!0}else ai.visible=!1;Yn()}function Hl(n){if(n===Xt)return;if(Xt=n,n<0){Zr=new Set,Tn.style.display="none",nl.style.cursor="",hl();return}Zr=Au(n);const e=st[n];Rn.copy(e.pos).project($t),Tn.style.display="block",Tn.style.left=(Rn.x*.5+.5)*innerWidth+"px",Tn.style.top=(-Rn.y*.5+.5)*innerHeight+"px",Tn.innerHTML=`${jt(e.name)}<div style="font-family:var(--mono,monospace);font-size:9px;letter-spacing:.12em;text-transform:uppercase;color:var(--dim);margin-top:2px">${jt(e.type)} · ${Zr.size-1} neighbours</div>`,nl.style.cursor="pointer",hl()}function wu(n){di=n,Js=n>=0?Au(n):new Set,hl()}let Jr=null;function Ru(n,e=1200){const t=n.length()||1,i=1+320/t;It.autoRotate=!1,It.target.lerp(n,.6),Gl({x:n.x*i,y:n.y*i,z:n.z*i},n,e)}let Os=null;function Gl(n,e,t){const i=$t.position.clone(),s=It.target.clone(),r=performance.now();Os=()=>{const a=Math.min(1,(performance.now()-r)/t),o=t?1-Math.pow(1-a,3):1;$t.position.set(i.x+(n.x-i.x)*o,i.y+(n.y-i.y)*o,i.z+(n.z-i.z)*o),It.target.set(s.x+(e.x-s.x)*o,s.y+(e.y-s.y)*o,s.z+(e.z-s.z)*o),a>=1&&(Os=null)}}function Cu(n){Pu();const e=(n.members||[]).filter(s=>rs.has(s));if(e.length<2)return;di=-1,Js=new Set,Wt.clear(),e.forEach(s=>Wt.add(s)),st.forEach(s=>{s.__visited=!1,s.__cur=!1}),Ei();let t=0;const i=()=>{if(t>0){const r=rs.get(e[t-1]);r.__cur=!1,r.__visited=!0}if(t>=e.length){const r=rs.get(e[e.length-1]);ol(r,e.length,e.length),Yn();return}const s=rs.get(e[t]);s.__cur=!0,s.__visited=!0,ol(s,t+1,e.length),Ru(s.pos),Yn(),t+=1,Jr=setTimeout(i,2400)};i()}function Pu(){Jr&&(clearTimeout(Jr),Jr=null)}function as(){Pu(),Wt.clear(),di=-1,Js=new Set,st.forEach(n=>{n.__visited=!1,n.__cur=!1}),Yn(),bu(),It.autoRotate=!0,fs(900)}const Xs=we("time"),ul=n=>we("timeLabel").textContent=n;function pa(n){n>=100?(Ns=1/0,ul("— full graph —")):(Ns=Us+n/100*(sl-Us),ul(`${ua(Us)} → ${ua(Ns)}`)),Yn()}Xs.addEventListener("input",n=>pa(+n.target.value));let Bs=null;const ma=()=>{Bs&&(clearInterval(Bs),Bs=null)};function sx(){ma();let n=0;Xs.value=0,pa(0),Bs=setInterval(()=>{n+=1.5,n>=100&&(n=100,ma()),Xs.value=n,pa(n)},60)}function Ei(){ma(),Xs.value=100,pa(100)}we("timePlay").addEventListener("click",()=>Bs?ma():sx());function zn(n){bt=n||null,we("focus").value=n||"",we("focus")._sync&&we("focus")._sync(),rx(),di=-1,Js=new Set,Yn(),bt?lx(bt,1400):fs(1200)}function os(n){Vs=n,we("colorMode").value=n,we("colorMode")._sync&&we("colorMode")._sync(),Wl(),Yn()}function zs(n){yu=n,we("glow").checked=n,Yn()}we("colorMode").addEventListener("change",n=>os(n.target.value));we("glow").addEventListener("change",n=>zs(n.target.checked));const $s=we("focus");et.initiatives.forEach(n=>{const e=document.createElement("option");e.value=n.name,e.textContent=`${n.name} (${n.node_count})`,$s.appendChild(e)});$s.addEventListener("change",n=>zn(n.target.value));const dn=we("chainPick");et.chains.forEach((n,e)=>{const t=document.createElement("option");t.value=e,t.textContent=`${n.name} (${(n.members||[]).length})`,dn.appendChild(t)});we("chainPlay").addEventListener("click",()=>{const n=et.chains[+dn.value||0];n&&Cu(n)});we("chainReset").addEventListener("click",as);ds=N0(et,{fmtDateTime:ru});let Ui=!1;const Ds=we("chainRead");async function Ni(n,e){if(n){Ds.textContent="…",we("reader").hidden=!1;const i=e?await ds.showNode(e):await ds.show((et.chains[+dn.value||0]||{}).id);if(Ds.textContent=i?"galaxy":"read",!i){we("reader").hidden=!0;return}const s=ds.chainId();if(s){const r=et.chains.findIndex(a=>a.id===s);r>=0&&+dn.value!==r&&(dn.value=r,dn._sync&&dn._sync())}}Ui=n,we("reader").hidden=!n,n&&(Hl(-1),Tn.style.display="none"),Ds.classList.toggle("go",n),Ds.textContent=n?"galaxy":"read";const t=we("rBack");t&&(t.textContent=n&&vs?"← board":"galaxy"),we("panel").hidden=n,we("rail").hidden=n,we("readout").hidden=n,we("hud").hidden=n}Ds.addEventListener("click",()=>{vs=!1,Ni(!Ui)});we("rBack").addEventListener("click",()=>{const n=vs;Ni(!1),n&&pi(!0,Vl.initiative())});addEventListener("keydown",n=>{if(n.key!=="Escape"||!Ui||/^(INPUT|TEXTAREA)$/.test(n.target.tagName))return;n.preventDefault(),n.stopImmediatePropagation();const e=vs;Ni(!1),e&&pi(!0,Vl.initiative())});we("readout").addEventListener("click",n=>{const e=n.target.closest("[data-read]");e&&(vs=!1,Ni(!0,e.dataset.read))});dn.addEventListener("change",()=>{Ui&&Ni(!0)});const dl=we("boardBtn");let Fi=!1,vs=!1;const Vl=O0(et,{onOpenNode:n=>{pi(!1),vs=!0,Ni(!0,n)}});function pi(n,e){n&&!Vl.show(e||$s.value||"*")||(Fi=n,we("board").hidden=!n,n&&(Hl(-1),Tn.style.display="none"),dl.classList.toggle("go",n),dl.textContent=n?"galaxy":"board",we("panel").hidden=n,we("rail").hidden=n,we("readout").hidden=n,we("hud").hidden=n)}dl.addEventListener("click",()=>pi(!Fi));we("bBack").addEventListener("click",()=>pi(!1));addEventListener("keydown",n=>{n.key!=="Escape"||!Fi||/^(INPUT|TEXTAREA|SELECT)$/.test(n.target.tagName)||(n.preventDefault(),pi(!1))});$s.addEventListener("change",()=>{Fi&&pi(!0,$s.value||"*")});function Lu(n){n.style.display="none";const e=document.createElement("div");e.className="dd";const t=document.createElement("div");t.className="dd-trigger",t.tabIndex=0,t.setAttribute("role","combobox"),t.setAttribute("aria-haspopup","listbox"),t.setAttribute("aria-expanded","false");const i=n.id&&document.querySelector(`label[for="${n.id}"]`);i&&t.setAttribute("aria-label",i.textContent.trim());const s=document.createElement("span");s.className="dd-label";const r=document.createElement("span");r.className="dd-caret",t.append(s,r);const a=document.createElement("div");a.className="dd-menu",a.hidden=!0,a.setAttribute("role","listbox"),e.append(t,a),n.after(e);const o=()=>{a.hidden=!0,t.classList.remove("open"),t.setAttribute("aria-expanded","false"),document.removeEventListener("pointerdown",c,!0),removeEventListener("resize",l),removeEventListener("scroll",l,!0)},c=m=>{e.contains(m.target)||o()};function l(){const m=t.getBoundingClientRect(),_=e.classList.contains("down"),S=5,p=8,d=(_?innerHeight-m.bottom:m.top)-S-p;a.style.minWidth=`${Math.round(m.width)}px`,a.style.maxHeight=`${Math.round(Math.max(120,Math.min(300,d)))}px`;const y=a.getBoundingClientRect().width;a.style.left=`${Math.round(Math.min(Math.max(p,m.left),Math.max(p,innerWidth-y-p)))}px`,a.style.top=_?`${Math.round(m.bottom+S)}px`:"auto",a.style.bottom=_?"auto":`${Math.round(innerHeight-m.top+S)}px`}function u(){const m=n.selectedOptions[0];s.textContent=m?m.textContent:"",[...a.children].forEach(_=>{const S=_.dataset.value===n.value;_.classList.toggle("sel",S),_.setAttribute("aria-selected",String(S))})}function g(){a.innerHTML="";for(const m of n.options){const _=document.createElement("div");_.className="dd-opt",_.dataset.value=m.value,_.setAttribute("role","option"),_.setAttribute("aria-selected",String(m.value===n.value));const S=m.textContent.match(/^(.*?)\s*\(([^)]+)\)\s*$/),p=document.createElement("span");if(p.className="dd-optlabel",p.textContent=S?S[1]:m.textContent,_.append(p),S){const d=document.createElement("span");d.className="dd-meta",d.textContent=S[2],_.append(d)}_.addEventListener("click",()=>{n.value=m.value,n.dispatchEvent(new Event("change")),u(),o()}),a.append(_)}u()}const h=()=>{g(),a.hidden=!1,l(),t.classList.add("open"),t.setAttribute("aria-expanded","true"),document.addEventListener("pointerdown",c,!0),addEventListener("resize",l),addEventListener("scroll",l,!0)};return t.addEventListener("click",()=>{a.hidden?h():o()}),t.addEventListener("keydown",m=>{m.key==="Enter"||m.key===" "||m.key==="ArrowDown"?(m.preventDefault(),a.hidden?h():o()):m.key==="Escape"&&!a.hidden&&(m.stopPropagation(),o())}),n._sync=u,g(),u}["chainPick","colorMode","focus"].forEach(n=>Lu(we(n)));Lu(we("bInit")).call;we("bInit").nextElementSibling.classList.add("down");const Or=et.meta;we("stats").innerHTML=`<div class="cell"><div class="n">${Or.node_count??et.nodes.length}</div><div class="l">insights</div></div><div class="cell"><div class="n">${Or.edge_count??et.edges.length}</div><div class="l">links</div></div><div class="cell"><div class="n">${Or.initiative_count??et.initiatives.length}</div><div class="l">projects</div></div><div class="cell"><div class="n">${Or.chain_count??et.chains.length}</div><div class="l">chains</div></div><div class="cell span">${ua(Us)} — ${ua(sl)}<span class="sub">one agent · ${Math.round((sl-Us)/86400)} days</span></div>`;function Qr(n){return"#"+n.getHexString()}function rx(){const n=we("projects");if(n)for(const e of n.children){const t=(e.dataset.init||null)===bt;e.classList.toggle("on",t),e.setAttribute("aria-pressed",String(t))}}function Du(){const n=we("projects"),e=[...et.initiatives].sort((t,i)=>(i.node_count||0)-(t.node_count||0));n.innerHTML=`<button type="button" class="pr all${bt?"":" on"}" data-init=""
       aria-pressed="${bt?"false":"true"}">
       <span class="nm">all projects</span><span class="ct">${Dl.length}</span>
       <span class="sw" style="background:transparent"></span></button>`+e.map(t=>`<button type="button" class="pr${bt===t.name?" on":""}"
       data-init="${jt(t.name)}" aria-pressed="${bt===t.name?"true":"false"}">
       <span class="nm">${jt(t.name)}</span><span class="ct">${t.node_count||0}</span>
       <span class="sw" style="background:${Qr(Q0(t.name))}"></span></button>`).join("");for(const t of n.children)t.addEventListener("click",()=>{const i=t.dataset.init||null;zn(bt===i?null:i)});ox(n)}const ax=12;function ox(n){const e=()=>{const t=n.querySelectorAll(".pr");if(t.length){const i=n.getBoundingClientRect().top-n.scrollTop,s=o=>t[o].getBoundingClientRect().bottom-i,r=innerHeight-n.getBoundingClientRect().top-96;let a=Math.min(ax,t.length);for(;a>4&&s(a-1)>r;)a--;n.style.maxHeight=`${Math.ceil(s(a-1))}px`}n.classList.toggle("more",n.scrollHeight-n.clientHeight-n.scrollTop>4)};e(),n.addEventListener("scroll",e,{passive:!0}),addEventListener("resize",e)}function Wl(){const n=we("legend");Vs==="initiative"?n.innerHTML="":Vs==="tier"?n.innerHTML=`operational (hippocampus) <span class="sw" style="background:${Qr(Fs(rl,"operational"))}"></span><br>archival (cortex) <span class="sw" style="background:${Qr(Fs(rl,"archival"))}"></span>`:n.innerHTML=["core","hot","warm","cold","frozen"].map(e=>`${e} <span class="sw" style="background:${Qr(Fs(Eu,e))}"></span>`).join("<br>")}Wl();Du();function Iu(n){let e=1/0,t=1/0,i=1/0,s=-1/0,r=-1/0,a=-1/0,o=!1;for(let u=0;u<Ii;u++){const g=st[u];if(g.isHub||!Ws(g)||n&&!n(g))continue;const h=g.pos;o=!0,h.x<e&&(e=h.x),h.x>s&&(s=h.x),h.y<t&&(t=h.y),h.y>r&&(r=h.y),h.z<i&&(i=h.z),h.z>a&&(a=h.z)}if(!o)return{c:new N,maxR:Yr};const c=new N((e+s)/2,(t+r)/2,(i+a)/2),l=Math.max(s-e,r-t,a-i)/2;return{c,maxR:l}}function fs(n=1200){const{c:e,maxR:t}=Iu(null),i=t/Math.tan($t.fov*Math.PI/180/2)*1;Gl({x:e.x,y:e.y,z:e.z+i},e,n)}function lx(n,e=1400){const{c:t,maxR:i}=Iu(r=>r.init===n),s=Math.max(260,i/Math.tan($t.fov*Math.PI/180/2)*1.8);It.autoRotate=!1,Gl({x:t.x,y:t.y,z:t.z+s},t,e)}const ls=[{tag:"a knowledge galaxy",title:"One mind. A knowledge galaxy.",narr:"Months of one agent's work across many projects, in one memory. Same rule shapes it as a galaxy: what belongs together pulls together. Each cluster a project, each point a thought.",apply(){as(),Ei(),zn(null),zs(!0),os("initiative"),fs(1500)}},{tag:"reasoning chains",title:"How — not just what.",narr:"A knowledge chain is the load-bearing path between insights. Watch how one conclusion was reached — node by node, in order.",apply(){Ei(),zn(null);const n=et.chains.reduce((e,t)=>(t.members||[]).length>(e?(e.members||[]).length:0)?t:e,null);n&&(dn.value=et.chains.indexOf(n),dn._sync&&dn._sync(),Cu(n))}},{tag:"one project, up close",title:"Each cluster is a real project.",narr:"Zoom into a single project and the structure appears: keystone facts in Core, standing rules in Hot, working notes in Warm — scoped and prioritized.",apply(){as(),Ei(),zs(!0),os("layer"),zn((et.initiatives[0]||{}).name||null)}},{tag:"memory layers",title:"Important glows first.",narr:"Memory has priority. Core and Hot glow largest and load on every re-entry; Warm is the working set; Cold and Frozen wait until asked.",apply(){as(),Ei(),zn(null),zs(!0),os("layer"),fs(1200)}},{tag:"two tiers",title:"Hippocampus & cortex.",narr:"Two tiers, like the brain. Operational is fast, messy working thought; archival is settled, durable knowledge. Operational decays and gets revisited; archival is what survives.",apply(){as(),Ei(),zn(null),os("tier"),fs(1200)}}];let ya=-1;function cx(n){we("scriptDots").innerHTML=ls.map((e,t)=>`<span class="dot${t===n?" on":""}"></span>`).join("")}function Xl(n){if(n<0||n>=ls.length)return;ya=n;const e=ls[n];we("scriptTag").textContent=e.tag,we("scriptNum").textContent=`${n+1} / ${ls.length}`,we("scriptTitle").textContent=e.title,we("scriptNarr").textContent=e.narr,we("scriptPrev").disabled=n===0,we("scriptNext").disabled=n===ls.length-1,cx(n),e.apply()}function hx(){we("panel").hidden=!0,we("script").hidden=!1,It.autoRotate=!1,Xl(0)}function Uu(){we("script").hidden=!0,we("panel").hidden=!1,ya=-1,as(),Ei(),zn(null),zs(!0),os("initiative")}const Nu=()=>Xl(Math.min(ya+1,ls.length-1)),Fu=()=>Xl(Math.max(ya-1,0));we("talkBtn").addEventListener("click",hx);we("scriptNext").addEventListener("click",Nu);we("scriptPrev").addEventListener("click",Fu);we("scriptExit").addEventListener("click",Uu);addEventListener("keydown",n=>{we("script").hidden||/^(INPUT|TEXTAREA)$/.test(n.target.tagName)||(n.key==="ArrowRight"||n.key===" "?(n.preventDefault(),Nu()):n.key==="ArrowLeft"?(n.preventDefault(),Fu()):n.key==="Escape"&&Uu())});function Ou(){xn.setClearColor(ze.bg,1),_n.fog.color.set(ze.bg),_n.fog.density=ze.fog,pu.opacity=ze.edgeOpacity,Fl.color.set(ze.bridge),zl.material.color.set(ze.particle),il.color.set(ze.star),il.opacity=ze.starOp,ai.material.color.set(ze.ring),Wl(),Du(),Yn(),ds&&ds.redraw()}function Bu(n){Gs=n;try{localStorage.setItem("kaeru-viz-theme",n)}catch{}n==="auto"?document.documentElement.removeAttribute("data-theme"):document.documentElement.setAttribute("data-theme",n),ze=Pl[Ll()],Ou();const e=we("themeSeg");if(e)for(const t of e.children)t.classList.toggle("on",t.dataset.t===n)}for(const n of we("themeSeg").children)n.addEventListener("click",()=>Bu(n.dataset.t));lu.addEventListener("change",()=>{Gs==="auto"&&(ze=Pl[Ll()],Ou())});const Dt=we("findInput"),bi=we("findHits");let ln=[],fn=-1;const Br=st.filter(n=>!n.isHub);function zu(n){const e=st[n];e&&(Ws(e)||(Ns=1/0,Xs.value=100,ul("— full graph —")),bt&&e.init!==bt&&zn(null),Ui&&Ni(!1),Fi&&pi(!1),Wt.clear(),st.forEach(t=>{t.__visited=!1,t.__cur=!1}),wu(n),Ru(e.pos,ou.matches?0:1100),fl(`Showing ${e.name}`))}function Ys(){bi.hidden=!0,ln=[],fn=-1,Dt.setAttribute("aria-expanded","false"),Dt.removeAttribute("aria-activedescendant")}function fl(n){we("findStatus").textContent=n}function ku(){if(Dt.setAttribute("aria-expanded","true"),!ln.length){bi.innerHTML='<div class="none">nothing found</div>',bi.hidden=!1,Dt.removeAttribute("aria-activedescendant"),fl("No nodes found");return}bi.innerHTML=ln.map((n,e)=>{const t=st[n];return`<div class="hit${e===fn?" on":""}" id="hit-${n}" role="option"
      aria-selected="${e===fn}" data-i="${n}">
      <span class="hn">${jt(t.name)}</span>
      <span class="hm">${jt(t.type)} · ${jt(t.init)}</span></div>`}).join(""),bi.hidden=!1,fn>=0&&Dt.setAttribute("aria-activedescendant",`hit-${ln[fn]}`),fl(`${ln.length} node${ln.length===1?"":"s"} found`);for(const n of bi.children)n.dataset.i&&n.addEventListener("mousedown",e=>{e.preventDefault(),zu(+n.dataset.i),Ys(),Dt.blur()})}function Hu(){const n=Dt.value.trim().toLowerCase();if(n.length<2){Ys();return}const e=[],t=[];for(let i=0;i<Br.length&&e.length+t.length<200;i++){const r=Br[i].name.toLowerCase().indexOf(n);r===0?e.push(en.get(Br[i].id)):r>0&&t.push(en.get(Br[i].id))}ln=[...e,...t].slice(0,40),fn=ln.length?0:-1,ku()}Dt.addEventListener("input",Hu);Dt.addEventListener("focus",()=>{Dt.value.trim().length>=2&&Hu()});Dt.addEventListener("blur",()=>setTimeout(Ys,120));Dt.addEventListener("keydown",n=>{if(n.key==="Escape"){Dt.value="",Ys(),Dt.blur();return}ln.length&&(n.key==="ArrowDown"||n.key==="ArrowUp"?(n.preventDefault(),fn=(fn+(n.key==="ArrowDown"?1:ln.length-1))%ln.length,ku(),bi.children[fn]?.scrollIntoView({block:"nearest"})):n.key==="Enter"&&fn>=0&&(n.preventDefault(),zu(ln[fn]),Ys(),Dt.blur()))});addEventListener("keydown",n=>{n.key!=="/"||n.target.tagName==="INPUT"||!we("script").hidden||Ui||Fi||(n.preventDefault(),Dt.focus(),Dt.select())});addEventListener("resize",()=>{$t.aspect=innerWidth/innerHeight,$t.updateProjectionMatrix(),xn.setSize(innerWidth,innerHeight)});let ph=0;function Gu(){requestAnimationFrame(Gu),ph+=1,performance.now()-ll>80&&(cl=0);const n=Ri&&cl<nx?Tu():-1;n!==dh&&(dh=n,fh=performance.now());let e=Xt;n<0?e=-1:performance.now()-fh>=ix&&(e=n),e!==Xt?Hl(e):Xt>=0&&(Rn.copy(st[Xt].pos).project($t),Tn.style.left=(Rn.x*.5+.5)*innerWidth+"px",Tn.style.top=(-Rn.y*.5+.5)*innerHeight+"px");let t=0;for(const i of Zs){const s=st[i.a].pos,r=st[i.b].pos;for(let a=0;a<3;a++){const o=(ph*.0016+a/3)%1;Kr[t*3]=s.x+(r.x-s.x)*o,Kr[t*3+1]=s.y+(r.y-s.y)*o+Math.sin(o*Math.PI)*40,Kr[t*3+2]=s.z+(r.z-s.z)*o,t++}}Bl.attributes.position.needsUpdate=!0,Os&&Os(),It.autoRotate=!ou.matches&&!(window.__rec&&window.__rec.active)&&Xt<0&&!Wt.size&&!bt&&we("script").hidden&&!Os,It.update(),!Ui&&!Fi&&xn.render(_n,$t)}if(new URLSearchParams(location.search).has("rec")){let n=null;window.__rec={active:!1,orbit(e){this.active=!0;const t=It.target;if(!n){const s=$t.position.clone().sub(t);n={r:Math.hypot(s.x,s.z),y:s.y}}const i=e*Math.PI*2;$t.position.set(t.x+Math.sin(i)*n.r,t.y+n.y,t.z+Math.cos(i)*n.r),$t.lookAt(t),xn.render(_n,$t)}}}Bu(Gs);fs(0);Gu();
