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
export const __wasm_split_load_moduleAccordionPagec40dfc7c95d33e1e782f25a01e729476_247158301ccf138a331bba3eba932e21_routeAccordionPagec40dfc7c95d33e1e782f25a01e729476 = makeLoad("/dx-components/assets/module_0_routeAccordionPagec40dfc7c95d33e1e782f25a01e729476-dxh3dde50b7683bd7cb.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPageb2d47ab205f0298f3cc6ed8f71b717b8_e3490683f11001d0749720277e40618f_routeAlertDialogPageb2d47ab205f0298f3cc6ed8f71b717b8 = makeLoad("/dx-components/assets/module_1_routeAlertDialogPageb2d47ab205f0298f3cc6ed8f71b717b8-dxh3cc3ff6d4e04e58.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPage8d86bf0590e4edc61bb25466b910355f_1e66b91ec009042a05ef60561d0e85bb_routeAspectRatioPage8d86bf0590e4edc61bb25466b910355f = makeLoad("/dx-components/assets/module_2_routeAspectRatioPage8d86bf0590e4edc61bb25466b910355f-dxh3037f367be4bd2.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPage48f12dbb2d9f38f5bf2c4ed4f7b89114_fb479bb60e15dc6332acc3f5ebd87fe8_routeAvatarPage48f12dbb2d9f38f5bf2c4ed4f7b89114 = makeLoad("/dx-components/assets/module_3_routeAvatarPage48f12dbb2d9f38f5bf2c4ed4f7b89114-dxh111c6b7629525d9c.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePage473a1aba532535821954b6d436f9ee8e_d74648c30a93a35d23d5572de22bf776_routeBadgePage473a1aba532535821954b6d436f9ee8e = makeLoad("/dx-components/assets/module_4_routeBadgePage473a1aba532535821954b6d436f9ee8e-dxh3ab0a972ad38fe63.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPagea6296915c5a6e537444072e11a4d0f5d_1f7fff5bedad83bcebe41504ffec82f1_routeButtonPagea6296915c5a6e537444072e11a4d0f5d = makeLoad("/dx-components/assets/module_5_routeButtonPagea6296915c5a6e537444072e11a4d0f5d-dxhe4be583b2b2cb05f.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPagee3cd2a471ef1e141bd6d7ecc365f2bea_d9668e38d3b3a1fab40054c34660ed26_routeCalendarPagee3cd2a471ef1e141bd6d7ecc365f2bea = makeLoad("/dx-components/assets/module_6_routeCalendarPagee3cd2a471ef1e141bd6d7ecc365f2bea-dxh65df3d5a44dcb37.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPage4f0934b96ec1a7667a7e6235deb2219d_be2bca23b75d85941acb3f25d7c3726e_routeCardPage4f0934b96ec1a7667a7e6235deb2219d = makeLoad("/dx-components/assets/module_7_routeCardPage4f0934b96ec1a7667a7e6235deb2219d-dxhaaecca15e2b9d2b.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPagea174506c3e1d85ec7426c180fe5ad174_6893e9d2e01c6db148ebd2080d0eab3b_routeCarouselPagea174506c3e1d85ec7426c180fe5ad174 = makeLoad("/dx-components/assets/module_8_routeCarouselPagea174506c3e1d85ec7426c180fe5ad174-dxhb4d673b2104d6b60.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPage1901163ab40d0601e654bd8cff7689f2_9ed79793f97a080adcd976762193e06a_routeCheckboxPage1901163ab40d0601e654bd8cff7689f2 = makeLoad("/dx-components/assets/module_9_routeCheckboxPage1901163ab40d0601e654bd8cff7689f2-dxh82fafbc3755b310.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePagea98a6b518fe84309512b53b40312a932_155fe4d19733cd0910f6f5ba0cc23f6d_routeCollapsiblePagea98a6b518fe84309512b53b40312a932 = makeLoad("/dx-components/assets/module_10_routeCollapsiblePagea98a6b518fe84309512b53b40312a932-dxhe0d5c567cf96cbf.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPage1108106ba088aef83c29c03625e8b921_733bd423d92508c9aade4ad6f4a9483f_routeComboboxPage1108106ba088aef83c29c03625e8b921 = makeLoad("/dx-components/assets/module_11_routeComboboxPage1108106ba088aef83c29c03625e8b921-dxhf6544f6e14fca6c.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPageaf8f64b6b5684877aab90f15cf083c67_d8a5004dab176d31edd29fa3dac4ed51_routeCommandPageaf8f64b6b5684877aab90f15cf083c67 = makeLoad("/dx-components/assets/module_12_routeCommandPageaf8f64b6b5684877aab90f15cf083c67-dxh9f321f6198a079e9.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemo4f10c3f1c892c9f2d3e6e3c993d8e119_324908f88959c794878860922bb29b49_routeComponentBlockDemo4f10c3f1c892c9f2d3e6e3c993d8e119 = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemo4f10c3f1c892c9f2d3e6e3c993d8e119-dxh416d6b7eef85deff.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPagee6740f5b335145af59b96dd879ffee47_cfbc4ce89d376bfa322875db4477b6e4_routeContextMenuPagee6740f5b335145af59b96dd879ffee47 = makeLoad("/dx-components/assets/module_14_routeContextMenuPagee6740f5b335145af59b96dd879ffee47-dxhad28f60b84f4a7f.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPage3b21c909f5dad62804f30c6dffabd09e_17a2e659b3616352b987752baa910226_routeDatePickerPage3b21c909f5dad62804f30c6dffabd09e = makeLoad("/dx-components/assets/module_15_routeDatePickerPage3b21c909f5dad62804f30c6dffabd09e-dxh2fe4d53dfd9045df.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPagebcd03b94539f40e84889cbb9fed5ef6a_ad4af5896435c0196bc5734baf358310_routeDialogPagebcd03b94539f40e84889cbb9fed5ef6a = makeLoad("/dx-components/assets/module_16_routeDialogPagebcd03b94539f40e84889cbb9fed5ef6a-dxh3f2de90b018ddd3.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPage07b5d14ebf3ce97bd0269629f707b1e6_0caa774330090ca11905c45d50538cdc_routeDragAndDropListPage07b5d14ebf3ce97bd0269629f707b1e6 = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPage07b5d14ebf3ce97bd0269629f707b1e6-dxh19c57d67e534e058.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPage3955340fcd61c611067045ac2ccfb629_4bb611e3afce05e28a084754bf1eda07_routeDrawerPage3955340fcd61c611067045ac2ccfb629 = makeLoad("/dx-components/assets/module_18_routeDrawerPage3955340fcd61c611067045ac2ccfb629-dxha7c5d71b47b9d4c.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPagea1e6bcd852a176e3d046b0e7e12b19f6_54dc695507d8980df74a9a1942963525_routeDropdownMenuPagea1e6bcd852a176e3d046b0e7e12b19f6 = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPagea1e6bcd852a176e3d046b0e7e12b19f6-dxhd241506a86e82b46.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPage8a0ed1146413f756c4dd71cd91be1240_e91592df6887c87b28c2034eacc3e3d1_routeFormPage8a0ed1146413f756c4dd71cd91be1240 = makeLoad("/dx-components/assets/module_20_routeFormPage8a0ed1146413f756c4dd71cd91be1240-dxh76bff08e4b8cb45.wasm", [], fusedImports);
export const __wasm_split_load_moduleHomed3bf9240dcf77778a1a1a2241e99851c_c42da11ac7ac21a6ff8360193c53daf9_routeHomed3bf9240dcf77778a1a1a2241e99851c = makeLoad("/dx-components/assets/module_21_routeHomed3bf9240dcf77778a1a1a2241e99851c-dxhd1b23cbf2352f12c.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPageb56b006a35c14a4f19efa2c5f0ed7647_c98fb3e4fd458b6848bb50c935ee1580_routeHoverCardPageb56b006a35c14a4f19efa2c5f0ed7647 = makeLoad("/dx-components/assets/module_22_routeHoverCardPageb56b006a35c14a4f19efa2c5f0ed7647-dxhabd1dec499ec5dac.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPage43412e8d93febdec7ee90252be1bd2f6_05353b36c225adff63d1af3e99ed4aa1_routeInputOtpPage43412e8d93febdec7ee90252be1bd2f6 = makeLoad("/dx-components/assets/module_23_routeInputOtpPage43412e8d93febdec7ee90252be1bd2f6-dxhb7ce1835d466d42.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPage2d506b4dc3fc7a6637c4ce3e1a0a526c_d8fb609ea8b6ad7d6f30c4d92c7d99ae_routeInputPage2d506b4dc3fc7a6637c4ce3e1a0a526c = makeLoad("/dx-components/assets/module_24_routeInputPage2d506b4dc3fc7a6637c4ce3e1a0a526c-dxh06a49f349414.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPage54d34e2f3de965345e643c41a2dcdda3_425b8ee2690a296e90f57731bf8fd60e_routeLabelPage54d34e2f3de965345e643c41a2dcdda3 = makeLoad("/dx-components/assets/module_25_routeLabelPage54d34e2f3de965345e643c41a2dcdda3-dxh5e49bdc49e9205d.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPaged169a2e6ca256433d1b4ccf4075af8ff_c9b77f94686a44d0acd8d4f556622264_routeMenubarPaged169a2e6ca256433d1b4ccf4075af8ff = makeLoad("/dx-components/assets/module_26_routeMenubarPaged169a2e6ca256433d1b4ccf4075af8ff-dxh2b40b3bbed1f73f5.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPagefbaf7389e9d0dc013c207db641764111_a57bc2c9ba04f013bf5f9f9fa13c391d_routeNavbarPagefbaf7389e9d0dc013c207db641764111 = makeLoad("/dx-components/assets/module_27_routeNavbarPagefbaf7389e9d0dc013c207db641764111-dxh79603ed0b894d93.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPage7316f0cc50d2e55c1944b2be24620d52_7e36f089b3ac846549c2655ca7f05515_routeNavigationMenuPage7316f0cc50d2e55c1944b2be24620d52 = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPage7316f0cc50d2e55c1944b2be24620d52-dxhfee6be824f9d0e1.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPage97f4535063a8e604a246c99f1f305cfe_88cbd97d6c48996d2eb9fd404b3edc22_routePaginationPage97f4535063a8e604a246c99f1f305cfe = makeLoad("/dx-components/assets/module_29_routePaginationPage97f4535063a8e604a246c99f1f305cfe-dxh96f1b4d1b2b735b.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPage956a194c0d5bbb5e5b9c305ba9d9ebcd_72c77702d7073f8d19e9fda9f1d6ec36_routePopoverPage956a194c0d5bbb5e5b9c305ba9d9ebcd = makeLoad("/dx-components/assets/module_30_routePopoverPage956a194c0d5bbb5e5b9c305ba9d9ebcd-dxhfbaf77ea3148f4bc.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPage2bfeed117bfbe48f31debff3f8f1037a_aa99c37108d3778fb9d41623f991b025_routeProgressPage2bfeed117bfbe48f31debff3f8f1037a = makeLoad("/dx-components/assets/module_31_routeProgressPage2bfeed117bfbe48f31debff3f8f1037a-dxhbedc43928ba538b0.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPage0aeea09cb59475c591713ab01303a4ca_624b333cb817a2f7a3007300b46aa8dd_routeRadioGroupPage0aeea09cb59475c591713ab01303a4ca = makeLoad("/dx-components/assets/module_32_routeRadioGroupPage0aeea09cb59475c591713ab01303a4ca-dxhe566ef208262ff14.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePage28e0c2b8a3cc5684acdb82038f295775_33c1ebe98a04f2dc3838b9f7a4ef27f8_routeResizablePage28e0c2b8a3cc5684acdb82038f295775 = makeLoad("/dx-components/assets/module_33_routeResizablePage28e0c2b8a3cc5684acdb82038f295775-dxh469cdb2471126e.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPage048712a5e147558226594014e04dfd18_4461ee9f102a7bed9a209f939dab14cd_routeScrollAreaPage048712a5e147558226594014e04dfd18 = makeLoad("/dx-components/assets/module_34_routeScrollAreaPage048712a5e147558226594014e04dfd18-dxhc9505aed14a31e.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPage262a53fc3dc286aeabc47c025ef9428f_56049106aca1945674c5c6b4fd834343_routeSelectPage262a53fc3dc286aeabc47c025ef9428f = makeLoad("/dx-components/assets/module_35_routeSelectPage262a53fc3dc286aeabc47c025ef9428f-dxh1f51e54de93fbc3e.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPage30f9a802bc4ce5efe911e600e2ba5142_86758726fc39dd9e45c75501d29f1208_routeSeparatorPage30f9a802bc4ce5efe911e600e2ba5142 = makeLoad("/dx-components/assets/module_36_routeSeparatorPage30f9a802bc4ce5efe911e600e2ba5142-dxh7c90343d36d9780.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPagecb82cb83487e1d7bb986dddc2066493d_25def5b3a47e142bda204cdb467274d4_routeSheetPagecb82cb83487e1d7bb986dddc2066493d = makeLoad("/dx-components/assets/module_37_routeSheetPagecb82cb83487e1d7bb986dddc2066493d-dxha6d6aa88f41d50eb.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPageb0ad98ec84078a62b0eac5e3a6e06eb1_6a9000e02ec55c6589ce0911c8b996ea_routeSidebarPageb0ad98ec84078a62b0eac5e3a6e06eb1 = makeLoad("/dx-components/assets/module_38_routeSidebarPageb0ad98ec84078a62b0eac5e3a6e06eb1-dxha98f8351c78f9747.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPagea3291b30f7a46e0c7259d0342ea9037e_3abb75c1af94480867f98b15af8ebab7_routeSkeletonPagea3291b30f7a46e0c7259d0342ea9037e = makeLoad("/dx-components/assets/module_39_routeSkeletonPagea3291b30f7a46e0c7259d0342ea9037e-dxh2b4c69b5d769feb7.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPagecfbbac830e41b271c567a5179b41e3c6_166368a27e39780e5a8171ce16e5c92c_routeSliderPagecfbbac830e41b271c567a5179b41e3c6 = makeLoad("/dx-components/assets/module_40_routeSliderPagecfbbac830e41b271c567a5179b41e3c6-dxh271bb8c2c283865e.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPage0f0cdc72822fc0f7799a697580934709_42995584d01f910499c2348a1ecd4432_routeSwitchPage0f0cdc72822fc0f7799a697580934709 = makeLoad("/dx-components/assets/module_41_routeSwitchPage0f0cdc72822fc0f7799a697580934709-dxhc04c6436865e82d.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPage31a50457f41ce3eb9c3d8401e2005acb_e2b32a415dbb2810e17d58a2a1f4fea1_routeTabsPage31a50457f41ce3eb9c3d8401e2005acb = makeLoad("/dx-components/assets/module_42_routeTabsPage31a50457f41ce3eb9c3d8401e2005acb-dxhf317f4b24dcabf2f.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPage1e5a3b98a9a3cb7d6b3b4361bb61b0a3_8913e1b8e55e26d6009eba5379fb96e8_routeTextareaPage1e5a3b98a9a3cb7d6b3b4361bb61b0a3 = makeLoad("/dx-components/assets/module_43_routeTextareaPage1e5a3b98a9a3cb7d6b3b4361bb61b0a3-dxh1f9f75b4d65ce2e7.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPagea85758451aeea1ba346c114c1a65743b_d8c1e23e10fce10cffb05ae44440f3df_routeToastPagea85758451aeea1ba346c114c1a65743b = makeLoad("/dx-components/assets/module_44_routeToastPagea85758451aeea1ba346c114c1a65743b-dxhe8fa7483ce12545a.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPagea3aa32b77213fd64a9b394006e6563d2_1fc43c7c76c0464d44224591ba0afe85_routeToggleGroupPagea3aa32b77213fd64a9b394006e6563d2 = makeLoad("/dx-components/assets/module_45_routeToggleGroupPagea3aa32b77213fd64a9b394006e6563d2-dxh38d0ef74842fc862.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePage617f7817c313f94d6f67f82e3d0b5de0_58ac2f359e2f3707b4c0a70a8f38b8b8_routeTogglePage617f7817c313f94d6f67f82e3d0b5de0 = makeLoad("/dx-components/assets/module_46_routeTogglePage617f7817c313f94d6f67f82e3d0b5de0-dxhd782c9b63d7d28c1.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPagebabeba9bba9002e58e609d86d802865e_91f5425e789bd76c4e46700615eccfc0_routeToolbarPagebabeba9bba9002e58e609d86d802865e = makeLoad("/dx-components/assets/module_47_routeToolbarPagebabeba9bba9002e58e609d86d802865e-dxhf05948376cbfaa95.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPagee8c64126c8b4d29929b440d8d33027d3_2540759387b4ffccbd23ef1369181195_routeTooltipPagee8c64126c8b4d29929b440d8d33027d3 = makeLoad("/dx-components/assets/module_48_routeTooltipPagee8c64126c8b4d29929b440d8d33027d3-dxh899652b0c258dcea.wasm", [], fusedImports);
