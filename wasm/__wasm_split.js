// when running the harness we need to make sure to uncommon this out...

export function makeLoad(url, deps, fusedImports, initIt) {
  let alreadyLoaded = false;
  return async (callbackIndex, callbackData) => {
    await Promise.all(deps.map((dep) => dep()));
    if (alreadyLoaded) return;
    try {
      const response = await fetch(url);
      const initSync = initIt || globalThis.__wasm_split_main_initSync;
      const mainExports = initSync(undefined, undefined);

      let imports = {
        env: {
          memory: mainExports.memory,
        },
        __wasm_split: {
          __indirect_function_table: mainExports.__indirect_function_table,
          __stack_pointer: mainExports.__stack_pointer,
          __tls_base: mainExports.__tls_base,
          memory: mainExports.memory,
        },
      };

      for (let mainExport in mainExports) {
        imports["__wasm_split"][mainExport] = mainExports[mainExport];
      }

      for (let name in fusedImports) {
        imports["__wasm_split"][name] = fusedImports[name];
      }

      let new_exports = await WebAssembly.instantiateStreaming(
        response,
        imports
      );

      alreadyLoaded = true;

      for (let name in new_exports.instance.exports) {
        fusedImports[name] = new_exports.instance.exports[name];
      }

      if (callbackIndex !== undefined) {
        mainExports.__indirect_function_table.get(callbackIndex)(
          callbackData,
          true
        );
      }
    } catch (e) {
      console.error(
        "Failed to load wasm-split module",
        e,
        url,
        deps,
        fusedImports
      );
      return;
    }
  };
}

let fusedImports = {};
export const __wasm_split_load_chunk_0 = makeLoad("/dx-components/assets/chunk_0_split-dxh8ecc98c21d9bd1e.wasm", [], fusedImports);
export const __wasm_split_load_moduleAccordionPage4f0015d841e03d0796fe605aebac200b_0a4d71d09f3346469af01b77a1c6a0f2_routeAccordionPage4f0015d841e03d0796fe605aebac200b = makeLoad("/dx-components/assets/module_0_routeAccordionPage4f0015d841e03d0796fe605aebac200b-dxha491113e454c4d9.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPagead7800b5ffe0209af035b9004d16c570_a9e70882a94bbe35b9bf50683b85b1d3_routeAlertDialogPagead7800b5ffe0209af035b9004d16c570 = makeLoad("/dx-components/assets/module_1_routeAlertDialogPagead7800b5ffe0209af035b9004d16c570-dxh5053e4e767765995.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPage67ec3a61f4add748c9884fae9b65d813_5874da3476f083bd4b4dfedad8ce9e76_routeAspectRatioPage67ec3a61f4add748c9884fae9b65d813 = makeLoad("/dx-components/assets/module_2_routeAspectRatioPage67ec3a61f4add748c9884fae9b65d813-dxh280752d4dc87ed.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPagecb8c2257b9599d982d352bd4b7de1d8c_c046802878ba720a7e306bf7365a9ca0_routeAvatarPagecb8c2257b9599d982d352bd4b7de1d8c = makeLoad("/dx-components/assets/module_3_routeAvatarPagecb8c2257b9599d982d352bd4b7de1d8c-dxh8ffa66ec8b257b9b.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePagef33d34ecf7a2a07aae7055d1ae289d93_055818a21bb449103441424a1914774d_routeBadgePagef33d34ecf7a2a07aae7055d1ae289d93 = makeLoad("/dx-components/assets/module_4_routeBadgePagef33d34ecf7a2a07aae7055d1ae289d93-dxh8d6fb2df14647d1d.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPagef6fee2e34a09b6549e9ce4d59857eb89_6a72ee0693be0b90bd009c0af5b14279_routeButtonPagef6fee2e34a09b6549e9ce4d59857eb89 = makeLoad("/dx-components/assets/module_5_routeButtonPagef6fee2e34a09b6549e9ce4d59857eb89-dxh86197c26851faf20.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPaged6b3d85ab3add9b47e3d0dd865b67ba3_6346c109fc70f8df2a6ed23ca93aff2d_routeCalendarPaged6b3d85ab3add9b47e3d0dd865b67ba3 = makeLoad("/dx-components/assets/module_6_routeCalendarPaged6b3d85ab3add9b47e3d0dd865b67ba3-dxhaf44f335f231a025.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPage5cef7de0232671eb9d76ee88e85e62c9_addb1fb30f7ac045ebdf4915a2487209_routeCardPage5cef7de0232671eb9d76ee88e85e62c9 = makeLoad("/dx-components/assets/module_7_routeCardPage5cef7de0232671eb9d76ee88e85e62c9-dxh3de45179ac54e138.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPagea09a5608d1d0aceaa6f1b2a3606a998b_a20ffb9422da7261c97950d1c8defe79_routeCarouselPagea09a5608d1d0aceaa6f1b2a3606a998b = makeLoad("/dx-components/assets/module_8_routeCarouselPagea09a5608d1d0aceaa6f1b2a3606a998b-dxh8e7dc9dcbad3b.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPage28098e9abc49d1c45a2b4e8ad632fe1f_719bb43bb07062ebcd1b6806cd1811da_routeCheckboxPage28098e9abc49d1c45a2b4e8ad632fe1f = makeLoad("/dx-components/assets/module_9_routeCheckboxPage28098e9abc49d1c45a2b4e8ad632fe1f-dxhb5dae23cb8c4ef48.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePage5354357a0437210171bae010542c53ee_7173d2cfced58ecd5743e23660f25244_routeCollapsiblePage5354357a0437210171bae010542c53ee = makeLoad("/dx-components/assets/module_10_routeCollapsiblePage5354357a0437210171bae010542c53ee-dxhb92c1ac3a9d91b23.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPagebc46c7db205bdd56187614c0525b4838_259667db34fff69b02f6d2e765afa69f_routeComboboxPagebc46c7db205bdd56187614c0525b4838 = makeLoad("/dx-components/assets/module_11_routeComboboxPagebc46c7db205bdd56187614c0525b4838-dxhc78abd4280abe72e.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPage25d89d4bfadfe8a019f9cd9f1dd544eb_d832ad49d5fc43241dced464472ad575_routeCommandPage25d89d4bfadfe8a019f9cd9f1dd544eb = makeLoad("/dx-components/assets/module_12_routeCommandPage25d89d4bfadfe8a019f9cd9f1dd544eb-dxhf49d658af48a72d5.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemo07634d63e34a64fb50540f5a0f2bad1f_0c2e54199f170bf0c3b3f04a7ade7b59_routeComponentBlockDemo07634d63e34a64fb50540f5a0f2bad1f = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemo07634d63e34a64fb50540f5a0f2bad1f-dxh1edb5f1cd5647d5.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPageb603a46da53a2a72c05dcdbd52cc3787_c86cf12175b350ca652d1be0f645e02d_routeContextMenuPageb603a46da53a2a72c05dcdbd52cc3787 = makeLoad("/dx-components/assets/module_14_routeContextMenuPageb603a46da53a2a72c05dcdbd52cc3787-dxh7517bf1cd7941bf.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPageab31e987168c9a99952cfc4c5759ba93_1697468725eaf969f7b823c93ca260c5_routeDatePickerPageab31e987168c9a99952cfc4c5759ba93 = makeLoad("/dx-components/assets/module_15_routeDatePickerPageab31e987168c9a99952cfc4c5759ba93-dxh90a33b36f3c06cd6.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPage0067e630d9dc11d66b796e0b45384629_e2fb7a99d8ce8aa2df167d12e3e81d5c_routeDialogPage0067e630d9dc11d66b796e0b45384629 = makeLoad("/dx-components/assets/module_16_routeDialogPage0067e630d9dc11d66b796e0b45384629-dxhc43f5397521bec6b.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPage68e21bcbe8e04f4b9587e7599cde94ff_72273d9e5c707f9238b99578b70926fe_routeDragAndDropListPage68e21bcbe8e04f4b9587e7599cde94ff = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPage68e21bcbe8e04f4b9587e7599cde94ff-dxhcbea2c838dc3536.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPage2423bcf45bbe63506be02cd12a18c537_7ab3e60801262c9304d464bc1d65d73d_routeDrawerPage2423bcf45bbe63506be02cd12a18c537 = makeLoad("/dx-components/assets/module_18_routeDrawerPage2423bcf45bbe63506be02cd12a18c537-dxh4176af1847e7da44.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPaged4a351dcb3acd4a2e81ad658ff9a9ad2_1112f43ec30f403251efbd5a1f648e1a_routeDropdownMenuPaged4a351dcb3acd4a2e81ad658ff9a9ad2 = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPaged4a351dcb3acd4a2e81ad658ff9a9ad2-dxh47d653b5ab70fb34.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPage668fc0d0987114e8cb03fe95f1fde43c_f3ffb6fc183e558fe9b74a8ae5151769_routeFormPage668fc0d0987114e8cb03fe95f1fde43c = makeLoad("/dx-components/assets/module_20_routeFormPage668fc0d0987114e8cb03fe95f1fde43c-dxhe0ddbcb5bbb626c9.wasm", [], fusedImports);
export const __wasm_split_load_moduleHome49e954aa00b192482fb7c3c68222bdb8_b9b6a55ff42e411088918801ad447cb7_routeHome49e954aa00b192482fb7c3c68222bdb8 = makeLoad("/dx-components/assets/module_21_routeHome49e954aa00b192482fb7c3c68222bdb8-dxh5cbfc5aa3c6e2e9e.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPagecc0c4cbca2b189d5aa01dbdeafb5cf61_da10fc174f9d21c8c927b9187c970ef2_routeHoverCardPagecc0c4cbca2b189d5aa01dbdeafb5cf61 = makeLoad("/dx-components/assets/module_22_routeHoverCardPagecc0c4cbca2b189d5aa01dbdeafb5cf61-dxhaf62fbd91e77239c.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPage14b2c645f5469940929c5546a6bd099d_6535ca64d23b37db23325f03130acac5_routeInputOtpPage14b2c645f5469940929c5546a6bd099d = makeLoad("/dx-components/assets/module_23_routeInputOtpPage14b2c645f5469940929c5546a6bd099d-dxhfbe4b37c6e88e54b.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPage2131174688c48c1011c2bf6a72866575_bd7aec8f44875ba20afbd35cc0cc394b_routeInputPage2131174688c48c1011c2bf6a72866575 = makeLoad("/dx-components/assets/module_24_routeInputPage2131174688c48c1011c2bf6a72866575-dxh5eab482d1c140a1.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPage1b5d0edfc3ba67f6ba4c916885072258_7f0d715614c8626aa57b2d9d1ed91a7c_routeLabelPage1b5d0edfc3ba67f6ba4c916885072258 = makeLoad("/dx-components/assets/module_25_routeLabelPage1b5d0edfc3ba67f6ba4c916885072258-dxh704cd55d9d3042d3.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPageaf123836159562f1dedaec4e0e3c808e_597381e64a31b9e6ed8d32d1158915cc_routeMenubarPageaf123836159562f1dedaec4e0e3c808e = makeLoad("/dx-components/assets/module_26_routeMenubarPageaf123836159562f1dedaec4e0e3c808e-dxh116c7b3e770b46a.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPage4ebb5e5551f64eb1175204b2fef5a3c4_1128a3430c676ba6f65f1549acfa500c_routeNavbarPage4ebb5e5551f64eb1175204b2fef5a3c4 = makeLoad("/dx-components/assets/module_27_routeNavbarPage4ebb5e5551f64eb1175204b2fef5a3c4-dxha79b14b1d76e5fd.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPage58d7f4dd193e4fbfe5a02d7017ee4438_d48a8c492e30393b8409853c97f150a9_routeNavigationMenuPage58d7f4dd193e4fbfe5a02d7017ee4438 = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPage58d7f4dd193e4fbfe5a02d7017ee4438-dxhcf893d5fa0139731.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPage7b367b7f17fc3b001d9cd8fee00d0caf_599d854a339acc452e28270a35b300a0_routePaginationPage7b367b7f17fc3b001d9cd8fee00d0caf = makeLoad("/dx-components/assets/module_29_routePaginationPage7b367b7f17fc3b001d9cd8fee00d0caf-dxhb5f18b18ae074a4.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPaged98a4f8a6160c582bb52a36140772d5a_3f5db3dabf6a4cbb0c79a5ffe9bfeb55_routePopoverPaged98a4f8a6160c582bb52a36140772d5a = makeLoad("/dx-components/assets/module_30_routePopoverPaged98a4f8a6160c582bb52a36140772d5a-dxhf6dd753d4b173.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPagee0812121cce2ec50a68ee4ddc03d414a_52f71bde6fbca913ab2577466c847a45_routeProgressPagee0812121cce2ec50a68ee4ddc03d414a = makeLoad("/dx-components/assets/module_31_routeProgressPagee0812121cce2ec50a68ee4ddc03d414a-dxh422b173b4e8043c1.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPageb37d5743f612e56c3c82848492a5bed4_b3d9553091d0cfac894afef5271b6f6c_routeRadioGroupPageb37d5743f612e56c3c82848492a5bed4 = makeLoad("/dx-components/assets/module_32_routeRadioGroupPageb37d5743f612e56c3c82848492a5bed4-dxh8c70a12b97f4b9dc.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePageb494231755238feefcabb377f150d446_f3b8880a943073c2fd0ccc1a461ccd43_routeResizablePageb494231755238feefcabb377f150d446 = makeLoad("/dx-components/assets/module_33_routeResizablePageb494231755238feefcabb377f150d446-dxh2cf114b65de2d6b.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPage072e327c74bdfee661988efd685cb21d_f22baa6252e566180b37b3e831e9dacf_routeScrollAreaPage072e327c74bdfee661988efd685cb21d = makeLoad("/dx-components/assets/module_34_routeScrollAreaPage072e327c74bdfee661988efd685cb21d-dxhc22eeaaecbce6efa.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPage67fff64fe4119540b884d3f24cb60de7_d389469897512f6f43ef3e80e52fe1d3_routeSelectPage67fff64fe4119540b884d3f24cb60de7 = makeLoad("/dx-components/assets/module_35_routeSelectPage67fff64fe4119540b884d3f24cb60de7-dxh4417ca959d47a5fb.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPage4f0955ca7706f7007f9e6b7e7787667e_4563dfa1292b31807ba2111b745636c2_routeSeparatorPage4f0955ca7706f7007f9e6b7e7787667e = makeLoad("/dx-components/assets/module_36_routeSeparatorPage4f0955ca7706f7007f9e6b7e7787667e-dxh34a0a158594174f5.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPagef89564f402c6bc4e625970fde4b5de47_a3499ff0742498dee6251860286ff5ef_routeSheetPagef89564f402c6bc4e625970fde4b5de47 = makeLoad("/dx-components/assets/module_37_routeSheetPagef89564f402c6bc4e625970fde4b5de47-dxh8716645c3ccbee6.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPage7f1d76c6968a59ba7ea3934698f155ae_3984af74e58cc250fad6b05b096a8d73_routeSidebarPage7f1d76c6968a59ba7ea3934698f155ae = makeLoad("/dx-components/assets/module_38_routeSidebarPage7f1d76c6968a59ba7ea3934698f155ae-dxh38226b3d548b01b.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPage51aedba0a2db01567e3043e48e3354f6_3dcf3678d7fef9a08eba9bff324d2cb5_routeSkeletonPage51aedba0a2db01567e3043e48e3354f6 = makeLoad("/dx-components/assets/module_39_routeSkeletonPage51aedba0a2db01567e3043e48e3354f6-dxhfd4ff5353121f142.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPagef1700403510e2da4865bc934882af6b7_a81e8b47bca79fdf91cb3562cbffd48d_routeSliderPagef1700403510e2da4865bc934882af6b7 = makeLoad("/dx-components/assets/module_40_routeSliderPagef1700403510e2da4865bc934882af6b7-dxh8178407fec1b2af.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPagef8ca3315ecd03118ef8d811992b09b7d_13d833a8c35c1a9cdbc28fe147df3264_routeSwitchPagef8ca3315ecd03118ef8d811992b09b7d = makeLoad("/dx-components/assets/module_41_routeSwitchPagef8ca3315ecd03118ef8d811992b09b7d-dxh7549197b357ebd2b.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPagef165b43a5c26e34ad5e096079de42c23_3de5980d7ad6982c23d0d9d91bbd152e_routeTabsPagef165b43a5c26e34ad5e096079de42c23 = makeLoad("/dx-components/assets/module_42_routeTabsPagef165b43a5c26e34ad5e096079de42c23-dxh76c6bae484ab4cc.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPage7a9653012d230b5d0c4262156a9206a6_5405ab6f262260f1c97a3b66e9c32891_routeTextareaPage7a9653012d230b5d0c4262156a9206a6 = makeLoad("/dx-components/assets/module_43_routeTextareaPage7a9653012d230b5d0c4262156a9206a6-dxhbc62b94aaf7429a.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPage14693064f51a44461058f86839faec97_61b803704e892ab8e8f25ae00b119fe5_routeToastPage14693064f51a44461058f86839faec97 = makeLoad("/dx-components/assets/module_44_routeToastPage14693064f51a44461058f86839faec97-dxhfda775eca460825c.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPage50420edbbab283cbcacd716efeeda365_442b98bc1f5a4f3d15fd03e29e9f85e6_routeToggleGroupPage50420edbbab283cbcacd716efeeda365 = makeLoad("/dx-components/assets/module_45_routeToggleGroupPage50420edbbab283cbcacd716efeeda365-dxhdbe360f55a248411.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePage5e3d7c64dbf75a4b03ea937e22edb0d9_146adf5a959374397eff1c1e3d47bc1d_routeTogglePage5e3d7c64dbf75a4b03ea937e22edb0d9 = makeLoad("/dx-components/assets/module_46_routeTogglePage5e3d7c64dbf75a4b03ea937e22edb0d9-dxhed5571695a58663.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPage542efa1ab8b7f6ed099f94be4a5a8f53_9e64d59841d8a4e95062e9f02b8ab557_routeToolbarPage542efa1ab8b7f6ed099f94be4a5a8f53 = makeLoad("/dx-components/assets/module_47_routeToolbarPage542efa1ab8b7f6ed099f94be4a5a8f53-dxhda558925101fb8fe.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPage5336eec1e3274032b74c0314f059852d_69f46b5a2a33edb4776e540160e0f3bc_routeTooltipPage5336eec1e3274032b74c0314f059852d = makeLoad("/dx-components/assets/module_48_routeTooltipPage5336eec1e3274032b74c0314f059852d-dxh3c8340bb66eeeffd.wasm", [], fusedImports);
