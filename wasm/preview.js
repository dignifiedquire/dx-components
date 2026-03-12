//#region js imports
import { RawInterpreter } from './snippets/dioxus-interpreter-js-c0f0333193e32f17/inline0.js';
import { setAttributeInner } from './snippets/dioxus-interpreter-js-c0f0333193e32f17/src/js/set_attribute.js';
import { get_select_data } from './snippets/dioxus-web-afdd4c4e5aff9d11/inline0.js';
import { WebDioxusChannel } from './snippets/dioxus-web-afdd4c4e5aff9d11/src/js/eval.js';

//#endregion

//#region exports

export class JSOwner {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JSOwner.prototype);
        obj.__wbg_ptr = ptr;
        JSOwnerFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JSOwnerFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsowner_free(ptr, 0);
    }
}
if (Symbol.dispose) JSOwner.prototype[Symbol.dispose] = JSOwner.prototype.free;

//#endregion

//#region wasm imports
import * as import1 from "./__wasm_split.js"
import * as import2 from "./__wasm_split.js"
import * as import3 from "./__wasm_split.js"
import * as import4 from "./__wasm_split.js"
import * as import5 from "./__wasm_split.js"
import * as import6 from "./__wasm_split.js"
import * as import7 from "./__wasm_split.js"
import * as import8 from "./__wasm_split.js"
import * as import9 from "./__wasm_split.js"
import * as import10 from "./__wasm_split.js"
import * as import11 from "./__wasm_split.js"
import * as import12 from "./__wasm_split.js"
import * as import13 from "./__wasm_split.js"
import * as import14 from "./__wasm_split.js"
import * as import15 from "./__wasm_split.js"
import * as import16 from "./__wasm_split.js"
import * as import17 from "./__wasm_split.js"
import * as import18 from "./__wasm_split.js"
import * as import19 from "./__wasm_split.js"
import * as import20 from "./__wasm_split.js"
import * as import21 from "./__wasm_split.js"
import * as import22 from "./__wasm_split.js"
import * as import23 from "./__wasm_split.js"
import * as import24 from "./__wasm_split.js"
import * as import25 from "./__wasm_split.js"
import * as import26 from "./__wasm_split.js"
import * as import27 from "./__wasm_split.js"
import * as import28 from "./__wasm_split.js"
import * as import29 from "./__wasm_split.js"
import * as import30 from "./__wasm_split.js"
import * as import31 from "./__wasm_split.js"
import * as import32 from "./__wasm_split.js"
import * as import33 from "./__wasm_split.js"
import * as import34 from "./__wasm_split.js"
import * as import35 from "./__wasm_split.js"
import * as import36 from "./__wasm_split.js"
import * as import37 from "./__wasm_split.js"
import * as import38 from "./__wasm_split.js"
import * as import39 from "./__wasm_split.js"
import * as import40 from "./__wasm_split.js"
import * as import41 from "./__wasm_split.js"
import * as import42 from "./__wasm_split.js"
import * as import43 from "./__wasm_split.js"
import * as import44 from "./__wasm_split.js"
import * as import45 from "./__wasm_split.js"
import * as import46 from "./__wasm_split.js"
import * as import47 from "./__wasm_split.js"
import * as import48 from "./__wasm_split.js"
import * as import49 from "./__wasm_split.js"
import * as import50 from "./__wasm_split.js"
import * as import51 from "./__wasm_split.js"
import * as import52 from "./__wasm_split.js"
import * as import53 from "./__wasm_split.js"
import * as import54 from "./__wasm_split.js"
import * as import55 from "./__wasm_split.js"
import * as import56 from "./__wasm_split.js"
import * as import57 from "./__wasm_split.js"
import * as import58 from "./__wasm_split.js"
import * as import59 from "./__wasm_split.js"
import * as import60 from "./__wasm_split.js"
import * as import61 from "./__wasm_split.js"
import * as import62 from "./__wasm_split.js"
import * as import63 from "./__wasm_split.js"
import * as import64 from "./__wasm_split.js"
import * as import65 from "./__wasm_split.js"
import * as import66 from "./__wasm_split.js"
import * as import67 from "./__wasm_split.js"
import * as import68 from "./__wasm_split.js"
import * as import69 from "./__wasm_split.js"
import * as import70 from "./__wasm_split.js"
import * as import71 from "./__wasm_split.js"
import * as import72 from "./__wasm_split.js"
import * as import73 from "./__wasm_split.js"
import * as import74 from "./__wasm_split.js"
import * as import75 from "./__wasm_split.js"
import * as import76 from "./__wasm_split.js"
import * as import77 from "./__wasm_split.js"
import * as import78 from "./__wasm_split.js"
import * as import79 from "./__wasm_split.js"
import * as import80 from "./__wasm_split.js"
import * as import81 from "./__wasm_split.js"
import * as import82 from "./__wasm_split.js"
import * as import83 from "./__wasm_split.js"
import * as import84 from "./__wasm_split.js"
import * as import85 from "./__wasm_split.js"
import * as import86 from "./__wasm_split.js"
import * as import87 from "./__wasm_split.js"
import * as import88 from "./__wasm_split.js"
import * as import89 from "./__wasm_split.js"
import * as import90 from "./__wasm_split.js"
import * as import91 from "./__wasm_split.js"
import * as import92 from "./__wasm_split.js"
import * as import93 from "./__wasm_split.js"
import * as import94 from "./__wasm_split.js"
import * as import95 from "./__wasm_split.js"
import * as import96 from "./__wasm_split.js"
import * as import97 from "./__wasm_split.js"
import * as import98 from "./__wasm_split.js"

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_8c4e43fe74559d73: function() { return logError(function (arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_String_8f0eb39a4a4c2f66: function() { return logError(function (arg0, arg1) {
            const ret = String(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg___wbindgen_bigint_get_as_i64_8fcf4ce7f1ca72a2: function(arg0, arg1) {
            const v = arg1;
            const ret = typeof(v) === 'bigint' ? v : undefined;
            if (!isLikeNone(ret)) {
                _assertBigInt(ret);
            }
            getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_boolean_get_bbbb1c18aa2f5e25: function(arg0) {
            const v = arg0;
            const ret = typeof(v) === 'boolean' ? v : undefined;
            if (!isLikeNone(ret)) {
                _assertBoolean(ret);
            }
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_debug_string_0bc8482c6e3508ae: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_in_47fa6863be6f2f25: function(arg0, arg1) {
            const ret = arg0 in arg1;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_bigint_31b12575b56f32fc: function(arg0) {
            const ret = typeof(arg0) === 'bigint';
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_function_0095a73b8b156f76: function(arg0) {
            const ret = typeof(arg0) === 'function';
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_object_5ae8e5880f2c1fbd: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_string_cd444516edc5b180: function(arg0) {
            const ret = typeof(arg0) === 'string';
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_is_undefined_9e4d92534c42d778: function(arg0) {
            const ret = arg0 === undefined;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_jsval_eq_11888390b0186270: function(arg0, arg1) {
            const ret = arg0 === arg1;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_jsval_loose_eq_9dd77d8cd6671811: function(arg0, arg1) {
            const ret = arg0 == arg1;
            _assertBoolean(ret);
            return ret;
        },
        __wbg___wbindgen_memory_bd1fbcf21fbef3c8: function() {
            const ret = wasm.memory;
            return ret;
        },
        __wbg___wbindgen_number_get_8ff4255516ccad3e: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            if (!isLikeNone(ret)) {
                _assertNum(ret);
            }
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_72fb696202c56729: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_d9b87ff7982e3b21: function() { return logError(function (arg0) {
            arg0._wbg_cb_unref();
        }, arguments); },
        __wbg_addEventListener_3acb0aad4483804c: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_altKey_73c1173ba53073d5: function() { return logError(function (arg0) {
            const ret = arg0.altKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_altKey_8155c319c215e3aa: function() { return logError(function (arg0) {
            const ret = arg0.altKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_altKey_a9d4404a96cee91e: function() { return logError(function (arg0) {
            const ret = arg0.altKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_animationName_370c082ffd589944: function() { return logError(function (arg0, arg1) {
            const ret = arg1.animationName;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_appendChild_dea38765a26d346d: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.appendChild(arg1);
            return ret;
        }, arguments); },
        __wbg_arrayBuffer_05ce1af23e9064e8: function() { return logError(function (arg0) {
            const ret = arg0.arrayBuffer();
            return ret;
        }, arguments); },
        __wbg_back_42e1073071def227: function() { return handleError(function (arg0) {
            arg0.back();
        }, arguments); },
        __wbg_blockSize_ef9a626745d7dfac: function() { return logError(function (arg0) {
            const ret = arg0.blockSize;
            return ret;
        }, arguments); },
        __wbg_blur_07f34335e06e5234: function() { return handleError(function (arg0) {
            arg0.blur();
        }, arguments); },
        __wbg_borderBoxSize_4ab11d62b5b77005: function() { return logError(function (arg0) {
            const ret = arg0.borderBoxSize;
            return ret;
        }, arguments); },
        __wbg_bubbles_ad88192d3c29e6f9: function() { return logError(function (arg0) {
            const ret = arg0.bubbles;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_button_d86841d0a03adc44: function() { return logError(function (arg0) {
            const ret = arg0.button;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_buttons_a158a0cad3175f24: function() { return logError(function (arg0) {
            const ret = arg0.buttons;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_call_389efe28435a9388: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_4708e0c13bdc8e95: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_changedTouches_b6ab7be7b1aed8d6: function() { return logError(function (arg0) {
            const ret = arg0.changedTouches;
            return ret;
        }, arguments); },
        __wbg_charCodeAt_8fdb057472688076: function() { return logError(function (arg0, arg1) {
            const ret = arg0.charCodeAt(arg1 >>> 0);
            return ret;
        }, arguments); },
        __wbg_checkValidity_211998f3bd3d6222: function() { return logError(function (arg0) {
            const ret = arg0.checkValidity();
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_checked_04db83ac6810bc82: function() { return logError(function (arg0) {
            const ret = arg0.checked;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_clearData_6e4fe5c1c18e6a19: function() { return handleError(function (arg0) {
            arg0.clearData();
        }, arguments); },
        __wbg_clearData_774f974a13a4aca8: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.clearData(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_clearTimeout_5a54f8841c30079a: function() { return logError(function (arg0) {
            const ret = clearTimeout(arg0);
            return ret;
        }, arguments); },
        __wbg_clientX_a3c5f4ff30e91264: function() { return logError(function (arg0) {
            const ret = arg0.clientX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_clientX_ed7d2827ca30c165: function() { return logError(function (arg0) {
            const ret = arg0.clientX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_clientY_79ab4711d0597b2c: function() { return logError(function (arg0) {
            const ret = arg0.clientY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_clientY_e28509acb9b4a42a: function() { return logError(function (arg0) {
            const ret = arg0.clientY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_code_dee0dae4730408e1: function() { return logError(function (arg0, arg1) {
            const ret = arg1.code;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_contentBoxSize_328a5cd3e7d063a9: function() { return logError(function (arg0) {
            const ret = arg0.contentBoxSize;
            return ret;
        }, arguments); },
        __wbg_createComment_b783f49934771bb3: function() { return logError(function (arg0, arg1, arg2) {
            const ret = arg0.createComment(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_createElementNS_ee00621496b30ec2: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = arg0.createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            return ret;
        }, arguments); },
        __wbg_createElement_49f60fdcaae809c8: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_createTextNode_55029686c9591bf3: function() { return logError(function (arg0, arg1, arg2) {
            const ret = arg0.createTextNode(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_ctrlKey_09a1b54d77dea92b: function() { return logError(function (arg0) {
            const ret = arg0.ctrlKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_ctrlKey_77fdaa123d7baf11: function() { return logError(function (arg0) {
            const ret = arg0.ctrlKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_ctrlKey_96ff94f8b18636a3: function() { return logError(function (arg0) {
            const ret = arg0.ctrlKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_dataTransfer_d924a622fbe51b06: function() { return logError(function (arg0) {
            const ret = arg0.dataTransfer;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_detail_934cf9545e3214d3: function() { return logError(function (arg0) {
            const ret = arg0.detail;
            return ret;
        }, arguments); },
        __wbg_document_ee35a3d3ae34ef6c: function() { return logError(function (arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_done_57b39ecd9addfe81: function() { return logError(function (arg0) {
            const ret = arg0.done;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_dropEffect_05a85bb8960b2b1e: function() { return logError(function (arg0, arg1) {
            const ret = arg1.dropEffect;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_effectAllowed_dbb296a6b7beabdd: function() { return logError(function (arg0, arg1) {
            const ret = arg1.effectAllowed;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_elapsedTime_92a4d0b90692bc49: function() { return logError(function (arg0) {
            const ret = arg0.elapsedTime;
            return ret;
        }, arguments); },
        __wbg_entries_58c7934c745daac7: function() { return logError(function (arg0) {
            const ret = Object.entries(arg0);
            return ret;
        }, arguments); },
        __wbg_entries_9203de2bd0a73594: function() { return logError(function (arg0) {
            const ret = arg0.entries();
            return ret;
        }, arguments); },
        __wbg_error_9a7fe3f932034cde: function() { return logError(function (arg0) {
            console.error(arg0);
        }, arguments); },
        __wbg_files_c7608e3fb8eb4d07: function() { return logError(function (arg0) {
            const ret = arg0.files;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_files_f9461f097760ef70: function() { return logError(function (arg0) {
            const ret = arg0.files;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_focus_128ff465f65677cc: function() { return handleError(function (arg0) {
            arg0.focus();
        }, arguments); },
        __wbg_force_6acda126382fc3c0: function() { return logError(function (arg0) {
            const ret = arg0.force;
            return ret;
        }, arguments); },
        __wbg_forward_fca5f9e344694a25: function() { return handleError(function (arg0) {
            arg0.forward();
        }, arguments); },
        __wbg_getAsFile_4272646c8ea66c0e: function() { return handleError(function (arg0) {
            const ret = arg0.getAsFile();
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_getAttribute_b9f6fc4b689c71b0: function() { return logError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.getAttribute(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_getBoundingClientRect_b5c8c34d07878818: function() { return logError(function (arg0) {
            const ret = arg0.getBoundingClientRect();
            return ret;
        }, arguments); },
        __wbg_getData_2aada4ab05d445e3: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.getData(getStringFromWasm0(arg2, arg3));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_getElementById_e34377b79d7285f6: function() { return logError(function (arg0, arg1, arg2) {
            const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_getNode_6cf64333dbfdbc5f: function() { return logError(function (arg0, arg1) {
            const ret = arg0.getNode(arg1 >>> 0);
            return ret;
        }, arguments); },
        __wbg_getTime_1e3cd1391c5c3995: function() { return logError(function (arg0) {
            const ret = arg0.getTime();
            return ret;
        }, arguments); },
        __wbg_getTimezoneOffset_81776d10a4ec18a8: function() { return logError(function (arg0) {
            const ret = arg0.getTimezoneOffset();
            return ret;
        }, arguments); },
        __wbg_get_4fe487fe39ff3573: function() { return logError(function (arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_get_5bd55a138a9e899f: function() { return logError(function (arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_get_9b94d73e6221f75c: function() { return logError(function (arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        }, arguments); },
        __wbg_get_b3ed3ad4be2bc8ac: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_select_data_884dda393040a62c: function() { return logError(function (arg0, arg1) {
            const ret = get_select_data(arg1);
            const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_hash_90eadad0e1447454: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.hash;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_head_a64c2648b30c3faf: function() { return logError(function (arg0) {
            const ret = arg0.head;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_height_3a1c04ec1a64cd16: function() { return logError(function (arg0) {
            const ret = arg0.height;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_height_45209601b4c4ede6: function() { return logError(function (arg0) {
            const ret = arg0.height;
            return ret;
        }, arguments); },
        __wbg_history_d6c2f9abc6e74880: function() { return handleError(function (arg0) {
            const ret = arg0.history;
            return ret;
        }, arguments); },
        __wbg_identifier_5feaba602edf9981: function() { return logError(function (arg0) {
            const ret = arg0.identifier;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_initialize_351c50f5d75de88c: function() { return logError(function (arg0, arg1, arg2) {
            arg0.initialize(arg1, arg2);
        }, arguments); },
        __wbg_inlineSize_3e4e7e8c813884fd: function() { return logError(function (arg0) {
            const ret = arg0.inlineSize;
            return ret;
        }, arguments); },
        __wbg_instanceof_ArrayBuffer_c367199e2fa2aa04: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Blob_ce92a9ddd729a84a: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Blob;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_DragEvent_4081e31b31f96acf: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof DragEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Element_9e662f49ab6c6beb: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Element;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_File_21240124aa87092d: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof File;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_HtmlElement_5abfac207260fd6f: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_HtmlFormElement_e47560673650ec20: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLFormElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_HtmlInputElement_c10b7260b4e0710a: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLInputElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_HtmlSelectElement_a1820832b22b0d1e: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLSelectElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_HtmlTextAreaElement_d17ace23f20e2338: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLTextAreaElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Map_53af74335dec57f4: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Map;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Node_da04bd8df43deba3: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Node;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Uint8Array_9b9075935c74707c: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_instanceof_Window_ed49b2db8df90359: function() { return logError(function (arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_isArray_d314bb98fcf08331: function() { return logError(function (arg0) {
            const ret = Array.isArray(arg0);
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_isComposing_9323fa62320f5fc0: function() { return logError(function (arg0) {
            const ret = arg0.isComposing;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_isPrimary_c2c28faa2ad84144: function() { return logError(function (arg0) {
            const ret = arg0.isPrimary;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_isSafeInteger_bfbc7332a9768d2a: function() { return logError(function (arg0) {
            const ret = Number.isSafeInteger(arg0);
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_item_c79c0bccbcfd8735: function() { return logError(function (arg0, arg1) {
            const ret = arg0.item(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_items_4130211600bde9a4: function() { return logError(function (arg0) {
            const ret = arg0.items;
            return ret;
        }, arguments); },
        __wbg_iterator_6ff6560ca1568e55: function() { return logError(function () {
            const ret = Symbol.iterator;
            return ret;
        }, arguments); },
        __wbg_key_d41e8e825e6bb0e9: function() { return logError(function (arg0, arg1) {
            const ret = arg1.key;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_kind_07735b12745b6ff3: function() { return logError(function (arg0, arg1) {
            const ret = arg1.kind;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_lastModified_a5cfce993c651681: function() { return logError(function (arg0) {
            const ret = arg0.lastModified;
            return ret;
        }, arguments); },
        __wbg_left_3b7c3c1030d5ca7a: function() { return logError(function (arg0) {
            const ret = arg0.left;
            return ret;
        }, arguments); },
        __wbg_length_25b2ccd77d48ecb1: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_length_32ed9a279acd054c: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_length_35a7bace40f36eac: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_length_68dc7c5cf1b6d349: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_length_9efde69e99cd464e: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_length_dd7a84decbd9cde7: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_location_22bcb1a188a96eb1: function() { return logError(function (arg0) {
            const ret = arg0.location;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_location_df7ca06c93e51763: function() { return logError(function (arg0) {
            const ret = arg0.location;
            return ret;
        }, arguments); },
        __wbg_log_0cc1b7768397bcfe: function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.log(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5), getStringFromWasm0(arg6, arg7));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        }, arguments); },
        __wbg_log_cb9e190acc5753fb: function() { return logError(function (arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.log(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        }, arguments); },
        __wbg_mark_7438147ce31e9d4b: function() { return logError(function (arg0, arg1) {
            performance.mark(getStringFromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_measure_fb7825c11612c823: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            let deferred0_0;
            let deferred0_1;
            let deferred1_0;
            let deferred1_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                deferred1_0 = arg2;
                deferred1_1 = arg3;
                performance.measure(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
                wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
            }
        }, arguments); },
        __wbg_metaKey_374999c340f70626: function() { return logError(function (arg0) {
            const ret = arg0.metaKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_metaKey_67113fb40365d736: function() { return logError(function (arg0) {
            const ret = arg0.metaKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_metaKey_6c0be48717d1799a: function() { return logError(function (arg0) {
            const ret = arg0.metaKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_name_171cddfde96a29c8: function() { return logError(function (arg0, arg1) {
            const ret = arg1.name;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_name_7f2f048a96cd9b6b: function() { return logError(function (arg0, arg1) {
            const ret = arg1.name;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_new_0_73afc35eb544e539: function() { return logError(function () {
            const ret = new Date();
            return ret;
        }, arguments); },
        __wbg_new_240a5a71b44014d1: function() { return handleError(function () {
            const ret = new DataTransfer();
            return ret;
        }, arguments); },
        __wbg_new_245cd5c49157e602: function() { return logError(function (arg0) {
            const ret = new Date(arg0);
            return ret;
        }, arguments); },
        __wbg_new_3134e09dc0fdb3aa: function() { return logError(function (arg0) {
            const ret = new RawInterpreter(arg0 >>> 0);
            return ret;
        }, arguments); },
        __wbg_new_361308b2356cecd0: function() { return logError(function () {
            const ret = new Object();
            return ret;
        }, arguments); },
        __wbg_new_39d59074c3674711: function() { return logError(function (arg0) {
            const ret = new WebDioxusChannel(JSOwner.__wrap(arg0));
            return ret;
        }, arguments); },
        __wbg_new_3eb36ae241fe6f44: function() { return logError(function () {
            const ret = new Array();
            return ret;
        }, arguments); },
        __wbg_new_c1eaab32d813ec69: function() { return handleError(function () {
            const ret = new FileReader();
            return ret;
        }, arguments); },
        __wbg_new_dca287b076112a51: function() { return logError(function () {
            const ret = new Map();
            return ret;
        }, arguments); },
        __wbg_new_dd2b680c8bf6ae29: function() { return logError(function (arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        }, arguments); },
        __wbg_new_no_args_1c7c842f08d00ebb: function() { return logError(function (arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_new_with_args_7bba34e94b1cfa40: function() { return logError(function (arg0, arg1, arg2, arg3) {
            const ret = new Function(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return ret;
        }, arguments); },
        __wbg_new_with_form_fc8d981754af7c47: function() { return handleError(function (arg0) {
            const ret = new FormData(arg0);
            return ret;
        }, arguments); },
        __wbg_next_3482f54c49e8af19: function() { return handleError(function (arg0) {
            const ret = arg0.next();
            return ret;
        }, arguments); },
        __wbg_next_418f80d8f5303233: function() { return logError(function (arg0) {
            const ret = arg0.next;
            return ret;
        }, arguments); },
        __wbg_offsetX_b76b9bb1f9235de9: function() { return logError(function (arg0) {
            const ret = arg0.offsetX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_offsetY_db5c1ddb866e1b82: function() { return logError(function (arg0) {
            const ret = arg0.offsetY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_ownerDocument_9347874c5cad87d7: function() { return logError(function (arg0) {
            const ret = arg0.ownerDocument;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_pageX_18c33c0de79ad555: function() { return logError(function (arg0) {
            const ret = arg0.pageX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_pageX_e236113b39fdb245: function() { return logError(function (arg0) {
            const ret = arg0.pageX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_pageY_5653bbc6f8a6f28d: function() { return logError(function (arg0) {
            const ret = arg0.pageY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_pageY_edbfa2d153a63704: function() { return logError(function (arg0) {
            const ret = arg0.pageY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_parentElement_75863410a8617953: function() { return logError(function (arg0) {
            const ret = arg0.parentElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_pathname_b2267fae358b49a7: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.pathname;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_pointerId_466b1bdcaf2fe835: function() { return logError(function (arg0) {
            const ret = arg0.pointerId;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_pointerType_ba53c6f18634a26d: function() { return logError(function (arg0, arg1) {
            const ret = arg1.pointerType;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_pressure_f01a99684f7a6cf3: function() { return logError(function (arg0) {
            const ret = arg0.pressure;
            return ret;
        }, arguments); },
        __wbg_preventDefault_cdcfcd7e301b9702: function() { return logError(function (arg0) {
            arg0.preventDefault();
        }, arguments); },
        __wbg_prototypesetcall_bdcdcc5842e4d77d: function() { return logError(function (arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        }, arguments); },
        __wbg_pseudoElement_826672ee2463e181: function() { return logError(function (arg0, arg1) {
            const ret = arg1.pseudoElement;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_pushState_b8b21dc05883695b: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.pushState(arg1, getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        }, arguments); },
        __wbg_push_8ffdcb2063340ba5: function() { return logError(function (arg0, arg1) {
            const ret = arg0.push(arg1);
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_queueMicrotask_0aa0a927f78f5d98: function() { return logError(function (arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        }, arguments); },
        __wbg_queueMicrotask_5bb536982f78a56f: function() { return logError(function (arg0) {
            queueMicrotask(arg0);
        }, arguments); },
        __wbg_radiusX_94859a491e8fcf6b: function() { return logError(function (arg0) {
            const ret = arg0.radiusX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_radiusY_fe1362e5f41bb351: function() { return logError(function (arg0) {
            const ret = arg0.radiusY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_readAsArrayBuffer_7dda4bab363b1552: function() { return handleError(function (arg0, arg1) {
            arg0.readAsArrayBuffer(arg1);
        }, arguments); },
        __wbg_readAsText_6be5d99a0de05eea: function() { return handleError(function (arg0, arg1) {
            arg0.readAsText(arg1);
        }, arguments); },
        __wbg_repeat_375aae5c5c6a0258: function() { return logError(function (arg0) {
            const ret = arg0.repeat;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_replaceState_aefa111958f68023: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.replaceState(arg1, getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        }, arguments); },
        __wbg_requestAnimationFrame_43682f8e1c5e5348: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.requestAnimationFrame(arg1);
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_resolve_002c4b7d9d8f6b64: function() { return logError(function (arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        }, arguments); },
        __wbg_result_a9f41cf43ff6e60f: function() { return handleError(function (arg0) {
            const ret = arg0.result;
            return ret;
        }, arguments); },
        __wbg_rotationAngle_89bafc842c5a13df: function() { return logError(function (arg0) {
            const ret = arg0.rotationAngle;
            return ret;
        }, arguments); },
        __wbg_run_f38ff56693c2434a: function() { return logError(function (arg0) {
            arg0.run();
        }, arguments); },
        __wbg_rustRecv_fad8746b24af4505: function() { return logError(function (arg0) {
            const ret = arg0.rustRecv();
            return ret;
        }, arguments); },
        __wbg_rustSend_cbcc25aeb21455da: function() { return logError(function (arg0, arg1) {
            arg0.rustSend(arg1);
        }, arguments); },
        __wbg_saveTemplate_c93ddeee7646b26c: function() { return logError(function (arg0, arg1, arg2, arg3) {
            var v0 = getArrayJsValueFromWasm0(arg1, arg2).slice();
            wasm.__wbindgen_free(arg1, arg2 * 4, 4);
            arg0.saveTemplate(v0, arg3);
        }, arguments); },
        __wbg_screenX_83378b3fced6f5a3: function() { return logError(function (arg0) {
            const ret = arg0.screenX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_screenX_e545a74d85d1d3dc: function() { return logError(function (arg0) {
            const ret = arg0.screenX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_screenY_1018dce6cdd419f4: function() { return logError(function (arg0) {
            const ret = arg0.screenY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_screenY_6a069f189d2f4a85: function() { return logError(function (arg0) {
            const ret = arg0.screenY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_scrollHeight_25e40881bfad55b6: function() { return logError(function (arg0) {
            const ret = arg0.scrollHeight;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_scrollIntoView_10646525aff3911a: function() { return logError(function (arg0, arg1) {
            arg0.scrollIntoView(arg1);
        }, arguments); },
        __wbg_scrollLeft_2b817c7719d17438: function() { return logError(function (arg0) {
            const ret = arg0.scrollLeft;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_scrollTo_fc2ddb2becb11097: function() { return logError(function (arg0, arg1, arg2) {
            arg0.scrollTo(arg1, arg2);
        }, arguments); },
        __wbg_scrollTop_4161d2a08060cb06: function() { return logError(function (arg0) {
            const ret = arg0.scrollTop;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_scrollWidth_d0233d345e3a50fa: function() { return logError(function (arg0) {
            const ret = arg0.scrollWidth;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_scrollX_b3151cb810a681ae: function() { return handleError(function (arg0) {
            const ret = arg0.scrollX;
            return ret;
        }, arguments); },
        __wbg_scrollY_8087997adf618f94: function() { return handleError(function (arg0) {
            const ret = arg0.scrollY;
            return ret;
        }, arguments); },
        __wbg_scroll_ecaf2e60aedabdeb: function() { return logError(function (arg0, arg1) {
            arg0.scroll(arg1);
        }, arguments); },
        __wbg_search_1b385e665c888780: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.search;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_setAttributeInner_964ef6e4f191e558: function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            setAttributeInner(arg0, getStringFromWasm0(arg1, arg2), arg3, arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        }, arguments); },
        __wbg_setAttribute_cc8e4c8a2a008508: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setData_c6fb07ac0e13293c: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setData(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setTimeout_db2dbaeefb6f39c7: function() { return handleError(function (arg0, arg1) {
            const ret = setTimeout(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_set_1eb0999cf5d27fc8: function() { return logError(function (arg0, arg1, arg2) {
            const ret = arg0.set(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_set_3f1d0b984ed272ed: function() { return logError(function (arg0, arg1, arg2) {
            arg0[arg1] = arg2;
        }, arguments); },
        __wbg_set_behavior_95b5c7eaefc26d7f: function() { return logError(function (arg0, arg1) {
            arg0.behavior = __wbindgen_enum_ScrollBehavior[arg1];
        }, arguments); },
        __wbg_set_behavior_995c72a459162385: function() { return logError(function (arg0, arg1) {
            arg0.behavior = __wbindgen_enum_ScrollBehavior[arg1];
        }, arguments); },
        __wbg_set_block_607a9575144934d2: function() { return logError(function (arg0, arg1) {
            arg0.block = __wbindgen_enum_ScrollLogicalPosition[arg1];
        }, arguments); },
        __wbg_set_dropEffect_042402609f0b7c6b: function() { return logError(function (arg0, arg1, arg2) {
            arg0.dropEffect = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_effectAllowed_0f5f8cdbd1be75e1: function() { return logError(function (arg0, arg1, arg2) {
            arg0.effectAllowed = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_f43e577aea94465b: function() { return logError(function (arg0, arg1, arg2) {
            arg0[arg1 >>> 0] = arg2;
        }, arguments); },
        __wbg_set_href_a9f78663078e72dc: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.href = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_inline_1644a32851f84ca6: function() { return logError(function (arg0, arg1) {
            arg0.inline = __wbindgen_enum_ScrollLogicalPosition[arg1];
        }, arguments); },
        __wbg_set_left_4869860d9e8e2130: function() { return logError(function (arg0, arg1) {
            arg0.left = arg1;
        }, arguments); },
        __wbg_set_onload_0ae5444d94882a19: function() { return logError(function (arg0, arg1) {
            arg0.onload = arg1;
        }, arguments); },
        __wbg_set_scrollRestoration_e8b6ea3675c217c1: function() { return handleError(function (arg0, arg1) {
            arg0.scrollRestoration = __wbindgen_enum_ScrollRestoration[arg1];
        }, arguments); },
        __wbg_set_textContent_3e87dba095d9cdbc: function() { return logError(function (arg0, arg1, arg2) {
            arg0.textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_top_2f635c4c8aa672e1: function() { return logError(function (arg0, arg1) {
            arg0.top = arg1;
        }, arguments); },
        __wbg_shiftKey_0439fb94db9ea71d: function() { return logError(function (arg0) {
            const ret = arg0.shiftKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_shiftKey_5558a3288542c985: function() { return logError(function (arg0) {
            const ret = arg0.shiftKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_shiftKey_564be91ec842bcc4: function() { return logError(function (arg0) {
            const ret = arg0.shiftKey;
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_size_e05d31cc6049815f: function() { return logError(function (arg0) {
            const ret = arg0.size;
            return ret;
        }, arguments); },
        __wbg_state_da286469d44b98d6: function() { return handleError(function (arg0) {
            const ret = arg0.state;
            return ret;
        }, arguments); },
        __wbg_static_accessor_GLOBAL_12837167ad935116: function() { return logError(function () {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f: function() { return logError(function () {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_SELF_a621d3dfbb60d0ce: function() { return logError(function () {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_static_accessor_WINDOW_f8727f0cf888e0bd: function() { return logError(function () {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_stringify_8d1cc6ff383e8bae: function() { return handleError(function (arg0) {
            const ret = JSON.stringify(arg0);
            return ret;
        }, arguments); },
        __wbg_tangentialPressure_1acf5dd7e72ac65c: function() { return logError(function (arg0) {
            const ret = arg0.tangentialPressure;
            return ret;
        }, arguments); },
        __wbg_targetTouches_e57e9e322146d699: function() { return logError(function (arg0) {
            const ret = arg0.targetTouches;
            return ret;
        }, arguments); },
        __wbg_target_521be630ab05b11e: function() { return logError(function (arg0) {
            const ret = arg0.target;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_textContent_fc823fb432e90037: function() { return logError(function (arg0, arg1) {
            const ret = arg1.textContent;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_then_0d9fe2c7b1857d32: function() { return logError(function (arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_then_b9e7b3b5f1a9e1b5: function() { return logError(function (arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        }, arguments); },
        __wbg_tiltX_23512bbdf748632c: function() { return logError(function (arg0) {
            const ret = arg0.tiltX;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_tiltY_220e7dce76b43f2a: function() { return logError(function (arg0) {
            const ret = arg0.tiltY;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_top_3d27ff6f468cf3fc: function() { return logError(function (arg0) {
            const ret = arg0.top;
            return ret;
        }, arguments); },
        __wbg_touches_55ce167b42bcdf52: function() { return logError(function (arg0) {
            const ret = arg0.touches;
            return ret;
        }, arguments); },
        __wbg_twist_5f51f4e973f83a8f: function() { return logError(function (arg0) {
            const ret = arg0.twist;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_type_3d9620e2fdd4df68: function() { return logError(function (arg0, arg1) {
            const ret = arg1.type;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_type_4edffca24c42b74d: function() { return logError(function (arg0, arg1) {
            const ret = arg1.type;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_type_e8c7fade6d73451b: function() { return logError(function (arg0, arg1) {
            const ret = arg1.type;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_update_memory_6f98ff33983e5901: function() { return logError(function (arg0, arg1) {
            arg0.update_memory(arg1);
        }, arguments); },
        __wbg_value_0546255b415e96c1: function() { return logError(function (arg0) {
            const ret = arg0.value;
            return ret;
        }, arguments); },
        __wbg_value_15684899da869c95: function() { return logError(function (arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_value_d402dce7dcb16251: function() { return logError(function (arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_value_e506a07878790ca0: function() { return logError(function (arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_weak_3a206f1c49e42f9e: function() { return logError(function (arg0) {
            const ret = arg0.weak();
            return ret;
        }, arguments); },
        __wbg_width_ae46cb8e98ee102f: function() { return logError(function (arg0) {
            const ret = arg0.width;
            return ret;
        }, arguments); },
        __wbg_width_ed40c9f333808b22: function() { return logError(function (arg0) {
            const ret = arg0.width;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000001: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 1104, function: Function { arguments: [NamedExternref("Event")], shim_idx: 1109, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm._ZN12wasm_bindgen7closure7destroy17h47703d66d189193eE, _ZN12wasm_bindgen7convert8closures1_6invoke17h810d8298235d48b1E);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000002: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 1104, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 1107, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, wasm._ZN12wasm_bindgen7closure7destroy17h47703d66d189193eE, _ZN12wasm_bindgen7convert8closures1_1_6invoke17h57158d2efc5b6de5E);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000003: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 1104, function: Function { arguments: [], shim_idx: 1105, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm._ZN12wasm_bindgen7closure7destroy17h47703d66d189193eE, _ZN12wasm_bindgen7convert8closures1_6invoke17h7dfc4b853d258b85E);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000004: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 1148, function: Function { arguments: [Externref], shim_idx: 1109, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm._ZN12wasm_bindgen7closure7destroy17h9bc4d8f864563b75E, _ZN12wasm_bindgen7convert8closures1_6invoke17h810d8298235d48b1E);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000005: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { dtor_idx: 1148, function: Function { arguments: [], shim_idx: 1105, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm._ZN12wasm_bindgen7closure7destroy17h9bc4d8f864563b75E, _ZN12wasm_bindgen7convert8closures1_6invoke17h7dfc4b853d258b85E);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000006: function() { return logError(function (arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000007: function() { return logError(function (arg0) {
            // Cast intrinsic for `I64 -> Externref`.
            const ret = arg0;
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000008: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000009: function() { return logError(function (arg0) {
            // Cast intrinsic for `U64 -> Externref`.
            const ret = BigInt.asUintN(64, arg0);
            return ret;
        }, arguments); },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./preview_bg.js": import0,
        "./__wasm_split.js": import1,
        "./__wasm_split.js": import2,
        "./__wasm_split.js": import3,
        "./__wasm_split.js": import4,
        "./__wasm_split.js": import5,
        "./__wasm_split.js": import6,
        "./__wasm_split.js": import7,
        "./__wasm_split.js": import8,
        "./__wasm_split.js": import9,
        "./__wasm_split.js": import10,
        "./__wasm_split.js": import11,
        "./__wasm_split.js": import12,
        "./__wasm_split.js": import13,
        "./__wasm_split.js": import14,
        "./__wasm_split.js": import15,
        "./__wasm_split.js": import16,
        "./__wasm_split.js": import17,
        "./__wasm_split.js": import18,
        "./__wasm_split.js": import19,
        "./__wasm_split.js": import20,
        "./__wasm_split.js": import21,
        "./__wasm_split.js": import22,
        "./__wasm_split.js": import23,
        "./__wasm_split.js": import24,
        "./__wasm_split.js": import25,
        "./__wasm_split.js": import26,
        "./__wasm_split.js": import27,
        "./__wasm_split.js": import28,
        "./__wasm_split.js": import29,
        "./__wasm_split.js": import30,
        "./__wasm_split.js": import31,
        "./__wasm_split.js": import32,
        "./__wasm_split.js": import33,
        "./__wasm_split.js": import34,
        "./__wasm_split.js": import35,
        "./__wasm_split.js": import36,
        "./__wasm_split.js": import37,
        "./__wasm_split.js": import38,
        "./__wasm_split.js": import39,
        "./__wasm_split.js": import40,
        "./__wasm_split.js": import41,
        "./__wasm_split.js": import42,
        "./__wasm_split.js": import43,
        "./__wasm_split.js": import44,
        "./__wasm_split.js": import45,
        "./__wasm_split.js": import46,
        "./__wasm_split.js": import47,
        "./__wasm_split.js": import48,
        "./__wasm_split.js": import49,
        "./__wasm_split.js": import50,
        "./__wasm_split.js": import51,
        "./__wasm_split.js": import52,
        "./__wasm_split.js": import53,
        "./__wasm_split.js": import54,
        "./__wasm_split.js": import55,
        "./__wasm_split.js": import56,
        "./__wasm_split.js": import57,
        "./__wasm_split.js": import58,
        "./__wasm_split.js": import59,
        "./__wasm_split.js": import60,
        "./__wasm_split.js": import61,
        "./__wasm_split.js": import62,
        "./__wasm_split.js": import63,
        "./__wasm_split.js": import64,
        "./__wasm_split.js": import65,
        "./__wasm_split.js": import66,
        "./__wasm_split.js": import67,
        "./__wasm_split.js": import68,
        "./__wasm_split.js": import69,
        "./__wasm_split.js": import70,
        "./__wasm_split.js": import71,
        "./__wasm_split.js": import72,
        "./__wasm_split.js": import73,
        "./__wasm_split.js": import74,
        "./__wasm_split.js": import75,
        "./__wasm_split.js": import76,
        "./__wasm_split.js": import77,
        "./__wasm_split.js": import78,
        "./__wasm_split.js": import79,
        "./__wasm_split.js": import80,
        "./__wasm_split.js": import81,
        "./__wasm_split.js": import82,
        "./__wasm_split.js": import83,
        "./__wasm_split.js": import84,
        "./__wasm_split.js": import85,
        "./__wasm_split.js": import86,
        "./__wasm_split.js": import87,
        "./__wasm_split.js": import88,
        "./__wasm_split.js": import89,
        "./__wasm_split.js": import90,
        "./__wasm_split.js": import91,
        "./__wasm_split.js": import92,
        "./__wasm_split.js": import93,
        "./__wasm_split.js": import94,
        "./__wasm_split.js": import95,
        "./__wasm_split.js": import96,
        "./__wasm_split.js": import97,
        "./__wasm_split.js": import98,
    };
}


//#endregion
function _ZN12wasm_bindgen7convert8closures1_6invoke17h7dfc4b853d258b85E(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._ZN12wasm_bindgen7convert8closures1_6invoke17h7dfc4b853d258b85E(arg0, arg1);
}

function _ZN12wasm_bindgen7convert8closures1_6invoke17h810d8298235d48b1E(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._ZN12wasm_bindgen7convert8closures1_6invoke17h810d8298235d48b1E(arg0, arg1, arg2);
}

function _ZN12wasm_bindgen7convert8closures1_1_6invoke17h57158d2efc5b6de5E(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._ZN12wasm_bindgen7convert8closures1_1_6invoke17h57158d2efc5b6de5E(arg0, arg1, arg2);
}


const __wbindgen_enum_ScrollBehavior = ["auto", "instant", "smooth"];


const __wbindgen_enum_ScrollLogicalPosition = ["start", "center", "end", "nearest"];


const __wbindgen_enum_ScrollRestoration = ["auto", "manual"];
const JSOwnerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsowner_free(ptr >>> 0, 1));


//#region intrinsics
function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertBigInt(n) {
    if (typeof(n) !== 'bigint') throw new Error(`expected a bigint argument, found ${typeof(n)}`);
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error(`expected a boolean argument, found ${typeof(n)}`);
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


//#endregion

//#region wasm loading
let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('preview_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
//#endregion
export { wasm as __wasm }
/*0c12e96a-b6d6-51b1-b05c-08e472f13b95*/
globalThis.__wasm_split_main_initSync = initSync;

// Actually perform the load
__wbg_init({module_or_path: "/dx-components/wasm/preview_bg.wasm"}).then((wasm) => {
    // assign this module to be accessible globally
    globalThis.__dx_mainWasm = wasm;
    globalThis.__dx_mainInit = __wbg_init;
    globalThis.__dx_mainInitSync = initSync;
    globalThis.__dx___wbg_get_imports = __wbg_get_imports;

    if (wasm.__wbindgen_start == undefined) {
        wasm.main();
    }
});

