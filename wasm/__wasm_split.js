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
export const __wasm_split_load_chunk_0 = makeLoad("/dx-components/assets/chunk_0_split-dxhe2e46c252d588854.wasm", [], fusedImports);
export const __wasm_split_load_moduleAccordionPage9b530fe5bc79ce82f74ca1d8418092c6_d81ee4eff91da7a048dbf8802a6c8cba_routeAccordionPage9b530fe5bc79ce82f74ca1d8418092c6 = makeLoad("/dx-components/assets/module_0_routeAccordionPage9b530fe5bc79ce82f74ca1d8418092c6-dxheb99e667bc673dad.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPage713290737aeda05a1b201f7088b1be60_6eba9c07012a0af0835a0b2316f767ec_routeAlertDialogPage713290737aeda05a1b201f7088b1be60 = makeLoad("/dx-components/assets/module_1_routeAlertDialogPage713290737aeda05a1b201f7088b1be60-dxhcbe4fb1874e3273d.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPage3598c3badaf74c032f884045f1407e03_e343ae010157e8c1e375e9ab1a40c45b_routeAspectRatioPage3598c3badaf74c032f884045f1407e03 = makeLoad("/dx-components/assets/module_2_routeAspectRatioPage3598c3badaf74c032f884045f1407e03-dxhab76abb545c38bba.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPage2379e72adc8672be4965791b7fd2ddc4_43b3b7d1cd2bffbb1f416e771d7504c7_routeAvatarPage2379e72adc8672be4965791b7fd2ddc4 = makeLoad("/dx-components/assets/module_3_routeAvatarPage2379e72adc8672be4965791b7fd2ddc4-dxh48221e75d661ea2.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePage404d6795115b7a103fca371a75aba696_8aab3dc19baf1b1fe91e650b06e21d32_routeBadgePage404d6795115b7a103fca371a75aba696 = makeLoad("/dx-components/assets/module_4_routeBadgePage404d6795115b7a103fca371a75aba696-dxh71e194d4f4def99a.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPageee2d3f2835b700a2374f91a841c2d0fe_fe93e575f3568299ae8de6e8d086a283_routeButtonPageee2d3f2835b700a2374f91a841c2d0fe = makeLoad("/dx-components/assets/module_5_routeButtonPageee2d3f2835b700a2374f91a841c2d0fe-dxhf52a1153e4f03dc2.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPage3ce1d4204ba3210c03c2669025e752b9_338df6899096f6a9d46e40d6ff1ae04d_routeCalendarPage3ce1d4204ba3210c03c2669025e752b9 = makeLoad("/dx-components/assets/module_6_routeCalendarPage3ce1d4204ba3210c03c2669025e752b9-dxh78c7f5d0ab55d7b9.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPage93526897bc9afbd9b4e278363d43b345_5cd791b0bdb629a90c7a262dc342694c_routeCardPage93526897bc9afbd9b4e278363d43b345 = makeLoad("/dx-components/assets/module_7_routeCardPage93526897bc9afbd9b4e278363d43b345-dxhca7f20dfd224f32.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPage346d8617adc881de411d974745ceb4c7_8259f5109f3ebded4ac39d6faeb370c9_routeCarouselPage346d8617adc881de411d974745ceb4c7 = makeLoad("/dx-components/assets/module_8_routeCarouselPage346d8617adc881de411d974745ceb4c7-dxh27daf67c50ef70e6.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPage14e06bc912103f95c8e7fc54ed93b409_42e406766e03fac6425d744570f61cd8_routeCheckboxPage14e06bc912103f95c8e7fc54ed93b409 = makeLoad("/dx-components/assets/module_9_routeCheckboxPage14e06bc912103f95c8e7fc54ed93b409-dxh1584c5b8f0454e1d.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePagef3613300e98d9c21fdd83c727ba86ced_ca7a8816996e4281e885a399aa7a2482_routeCollapsiblePagef3613300e98d9c21fdd83c727ba86ced = makeLoad("/dx-components/assets/module_10_routeCollapsiblePagef3613300e98d9c21fdd83c727ba86ced-dxhe120c67cb193753.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPage20f0828ac2acaf5af9ee8bdfab42799e_0578cc9d258968d22fbb2d5b24e33904_routeComboboxPage20f0828ac2acaf5af9ee8bdfab42799e = makeLoad("/dx-components/assets/module_11_routeComboboxPage20f0828ac2acaf5af9ee8bdfab42799e-dxh5a4a62d6f549bba7.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPage010ff396202903553680dc13081d21ec_a8683f24fc0dc5a7e319bd7269040afa_routeCommandPage010ff396202903553680dc13081d21ec = makeLoad("/dx-components/assets/module_12_routeCommandPage010ff396202903553680dc13081d21ec-dxh5d7c913337847b7.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemo8bd5d86316168f579fe957ee015cd93d_6b6e33881c729954c44ab47ce3a638a1_routeComponentBlockDemo8bd5d86316168f579fe957ee015cd93d = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemo8bd5d86316168f579fe957ee015cd93d-dxhfa34eda47f7a25d.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPage3cca2000ad6c23c4fd7a8666a952a104_4df67bdfb6b164f2acb8033dc0bdeb19_routeContextMenuPage3cca2000ad6c23c4fd7a8666a952a104 = makeLoad("/dx-components/assets/module_14_routeContextMenuPage3cca2000ad6c23c4fd7a8666a952a104-dxh3d82ca48ac95ebb4.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPage7da97353b32d69051ff44dc3df0b3aba_e7da1e18767a7c55ca9a3c2f2a59f173_routeDatePickerPage7da97353b32d69051ff44dc3df0b3aba = makeLoad("/dx-components/assets/module_15_routeDatePickerPage7da97353b32d69051ff44dc3df0b3aba-dxh4776309151911ef.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPage38d7ce802c014bcaec928051fb799715_5c0837548501e2e5ef6df5875d410690_routeDialogPage38d7ce802c014bcaec928051fb799715 = makeLoad("/dx-components/assets/module_16_routeDialogPage38d7ce802c014bcaec928051fb799715-dxhfae2824c127dd4e.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPage99599ac10555284c9d428bbb1050b857_1fdcffbec97e1f645565f87f32137385_routeDragAndDropListPage99599ac10555284c9d428bbb1050b857 = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPage99599ac10555284c9d428bbb1050b857-dxh34c6b4ff1bc86b4.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPage9bea5b11233cf7b8676053aa666e47e7_410448d21b2b09578ad0b0d599d86990_routeDrawerPage9bea5b11233cf7b8676053aa666e47e7 = makeLoad("/dx-components/assets/module_18_routeDrawerPage9bea5b11233cf7b8676053aa666e47e7-dxh2e4c2ed79a19b6b6.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPage717716b25b7fc194c003e63ce3954a38_4fc128131d39b90f0330dcfa4bd35d8b_routeDropdownMenuPage717716b25b7fc194c003e63ce3954a38 = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPage717716b25b7fc194c003e63ce3954a38-dxh8a1cc335105ca21.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPage34cddfcd70b292aeef160f5bb070fbe5_061d0b5c3f24bde386864f6c70c073be_routeFormPage34cddfcd70b292aeef160f5bb070fbe5 = makeLoad("/dx-components/assets/module_20_routeFormPage34cddfcd70b292aeef160f5bb070fbe5-dxh22c043e33151ea6d.wasm", [], fusedImports);
export const __wasm_split_load_moduleHome5973e59cb97b75cdbe618458055090eb_78d854e7410bb9b2b4c81607be65aa2f_routeHome5973e59cb97b75cdbe618458055090eb = makeLoad("/dx-components/assets/module_21_routeHome5973e59cb97b75cdbe618458055090eb-dxha85981918dbad05c.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPageef6dc504551ab8cee3626a740ef07cd1_9053c35881a3c1b9713f1f383b23fbaf_routeHoverCardPageef6dc504551ab8cee3626a740ef07cd1 = makeLoad("/dx-components/assets/module_22_routeHoverCardPageef6dc504551ab8cee3626a740ef07cd1-dxhf2e65c0b2f684f6.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPage967c8b1924af594d0fb9f17392cb77f3_e6c82d621da714a7ba5c95b72258188a_routeInputOtpPage967c8b1924af594d0fb9f17392cb77f3 = makeLoad("/dx-components/assets/module_23_routeInputOtpPage967c8b1924af594d0fb9f17392cb77f3-dxh95a6f5abdb24a057.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPage23aa12e8f627802b17c41a97d5f3b123_5dd82641284a7fc1bb54d4fbb9918622_routeInputPage23aa12e8f627802b17c41a97d5f3b123 = makeLoad("/dx-components/assets/module_24_routeInputPage23aa12e8f627802b17c41a97d5f3b123-dxh71617a9342953213.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPagec9fc1153aa5a25f0d172977b2d9aba5f_00ec69099e99da255d3238c6a1a57bce_routeLabelPagec9fc1153aa5a25f0d172977b2d9aba5f = makeLoad("/dx-components/assets/module_25_routeLabelPagec9fc1153aa5a25f0d172977b2d9aba5f-dxhd950439bb5e57d46.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPage2b959a00cd26f7fbbd3f3c7c3a5a8f2f_08b9609e640aa50548ff17cf68386c8b_routeMenubarPage2b959a00cd26f7fbbd3f3c7c3a5a8f2f = makeLoad("/dx-components/assets/module_26_routeMenubarPage2b959a00cd26f7fbbd3f3c7c3a5a8f2f-dxhc39f072843697ae.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPage5b632fdc04e8ba0734283d2df8f164b7_35ecb2be9569670eacd9f4b8a2c3a635_routeNavbarPage5b632fdc04e8ba0734283d2df8f164b7 = makeLoad("/dx-components/assets/module_27_routeNavbarPage5b632fdc04e8ba0734283d2df8f164b7-dxhf6c38bff9478b21b.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPagec4085598effcf9a30359cda17c04bf54_200898f1097d11df8cdf5ca4422ddfb1_routeNavigationMenuPagec4085598effcf9a30359cda17c04bf54 = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPagec4085598effcf9a30359cda17c04bf54-dxh20e9a3562168692d.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPage48dfea590199b3ecf557df4b27fb8a4c_e0ae18cf2264a2c73e75e4c3e4d4e183_routePaginationPage48dfea590199b3ecf557df4b27fb8a4c = makeLoad("/dx-components/assets/module_29_routePaginationPage48dfea590199b3ecf557df4b27fb8a4c-dxh69b91fea86d9d74.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPage1f8b5af8fde5b99db65f03ba243769b7_3cfaeb6018f4bf6e4909b5c00016b58e_routePopoverPage1f8b5af8fde5b99db65f03ba243769b7 = makeLoad("/dx-components/assets/module_30_routePopoverPage1f8b5af8fde5b99db65f03ba243769b7-dxh372bffa57479fdf4.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPage8c0acc5f87f7100a6467f5375cf7ee21_043d34d53dca9672980b440a5879fdd5_routeProgressPage8c0acc5f87f7100a6467f5375cf7ee21 = makeLoad("/dx-components/assets/module_31_routeProgressPage8c0acc5f87f7100a6467f5375cf7ee21-dxh386251a7a76139b2.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPage6e3c64a7ae2d72c477c1b62718b96144_1c70f46785e3c5e8ffe4d434455e2d57_routeRadioGroupPage6e3c64a7ae2d72c477c1b62718b96144 = makeLoad("/dx-components/assets/module_32_routeRadioGroupPage6e3c64a7ae2d72c477c1b62718b96144-dxhaa6a942d7849b7c.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePage70f421e31b894d0b206c7427bd24b083_ab7a4aea0f59d7ea3d3309bfa6ba04fa_routeResizablePage70f421e31b894d0b206c7427bd24b083 = makeLoad("/dx-components/assets/module_33_routeResizablePage70f421e31b894d0b206c7427bd24b083-dxha63bf554da13c522.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPage8a490bb544b9aa6da963ea4904e0f63c_569218c6a25a478d1bc1bf0dc5a076ac_routeScrollAreaPage8a490bb544b9aa6da963ea4904e0f63c = makeLoad("/dx-components/assets/module_34_routeScrollAreaPage8a490bb544b9aa6da963ea4904e0f63c-dxhd98365efb2259d20.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPage407deaa0c4f5346ae49f67b9829713ca_9f0806e6b7aaf5b76283d5b8294249ea_routeSelectPage407deaa0c4f5346ae49f67b9829713ca = makeLoad("/dx-components/assets/module_35_routeSelectPage407deaa0c4f5346ae49f67b9829713ca-dxhc5b7f19fd7d8ef77.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPage87e2f4a15fe9ee7d3e95acd9f6aa4f0d_268d6b0eaa3d044301b23816c1c94e16_routeSeparatorPage87e2f4a15fe9ee7d3e95acd9f6aa4f0d = makeLoad("/dx-components/assets/module_36_routeSeparatorPage87e2f4a15fe9ee7d3e95acd9f6aa4f0d-dxhe02290d3ad187e92.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPaged8a811fe433d53124c459a272cfc824c_16c1b76fc4c91645ba2d046eb9211261_routeSheetPaged8a811fe433d53124c459a272cfc824c = makeLoad("/dx-components/assets/module_37_routeSheetPaged8a811fe433d53124c459a272cfc824c-dxhc87c765dbe0c48.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPage6b275594010865d0ca45488308bbb32e_eda2d634ea83d560703244d6e669ea0c_routeSidebarPage6b275594010865d0ca45488308bbb32e = makeLoad("/dx-components/assets/module_38_routeSidebarPage6b275594010865d0ca45488308bbb32e-dxhe7646753b25f91a7.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPagefdb13c5c4cf3a9a1b5187afa4094136f_15058ed3278a35d5aba27fc5eeaded04_routeSkeletonPagefdb13c5c4cf3a9a1b5187afa4094136f = makeLoad("/dx-components/assets/module_39_routeSkeletonPagefdb13c5c4cf3a9a1b5187afa4094136f-dxh6bf617ed839a8dfb.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPage2efb5e748b79c6d26a468d2671b31fb5_44a4440fa3a148e730a825ea4766ef0b_routeSliderPage2efb5e748b79c6d26a468d2671b31fb5 = makeLoad("/dx-components/assets/module_40_routeSliderPage2efb5e748b79c6d26a468d2671b31fb5-dxhdc77b52e5172b774.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPagedd251de297358b194d154840949f0197_382ccefb8c53b9eb3083bca2e1b4dcf2_routeSwitchPagedd251de297358b194d154840949f0197 = makeLoad("/dx-components/assets/module_41_routeSwitchPagedd251de297358b194d154840949f0197-dxh70601573edff7c81.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPage220b92d5624a6ca1fee400ccb238d869_213980aa863d0f7be52ae0fb873769f5_routeTabsPage220b92d5624a6ca1fee400ccb238d869 = makeLoad("/dx-components/assets/module_42_routeTabsPage220b92d5624a6ca1fee400ccb238d869-dxhbadc4531c4526f41.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPage71e8a092d4257a70dae5b8ac7a8f4e0d_0858ef1b27342f25c1c548e7e20e6490_routeTextareaPage71e8a092d4257a70dae5b8ac7a8f4e0d = makeLoad("/dx-components/assets/module_43_routeTextareaPage71e8a092d4257a70dae5b8ac7a8f4e0d-dxhdeb854f97fd6b0b1.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPagecc3b271921555e69a22f8a34b71df4d1_23c2db3c26aba748af698b4e1a036c31_routeToastPagecc3b271921555e69a22f8a34b71df4d1 = makeLoad("/dx-components/assets/module_44_routeToastPagecc3b271921555e69a22f8a34b71df4d1-dxh9f8ff0252f5dc29a.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPageaa7bf3b5707675ab12fd81da72ba8cd9_57d537dc8877e3e6337c8b34ebae81cf_routeToggleGroupPageaa7bf3b5707675ab12fd81da72ba8cd9 = makeLoad("/dx-components/assets/module_45_routeToggleGroupPageaa7bf3b5707675ab12fd81da72ba8cd9-dxhcdd1ce822cbe7175.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePageb37e400529d63fc4bb318955572df0e4_8ca0962f8d4d79769ce4771a68bbef7d_routeTogglePageb37e400529d63fc4bb318955572df0e4 = makeLoad("/dx-components/assets/module_46_routeTogglePageb37e400529d63fc4bb318955572df0e4-dxhd2db82c6b710f490.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPagead10fe1f235b0a378ea080c39c98bb82_e29f8b6882a88e47fb4a0f2fa07aa937_routeToolbarPagead10fe1f235b0a378ea080c39c98bb82 = makeLoad("/dx-components/assets/module_47_routeToolbarPagead10fe1f235b0a378ea080c39c98bb82-dxh91df61555f6bbd.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPage3ac98d5bedcae7333ab48d4d1f3fb023_c6cf43bbfa234e0ccc3f0e313c4b0806_routeTooltipPage3ac98d5bedcae7333ab48d4d1f3fb023 = makeLoad("/dx-components/assets/module_48_routeTooltipPage3ac98d5bedcae7333ab48d4d1f3fb023-dxh76beafca43ae48ea.wasm", [], fusedImports);
