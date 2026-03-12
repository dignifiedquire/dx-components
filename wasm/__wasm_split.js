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
export const __wasm_split_load_moduleAccordionPagefec1caa7358878fa8bf65dfa14993a09_a1c1caed8feb49aebd069f35d56d010e_routeAccordionPagefec1caa7358878fa8bf65dfa14993a09 = makeLoad("/dx-components/assets/module_0_routeAccordionPagefec1caa7358878fa8bf65dfa14993a09-dxhddc1fa51fcf6a78.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPageaea2b8b57c2a3f4c11de83135918c8b9_396dfd8b61a040122a5cac7795276278_routeAlertDialogPageaea2b8b57c2a3f4c11de83135918c8b9 = makeLoad("/dx-components/assets/module_1_routeAlertDialogPageaea2b8b57c2a3f4c11de83135918c8b9-dxh50ecd3af2592f07e.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPageae813ca5040fb3a6fe8b0ae56e61881c_e7fa40dd3f8ed049bd2c311edbe5024b_routeAspectRatioPageae813ca5040fb3a6fe8b0ae56e61881c = makeLoad("/dx-components/assets/module_2_routeAspectRatioPageae813ca5040fb3a6fe8b0ae56e61881c-dxh946c1a2f9064f260.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPage51b7558d35f9177b1308dec40ff175b9_2212696e249ecb6d3e66d55da037e7a5_routeAvatarPage51b7558d35f9177b1308dec40ff175b9 = makeLoad("/dx-components/assets/module_3_routeAvatarPage51b7558d35f9177b1308dec40ff175b9-dxh567eca61e46d3346.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePage2798f69953fe1e65c8646160dd2f82b8_341ab6898e53bb7a4324cf4c648b37fb_routeBadgePage2798f69953fe1e65c8646160dd2f82b8 = makeLoad("/dx-components/assets/module_4_routeBadgePage2798f69953fe1e65c8646160dd2f82b8-dxhee3721341281aaff.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPage9f04fe784dcb0b5bb169d7abdc3da8fc_9e129cec738aa300b447e5abaaa8eb67_routeButtonPage9f04fe784dcb0b5bb169d7abdc3da8fc = makeLoad("/dx-components/assets/module_5_routeButtonPage9f04fe784dcb0b5bb169d7abdc3da8fc-dxh10f4696cc49eedd.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPage7af9ab8c4fd3384b7f048dd7a31fdaaf_3db7041275d81607fc8afe480ef8fd70_routeCalendarPage7af9ab8c4fd3384b7f048dd7a31fdaaf = makeLoad("/dx-components/assets/module_6_routeCalendarPage7af9ab8c4fd3384b7f048dd7a31fdaaf-dxhbca6b038ba52d961.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPageabe9f047a5092b4867761354ca7c9bee_9be47ab04bc3359812ed05f3fa95f888_routeCardPageabe9f047a5092b4867761354ca7c9bee = makeLoad("/dx-components/assets/module_7_routeCardPageabe9f047a5092b4867761354ca7c9bee-dxh27173b05ff9b1f4.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPageddc92be24b068a93f9eda7fc22a3e86c_f67a3bfcbe2db64241d20f03cda63420_routeCarouselPageddc92be24b068a93f9eda7fc22a3e86c = makeLoad("/dx-components/assets/module_8_routeCarouselPageddc92be24b068a93f9eda7fc22a3e86c-dxhff5688962cf96841.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPaged1d173f0dfce16de5c2193946f22b61b_39b20dcbc9fc97bc017afd8af95a6537_routeCheckboxPaged1d173f0dfce16de5c2193946f22b61b = makeLoad("/dx-components/assets/module_9_routeCheckboxPaged1d173f0dfce16de5c2193946f22b61b-dxhcb9bb18f5d69626a.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePage2a8a244459d64591b5ef58c917493d58_4132491eddbaf6bf57c1981ed8cc0f0c_routeCollapsiblePage2a8a244459d64591b5ef58c917493d58 = makeLoad("/dx-components/assets/module_10_routeCollapsiblePage2a8a244459d64591b5ef58c917493d58-dxh14d5a1f72c1035f9.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPaged7b43b052c4d9a3725af7e47e4bb0f6f_192fbb73e9f9df353d438aee132ba37f_routeComboboxPaged7b43b052c4d9a3725af7e47e4bb0f6f = makeLoad("/dx-components/assets/module_11_routeComboboxPaged7b43b052c4d9a3725af7e47e4bb0f6f-dxh7ac345bfd5502041.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPage8272fc57a7281f6d8a774f74aa5b4c07_fc97a080d4e7f167b087f0fdbc853d88_routeCommandPage8272fc57a7281f6d8a774f74aa5b4c07 = makeLoad("/dx-components/assets/module_12_routeCommandPage8272fc57a7281f6d8a774f74aa5b4c07-dxh94312f1e9d2188b.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemo8c7733b51464637fdbedd414ac9bf912_fd849952ae5a8c72c54afeb729db3c18_routeComponentBlockDemo8c7733b51464637fdbedd414ac9bf912 = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemo8c7733b51464637fdbedd414ac9bf912-dxhb146abb9e5c88529.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPage907edc21e1d45f590ec431ffe0286334_4aea86c2a21e5bb51f0872173520f694_routeContextMenuPage907edc21e1d45f590ec431ffe0286334 = makeLoad("/dx-components/assets/module_14_routeContextMenuPage907edc21e1d45f590ec431ffe0286334-dxh52a516a72fb9ff4.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPage7fe024494b7ace3440883c6f47e03013_fd603c2b51edfd967bcaa64751be5390_routeDatePickerPage7fe024494b7ace3440883c6f47e03013 = makeLoad("/dx-components/assets/module_15_routeDatePickerPage7fe024494b7ace3440883c6f47e03013-dxh43617cb55a73821d.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPaged75901537c8212796d41c5b498a9870e_14dd8b06c4cfe4543d9349d609433b97_routeDialogPaged75901537c8212796d41c5b498a9870e = makeLoad("/dx-components/assets/module_16_routeDialogPaged75901537c8212796d41c5b498a9870e-dxh591e774694eeaf25.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPage526a5be4ebf68901739b47ea7f9da972_dad705c540186c298d9136a33309648e_routeDragAndDropListPage526a5be4ebf68901739b47ea7f9da972 = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPage526a5be4ebf68901739b47ea7f9da972-dxh946fd13dc509bc7.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPage74889d749c951be4c1288a46b6fc2462_38f6e9d1f5c85a9406302404edb74bfe_routeDrawerPage74889d749c951be4c1288a46b6fc2462 = makeLoad("/dx-components/assets/module_18_routeDrawerPage74889d749c951be4c1288a46b6fc2462-dxh3f23ea54b3a64f1e.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPaged39eb8bf2d038be2909418aee9f71833_e163df72c20a347d770ffca18473b99b_routeDropdownMenuPaged39eb8bf2d038be2909418aee9f71833 = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPaged39eb8bf2d038be2909418aee9f71833-dxhb17fe91c65af707b.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPage79c4ab914ac865c84806fa5c4e4c1fcf_ee979c196b726cf0ccf93910560fcd45_routeFormPage79c4ab914ac865c84806fa5c4e4c1fcf = makeLoad("/dx-components/assets/module_20_routeFormPage79c4ab914ac865c84806fa5c4e4c1fcf-dxhf1a6d787e7115db.wasm", [], fusedImports);
export const __wasm_split_load_moduleHome38cb31c4aca122e619c350b48e737a9c_9bc39cf3f5318f8d0a8cc03eda14f49e_routeHome38cb31c4aca122e619c350b48e737a9c = makeLoad("/dx-components/assets/module_21_routeHome38cb31c4aca122e619c350b48e737a9c-dxhf7b7408a5ee8392.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPage76902a76fc7be054e5395eb3fa65325b_6bdb2d9b5ebfd5637f46f7fa802be8e9_routeHoverCardPage76902a76fc7be054e5395eb3fa65325b = makeLoad("/dx-components/assets/module_22_routeHoverCardPage76902a76fc7be054e5395eb3fa65325b-dxh8ffec375cd68f971.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPageee0c844dc2cdf592c1e82b9948374f84_a4eba4b5c102972739dd9a195aa09818_routeInputOtpPageee0c844dc2cdf592c1e82b9948374f84 = makeLoad("/dx-components/assets/module_23_routeInputOtpPageee0c844dc2cdf592c1e82b9948374f84-dxh4877ec1838d136f.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPage9d3f6811aa032c47c0e6ded2094719f6_646fca367de5083e4678eba33a9fc985_routeInputPage9d3f6811aa032c47c0e6ded2094719f6 = makeLoad("/dx-components/assets/module_24_routeInputPage9d3f6811aa032c47c0e6ded2094719f6-dxha1221789b1446082.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPagee80d0f12a4824bdd8e4bd8c264077606_21853b531ae5f5d343554dbe2a5fe2d6_routeLabelPagee80d0f12a4824bdd8e4bd8c264077606 = makeLoad("/dx-components/assets/module_25_routeLabelPagee80d0f12a4824bdd8e4bd8c264077606-dxh642bc7f172529a6d.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPage55f155c8ab8620c956c7761fdf4b4e2a_460881555d51152169f4f5a6c4244def_routeMenubarPage55f155c8ab8620c956c7761fdf4b4e2a = makeLoad("/dx-components/assets/module_26_routeMenubarPage55f155c8ab8620c956c7761fdf4b4e2a-dxh3f5dd14a6869e9c6.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPageab9178e80ec57f0863025b7586478469_c039596bf088d9b688ec92ad93f4fd7d_routeNavbarPageab9178e80ec57f0863025b7586478469 = makeLoad("/dx-components/assets/module_27_routeNavbarPageab9178e80ec57f0863025b7586478469-dxh3d2ff61b5c93571.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPage414086395ae18c5b0dcb09253374d828_a002bb60d93c39b73105bce60ca40c9e_routeNavigationMenuPage414086395ae18c5b0dcb09253374d828 = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPage414086395ae18c5b0dcb09253374d828-dxhb5be507ff0fef2fb.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPagea9a83865eb7b7aaa8c2699be9b06bcb0_096bf81bb1a0a2714a7b29bf2a80d591_routePaginationPagea9a83865eb7b7aaa8c2699be9b06bcb0 = makeLoad("/dx-components/assets/module_29_routePaginationPagea9a83865eb7b7aaa8c2699be9b06bcb0-dxh943b4c85764c17.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPagea581978c27c40fd6432ad547852d6f14_2d814de022c91090d5fa31bee7692dd7_routePopoverPagea581978c27c40fd6432ad547852d6f14 = makeLoad("/dx-components/assets/module_30_routePopoverPagea581978c27c40fd6432ad547852d6f14-dxhc27b3bb137e9981a.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPage24f0e8b5091335dd7a40ea6cc7f22921_b09a18b50a26aa6b07188692ffb2d1da_routeProgressPage24f0e8b5091335dd7a40ea6cc7f22921 = makeLoad("/dx-components/assets/module_31_routeProgressPage24f0e8b5091335dd7a40ea6cc7f22921-dxh675c58e45a20f61.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPage3312fcf45a277eba295d092a2832d026_dc5a5f5a10766bd403d34e6dce9e1e5f_routeRadioGroupPage3312fcf45a277eba295d092a2832d026 = makeLoad("/dx-components/assets/module_32_routeRadioGroupPage3312fcf45a277eba295d092a2832d026-dxhc7854411ea48a6f.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePage7be66165e334b0a7b873d6146eafa796_ee0c552557539476560f6ab310bdc1b7_routeResizablePage7be66165e334b0a7b873d6146eafa796 = makeLoad("/dx-components/assets/module_33_routeResizablePage7be66165e334b0a7b873d6146eafa796-dxhd56f909452b9ea13.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPagef10bcb2493ac5a1e9a92be305bec4265_fc54c315df11d7496e86977e51acf2dd_routeScrollAreaPagef10bcb2493ac5a1e9a92be305bec4265 = makeLoad("/dx-components/assets/module_34_routeScrollAreaPagef10bcb2493ac5a1e9a92be305bec4265-dxh9fc4c0cf6f6f20.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPage85ef0766d1364668d32a9c8eeaa174bb_796620a985b3b108f83262e57b0febf4_routeSelectPage85ef0766d1364668d32a9c8eeaa174bb = makeLoad("/dx-components/assets/module_35_routeSelectPage85ef0766d1364668d32a9c8eeaa174bb-dxh62bf3ab38bc266a.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPagebb653cc26cfd0775ace72b90a5a4f38f_88703a351cab9dd9bd745faeb9d4ae8e_routeSeparatorPagebb653cc26cfd0775ace72b90a5a4f38f = makeLoad("/dx-components/assets/module_36_routeSeparatorPagebb653cc26cfd0775ace72b90a5a4f38f-dxh671fcd3343f518c.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPage66c90bf8ec1b179e9446d7e613b45e80_b188bb8b200420860c10ddabeebf6dd8_routeSheetPage66c90bf8ec1b179e9446d7e613b45e80 = makeLoad("/dx-components/assets/module_37_routeSheetPage66c90bf8ec1b179e9446d7e613b45e80-dxh6e7b70b09ac471a.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPage889eda2fa96166fa47616bced23f47c7_0f965f022db6790452216b67c3f47343_routeSidebarPage889eda2fa96166fa47616bced23f47c7 = makeLoad("/dx-components/assets/module_38_routeSidebarPage889eda2fa96166fa47616bced23f47c7-dxhe161847c69961117.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPage40d39b1113d6e9bf051d198ef220b920_f3413c5887a5a60a6d33db0e77b73381_routeSkeletonPage40d39b1113d6e9bf051d198ef220b920 = makeLoad("/dx-components/assets/module_39_routeSkeletonPage40d39b1113d6e9bf051d198ef220b920-dxh88949b1f2be9d3d9.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPage2b1ab021b370319c7f3a3d941e8480ce_c39a64aca59e23c076b12daeadd5ead5_routeSliderPage2b1ab021b370319c7f3a3d941e8480ce = makeLoad("/dx-components/assets/module_40_routeSliderPage2b1ab021b370319c7f3a3d941e8480ce-dxhc22ecdc41541674d.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPage29a0bee69eb02fd2860c9a46adcbb9b1_c0f81cd9460be6175ac67b1872f805bb_routeSwitchPage29a0bee69eb02fd2860c9a46adcbb9b1 = makeLoad("/dx-components/assets/module_41_routeSwitchPage29a0bee69eb02fd2860c9a46adcbb9b1-dxh847f6265552f577.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPage4da33e2cb2c05a9f32f1b5940a37d130_ddc2358787ae783ebc1a6252790c5e27_routeTabsPage4da33e2cb2c05a9f32f1b5940a37d130 = makeLoad("/dx-components/assets/module_42_routeTabsPage4da33e2cb2c05a9f32f1b5940a37d130-dxhe323771cb74a4199.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPage5b716b03d93718df74c1a62421524d39_ee925caa371b9520c6b614b6e064c312_routeTextareaPage5b716b03d93718df74c1a62421524d39 = makeLoad("/dx-components/assets/module_43_routeTextareaPage5b716b03d93718df74c1a62421524d39-dxh69a05999d9f467.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPage75e9c12791f50bd64477d9ccf87a91ca_e72c277ac5668a1c10bf85ec38c6ba11_routeToastPage75e9c12791f50bd64477d9ccf87a91ca = makeLoad("/dx-components/assets/module_44_routeToastPage75e9c12791f50bd64477d9ccf87a91ca-dxh56696eb864897035.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPage76e7cff495e059a47ad4559dcf75c97a_8b40341e330e18d1c4e3b1f604647d99_routeToggleGroupPage76e7cff495e059a47ad4559dcf75c97a = makeLoad("/dx-components/assets/module_45_routeToggleGroupPage76e7cff495e059a47ad4559dcf75c97a-dxh54b2e1deb7f417f9.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePagec25ba97415a112289a1f949a62df3c66_2a29b97406159100835b8080593c95a8_routeTogglePagec25ba97415a112289a1f949a62df3c66 = makeLoad("/dx-components/assets/module_46_routeTogglePagec25ba97415a112289a1f949a62df3c66-dxhb4dc94578f760d9.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPage0f9cc49a9935bb9db69be7459dfc854a_4f6470c6822bfa2b548864c23a5fde63_routeToolbarPage0f9cc49a9935bb9db69be7459dfc854a = makeLoad("/dx-components/assets/module_47_routeToolbarPage0f9cc49a9935bb9db69be7459dfc854a-dxhf373c0547fe45da2.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPage83ab9daba90502fb2f6f6156c86bdb3a_04917d02d40332f7518006dc9d1ed3e0_routeTooltipPage83ab9daba90502fb2f6f6156c86bdb3a = makeLoad("/dx-components/assets/module_48_routeTooltipPage83ab9daba90502fb2f6f6156c86bdb3a-dxhe91c3da6474b1dc9.wasm", [], fusedImports);
