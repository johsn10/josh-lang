let Q=0,S=`string`,R=1,U=`Object`,O=`utf-8`,M=null,N=`undefined`,V=4,T=`function`,K=128,J=Array,P=Error,W=Object,L=undefined;var D=(async(a,b)=>{if(typeof Response===T&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===T){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var l=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==M){return `${a}`};if(b==S){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==M){return `Symbol`}else{return `Symbol(${b})`}};if(b==T){const b=a.name;if(typeof b==S&&b.length>Q){return `Function(${b})`}else{return `Function`}};if(J.isArray(a)){const b=a.length;let c=`[`;if(b>Q){c+=l(a[Q])};for(let d=R;d<b;d++){c+=`, `+ l(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>R){d=c[R]}else{return toString.call(a)};if(d==U){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return U}};if(a instanceof P){return `${a.name}: ${a.message}\n${a.stack}`};return d});var F=((a,b)=>{});var B=((a,b)=>{a=a>>>Q;const c=A();const d=c.subarray(a/V,a/V+ b);const e=[];for(let a=Q;a<d.length;a++){e.push(f(d[a]))};return e});var k=(a=>{if(d===b.length)b.push(b.length+ R);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});var v=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_ref__h84c0d7df384fc35c(c,d,u(e))}finally{b[t++]=L}});function C(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}var I=(async(b)=>{if(a!==L)return a;if(typeof b===N){b=new URL(`first_yew-65a55b398cd3381e_bg.wasm`,import.meta.url)};const c=E();if(typeof b===S||typeof Request===T&&b instanceof Request||typeof URL===T&&b instanceof URL){b=fetch(b)};F(c);const {instance:d,module:e}=await D(await b,c);return G(d,e)});var H=(b=>{if(a!==L)return a;const c=E();F(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return G(d,b)});var r=(()=>{if(q===M||q.byteLength===Q){q=new Int32Array(a.memory.buffer)};return q});var y=(a=>a===L||a===M);var c=(a=>b[a]);var G=((b,c)=>{a=b.exports;I.__wbindgen_wasm_module=c;q=M;z=M;h=M;a.__wbindgen_start();return a});var E=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;r()[a/V+ R]=y(d)?Q:d;r()[a/V+ Q]=!y(d)});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;r()[a/V+ R]=y(d)?Q:d;r()[a/V+ Q]=!y(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>Q});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>Q});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;r()[a/V+ R]=y(d)?Q:d;r()[a/V+ Q]=!y(d)});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>Q});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==R){b.a=Q;return !0};const c=!1;return c});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new P();return k(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/V+ R]=g;r()[b/V+ Q]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(j(b,c))}finally{a.__wbindgen_free(d,e,R)}});b.wbg.__wbg_queueMicrotask_118eeb525d584d9a=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_26a89c14c53809c0=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===T;return b});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=B(b,c).slice();a.__wbindgen_free(b,c*V,V);console.error(...d)});b.wbg.__wbg_log_7c3433e130418e14=((b,c)=>{var d=B(b,c).slice();a.__wbindgen_free(b,c*V,V);console.log(...d)});b.wbg.__wbg_body_3eb73da919b867a1=(a=>{const b=c(a).body;return y(b)?Q:k(b)});b.wbg.__wbg_createElement_1a136faad4101f43=function(){return C(((a,b,d)=>{const e=c(a).createElement(j(b,d));return k(e)}),arguments)};b.wbg.__wbg_createElementNS_d47e0c50fa2904e0=function(){return C(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===Q?L:j(b,d),j(e,f));return k(g)}),arguments)};b.wbg.__wbg_createTextNode_dbdd908f92bae1b1=((a,b,d)=>{const e=c(a).createTextNode(j(b,d));return k(e)});b.wbg.__wbg_instanceof_Window_99dc9805eaa2614b=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5257b70811e953c0=(a=>{const b=c(a).document;return y(b)?Q:k(b)});b.wbg.__wbg_instanceof_Element_f614cf57d4316979=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_0819c2800784a176=((b,d)=>{const e=c(d).namespaceURI;var f=y(e)?Q:p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=m;r()[b/V+ R]=g;r()[b/V+ Q]=f});b.wbg.__wbg_setinnerHTML_99deeacfff0ae4cc=((a,b,d)=>{c(a).innerHTML=j(b,d)});b.wbg.__wbg_outerHTML_69934f9195df65af=((b,d)=>{const e=c(d).outerHTML;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/V+ R]=g;r()[b/V+ Q]=f});b.wbg.__wbg_removeAttribute_5c264e727b67dbdb=function(){return C(((a,b,d)=>{c(a).removeAttribute(j(b,d))}),arguments)};b.wbg.__wbg_setAttribute_0918ea45d5a1c663=function(){return C(((a,b,d,e,f)=>{c(a).setAttribute(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_target_791826e938c3e308=(a=>{const b=c(a).target;return y(b)?Q:k(b)});b.wbg.__wbg_bubbles_f0783dc095f8e220=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_191799b8e0ab3254=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_d94a39b8c8f6eed1=(a=>{const b=c(a).composedPath();return k(b)});b.wbg.__wbg_value_ab23a75318ea828f=((b,d)=>{const e=c(d).value;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/V+ R]=g;r()[b/V+ Q]=f});b.wbg.__wbg_setvalue_918a8ae77531a942=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_parentNode_f3957fdd408a62f7=(a=>{const b=c(a).parentNode;return y(b)?Q:k(b)});b.wbg.__wbg_parentElement_86a7612dde875ba9=(a=>{const b=c(a).parentElement;return y(b)?Q:k(b)});b.wbg.__wbg_childNodes_75d3da5f3a7bb985=(a=>{const b=c(a).childNodes;return k(b)});b.wbg.__wbg_lastChild_8f7b6f3825115eff=(a=>{const b=c(a).lastChild;return y(b)?Q:k(b)});b.wbg.__wbg_nextSibling_13e9454ef5323f1a=(a=>{const b=c(a).nextSibling;return y(b)?Q:k(b)});b.wbg.__wbg_setnodeValue_8656e865e9b11bbb=((a,b,d)=>{c(a).nodeValue=b===Q?L:j(b,d)});b.wbg.__wbg_textContent_efe8338af53ddf62=((b,d)=>{const e=c(d).textContent;var f=y(e)?Q:p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=m;r()[b/V+ R]=g;r()[b/V+ Q]=f});b.wbg.__wbg_cloneNode_80501c66ab115588=function(){return C((a=>{const b=c(a).cloneNode();return k(b)}),arguments)};b.wbg.__wbg_insertBefore_882082ef4c5d7766=function(){return C(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_removeChild_14b08321b677677a=function(){return C(((a,b)=>{const d=c(a).removeChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_cb6366cb0956ce29=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_99e27ed8897850f2=(a=>{const b=c(a).host;return k(b)});b.wbg.__wbg_addEventListener_1b158e9e95e0ab00=function(){return C(((a,b,d,e,f)=>{c(a).addEventListener(j(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_setchecked_3b12f3d602a63e47=((a,b)=>{c(a).checked=b!==Q});b.wbg.__wbg_value_c93cb4b4d352228e=((b,d)=>{const e=c(d).value;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/V+ R]=g;r()[b/V+ Q]=f});b.wbg.__wbg_setvalue_9bd3f93b3864ddbf=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_get_c43534c00f382c8a=((a,b)=>{const d=c(a)[b>>>Q];return k(d)});b.wbg.__wbg_length_d99b680fd68bf71b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_5859b6d41c6fe9f7=((a,b)=>{const c=new Function(j(a,b));return k(c)});b.wbg.__wbg_call_a79f1973a4f07d5e=function(){return C(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_new_87d841e70661f6e9=(()=>{const a=new W();return k(a)});b.wbg.__wbg_self_086b5302bcafb962=function(){return C((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_132fa5d7546f1de5=function(){return C((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_e5f801a37ad7d07b=function(){return C((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_f9a61fce4af6b7c1=function(){return C((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===L;return b});b.wbg.__wbg_from_a663e01d8dab8e44=(a=>{const b=J.from(c(a));return k(b)});b.wbg.__wbg_is_a5728dbfb61c82cd=((a,b)=>{const d=W.is(c(a),c(b));return d});b.wbg.__wbg_resolve_97ecd55ee839391b=(a=>{const b=Promise.resolve(c(a));return k(b)});b.wbg.__wbg_then_7aeb7c5f1536640f=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_set_37a50e901587b477=function(){return C(((a,b,d)=>{const e=Reflect.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=l(c(d));const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/V+ R]=g;r()[b/V+ Q]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new P(j(a,b))});b.wbg.__wbindgen_closure_wrapper1762=((a,b,c)=>{const d=s(a,b,921,v);return k(d)});b.wbg.__wbindgen_closure_wrapper1912=((a,b,c)=>{const d=w(a,b,976,x);return k(d)});return b});var A=(()=>{if(z===M||z.byteLength===Q){z=new Uint32Array(a.memory.buffer)};return z});var x=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he5a13ab9feccda9d(b,c,k(d))});var s=((b,c,d,e)=>{const f={a:b,b:c,cnt:R,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===Q){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=Q}}};g.original=f;return g});var e=(a=>{if(a<132)return;b[a]=d;d=a});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:R,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=Q;try{return e(c,f.b,...b)}finally{if(--f.cnt===Q){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var p=((a,b,c)=>{if(c===L){const c=n.encode(a);const d=b(c.length,R)>>>Q;i().subarray(d,d+ c.length).set(c);m=c.length;return d};let d=a.length;let e=b(d,R)>>>Q;const f=i();let g=Q;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==Q){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,R)>>>Q;const b=i().subarray(e+ g,e+ d);const f=o(a,b);g+=f.written};m=g;return e});var i=(()=>{if(h===M||h.byteLength===Q){h=new Uint8Array(a.memory.buffer)};return h});var j=((a,b)=>{a=a>>>Q;return g.decode(i().subarray(a,a+ b))});var u=(a=>{if(t==R)throw new P(`out of js stack`);b[--t]=a;return t});let a;const b=new J(K).fill(L);b.push(L,M,!0,!1);let d=b.length;const g=typeof TextDecoder!==N?new TextDecoder(O,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw P(`TextDecoder not available`)}};if(typeof TextDecoder!==N){g.decode()};let h=M;let m=Q;const n=typeof TextEncoder!==N?new TextEncoder(O):{encode:()=>{throw P(`TextEncoder not available`)}};const o=typeof n.encodeInto===T?((a,b)=>n.encodeInto(a,b)):((a,b)=>{const c=n.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=M;let t=K;let z=M;export default I;export{H as initSync}