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
export const __wasm_split_load_moduleAccordionPageea2d4cd273f9ee0a002639cbd3fa8d02_ed7436ac1fc7fc04c0732ef169e1850e_routeAccordionPageea2d4cd273f9ee0a002639cbd3fa8d02 = makeLoad("/dx-components/assets/module_0_routeAccordionPageea2d4cd273f9ee0a002639cbd3fa8d02-dxh44136d39f9394ff1.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPage68fe2c0627f586ad5bd95eab7d73fba0_c24ca59a5d3ed1acd48e5f80d6ba175b_routeAlertDialogPage68fe2c0627f586ad5bd95eab7d73fba0 = makeLoad("/dx-components/assets/module_1_routeAlertDialogPage68fe2c0627f586ad5bd95eab7d73fba0-dxh6f1745a69ef5a99.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPage94a414a1dc1a19a3f32448b46e18580a_9e08ecac094fb6f9de9b2f510e6b7b39_routeAspectRatioPage94a414a1dc1a19a3f32448b46e18580a = makeLoad("/dx-components/assets/module_2_routeAspectRatioPage94a414a1dc1a19a3f32448b46e18580a-dxhf2c9d92113f2a05b.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPage0ca4914c0033449850af8002dad3f722_82d7f2a70bf5eb4cbd2a08133b50942e_routeAvatarPage0ca4914c0033449850af8002dad3f722 = makeLoad("/dx-components/assets/module_3_routeAvatarPage0ca4914c0033449850af8002dad3f722-dxhcde0fd35a8e6318a.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePage8e8b9a93ecba82f14a9ba794bcfbb810_bfc00097ec1c7d0b634e35e15e6084d1_routeBadgePage8e8b9a93ecba82f14a9ba794bcfbb810 = makeLoad("/dx-components/assets/module_4_routeBadgePage8e8b9a93ecba82f14a9ba794bcfbb810-dxh867d748d369a5a1.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPageef62f9e17ec22f499268d82e0f942da9_4cd8d662f6d17813a283cd441eeb2cf4_routeButtonPageef62f9e17ec22f499268d82e0f942da9 = makeLoad("/dx-components/assets/module_5_routeButtonPageef62f9e17ec22f499268d82e0f942da9-dxhdf8d173fdecfd8e.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPageee85854794e94457a24f0f950dd17042_3157a8817a1fb5b9bcdc9cf90c4f9833_routeCalendarPageee85854794e94457a24f0f950dd17042 = makeLoad("/dx-components/assets/module_6_routeCalendarPageee85854794e94457a24f0f950dd17042-dxh8783d77b6f26c152.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPage881442dec8e32e39d86494dd5802a24a_a4aa182d416202f234230ee6b6d8f26f_routeCardPage881442dec8e32e39d86494dd5802a24a = makeLoad("/dx-components/assets/module_7_routeCardPage881442dec8e32e39d86494dd5802a24a-dxh96913f6bf392dc.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPage9dc5ecd3c9a3d1e038719ae0300a3b2e_a95bfb9c6ecd0fbfc51fbfe11b618153_routeCarouselPage9dc5ecd3c9a3d1e038719ae0300a3b2e = makeLoad("/dx-components/assets/module_8_routeCarouselPage9dc5ecd3c9a3d1e038719ae0300a3b2e-dxh11d05628c98b2a1b.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPage73cf5bd3f250a848ed96df33eac3eab7_689e0ef75c25430a3c5c0564df319ab6_routeCheckboxPage73cf5bd3f250a848ed96df33eac3eab7 = makeLoad("/dx-components/assets/module_9_routeCheckboxPage73cf5bd3f250a848ed96df33eac3eab7-dxhf03b78fede9b1522.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePage12135a5e520c6c4d2c70830374f48df5_e336a6504f9e48cbaf1d77656a4f152f_routeCollapsiblePage12135a5e520c6c4d2c70830374f48df5 = makeLoad("/dx-components/assets/module_10_routeCollapsiblePage12135a5e520c6c4d2c70830374f48df5-dxh5ab0b41fb177416a.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPage4bd9c494bc7daf7dcad76887e4392453_ced845360188b1d810f2a063dd720dbb_routeComboboxPage4bd9c494bc7daf7dcad76887e4392453 = makeLoad("/dx-components/assets/module_11_routeComboboxPage4bd9c494bc7daf7dcad76887e4392453-dxh624d4a856b8603e.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPage06c93a711bc6d10c6d187fe517f2c7c4_8632e1fecd7eb81644be9132a022748a_routeCommandPage06c93a711bc6d10c6d187fe517f2c7c4 = makeLoad("/dx-components/assets/module_12_routeCommandPage06c93a711bc6d10c6d187fe517f2c7c4-dxh10a0e3e2b3a3aab6.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemo8d1909eb7d8f8d67159480e1e0690c95_ec8e9e308aa80b29c877781fca2d05a8_routeComponentBlockDemo8d1909eb7d8f8d67159480e1e0690c95 = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemo8d1909eb7d8f8d67159480e1e0690c95-dxhf8a581eead3cd3a.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPage9f4e69a0618373b3d977406233a34ed8_f9e835a299bae3d2645c223d639118ff_routeContextMenuPage9f4e69a0618373b3d977406233a34ed8 = makeLoad("/dx-components/assets/module_14_routeContextMenuPage9f4e69a0618373b3d977406233a34ed8-dxh3c7f69427a1d819.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPagebe92966737ef09d9a2d8cd1693c36487_fea188ca4be8eb7de8f86311e12363cc_routeDatePickerPagebe92966737ef09d9a2d8cd1693c36487 = makeLoad("/dx-components/assets/module_15_routeDatePickerPagebe92966737ef09d9a2d8cd1693c36487-dxh0bee74983a992.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPage803f73f4c48f33d82c7bf54c6b5096a1_79f5f3a0d1b1eeed850614ec82e8ca62_routeDialogPage803f73f4c48f33d82c7bf54c6b5096a1 = makeLoad("/dx-components/assets/module_16_routeDialogPage803f73f4c48f33d82c7bf54c6b5096a1-dxhcd37d7d3a43959f.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPage726dec122f0554b79977d561c7f46dee_859eaedc6e4c8c82b25486e26d321dcd_routeDragAndDropListPage726dec122f0554b79977d561c7f46dee = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPage726dec122f0554b79977d561c7f46dee-dxhf7ef9118c7262b2e.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPage390702ff76dbba18baebae02f9aa6c76_652b744cac452af9a5565bef1b604111_routeDrawerPage390702ff76dbba18baebae02f9aa6c76 = makeLoad("/dx-components/assets/module_18_routeDrawerPage390702ff76dbba18baebae02f9aa6c76-dxh991961f2b9d96af.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPagecec2d42f45d29ccdb3cd549717eefd5c_743f0213fad3d3bb8cc04ec8e39e8b98_routeDropdownMenuPagecec2d42f45d29ccdb3cd549717eefd5c = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPagecec2d42f45d29ccdb3cd549717eefd5c-dxh6e80643b516027a6.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPage26f226e9747240a0e59fe614c20ffebd_9dc454b8f639188d368be174a3cda5d6_routeFormPage26f226e9747240a0e59fe614c20ffebd = makeLoad("/dx-components/assets/module_20_routeFormPage26f226e9747240a0e59fe614c20ffebd-dxh40c86c2a60125dfc.wasm", [], fusedImports);
export const __wasm_split_load_moduleHomed51c9cd64b6816b9bab2495114d9e3b9_d441565368f0ee6e92f4dee322c0b224_routeHomed51c9cd64b6816b9bab2495114d9e3b9 = makeLoad("/dx-components/assets/module_21_routeHomed51c9cd64b6816b9bab2495114d9e3b9-dxh5acf3bafd1447d.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPage88d547b3471fd1979504ef67b6c98382_27511851604a35fb7876ecdf814221d4_routeHoverCardPage88d547b3471fd1979504ef67b6c98382 = makeLoad("/dx-components/assets/module_22_routeHoverCardPage88d547b3471fd1979504ef67b6c98382-dxh12b4116e3cc6c6bb.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPageddf661b25f79aba1bbb83cf7ece9fa6f_39a507e7218e48a3e411616e5f0db21f_routeInputOtpPageddf661b25f79aba1bbb83cf7ece9fa6f = makeLoad("/dx-components/assets/module_23_routeInputOtpPageddf661b25f79aba1bbb83cf7ece9fa6f-dxhf2c2bd4e229bec38.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPagefb2fb236410fc86fdbbca21eca96ab64_a80d33623a28625a679bd0c79e93c945_routeInputPagefb2fb236410fc86fdbbca21eca96ab64 = makeLoad("/dx-components/assets/module_24_routeInputPagefb2fb236410fc86fdbbca21eca96ab64-dxh8b4fd0256cabc57a.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPage2d18ea0a4c6fba4678ee118a44ec0777_7843268d19c6aca12592d5546048980b_routeLabelPage2d18ea0a4c6fba4678ee118a44ec0777 = makeLoad("/dx-components/assets/module_25_routeLabelPage2d18ea0a4c6fba4678ee118a44ec0777-dxh8e677a88b8948f.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPagedf961876ad88f0212729f1022de99e45_62800ecbf0d22a7f53bcfa15a00fac1a_routeMenubarPagedf961876ad88f0212729f1022de99e45 = makeLoad("/dx-components/assets/module_26_routeMenubarPagedf961876ad88f0212729f1022de99e45-dxhdde15e53b89d3cf9.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPaged9b9ab6e0f111f9f2e8e42d24a8309cc_4e8347dbc2de500a14317ad69fd908be_routeNavbarPaged9b9ab6e0f111f9f2e8e42d24a8309cc = makeLoad("/dx-components/assets/module_27_routeNavbarPaged9b9ab6e0f111f9f2e8e42d24a8309cc-dxh19833d234b76ff0.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPage3061961a57c4df65cde2954a35b88d29_77bbbdc59da269abda10ef2f84002a6f_routeNavigationMenuPage3061961a57c4df65cde2954a35b88d29 = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPage3061961a57c4df65cde2954a35b88d29-dxha8b978c506a35a6.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPage57a196ff8d2c975c3432bdfc31092c46_68ca724a66766f7869669a3665a61424_routePaginationPage57a196ff8d2c975c3432bdfc31092c46 = makeLoad("/dx-components/assets/module_29_routePaginationPage57a196ff8d2c975c3432bdfc31092c46-dxh57db588907cea6f.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPage97841c5d6ff83b0d4ff259cc185f0efe_e912ca408e5bd22fe05b8a0de2f4dbb5_routePopoverPage97841c5d6ff83b0d4ff259cc185f0efe = makeLoad("/dx-components/assets/module_30_routePopoverPage97841c5d6ff83b0d4ff259cc185f0efe-dxhb3667eadad35e9.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPageafb29a6a089abc8ad165746ac0988fc7_79ad7045d9d82400b21e329cac58bb97_routeProgressPageafb29a6a089abc8ad165746ac0988fc7 = makeLoad("/dx-components/assets/module_31_routeProgressPageafb29a6a089abc8ad165746ac0988fc7-dxhbf8d7f78bae214.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPage020bc9661d43a9b695f13136991d1414_27337bdde390886907800055143eb3d9_routeRadioGroupPage020bc9661d43a9b695f13136991d1414 = makeLoad("/dx-components/assets/module_32_routeRadioGroupPage020bc9661d43a9b695f13136991d1414-dxh84851c4fc8a164f4.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePage5129bfeb2a55616e8e31052381331512_7e2ccaa6da220e0e49f346d3ac6505ec_routeResizablePage5129bfeb2a55616e8e31052381331512 = makeLoad("/dx-components/assets/module_33_routeResizablePage5129bfeb2a55616e8e31052381331512-dxh9663995eb5d8ef71.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPagee013a2edaab8745e921d1b4f8b68dec0_008638c1149973c2f4452e34b4b35d8a_routeScrollAreaPagee013a2edaab8745e921d1b4f8b68dec0 = makeLoad("/dx-components/assets/module_34_routeScrollAreaPagee013a2edaab8745e921d1b4f8b68dec0-dxh2746e5ddd7142ce8.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPage7a0ef03a4da6b1cd7ce8e1a10f6f44a4_901502300d646f0c95cd6278f637d0a2_routeSelectPage7a0ef03a4da6b1cd7ce8e1a10f6f44a4 = makeLoad("/dx-components/assets/module_35_routeSelectPage7a0ef03a4da6b1cd7ce8e1a10f6f44a4-dxh7b781a33e3ec26c.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPage5c573ac72ff8522e576a60f72499e6b0_200738a832adf2f43ae979d2c2b0308b_routeSeparatorPage5c573ac72ff8522e576a60f72499e6b0 = makeLoad("/dx-components/assets/module_36_routeSeparatorPage5c573ac72ff8522e576a60f72499e6b0-dxh4be73cb5a18c49e3.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPagea16183a9758d8ed9a9754fac2b204cf1_c92f919aa07a9c3420e661bab19023d5_routeSheetPagea16183a9758d8ed9a9754fac2b204cf1 = makeLoad("/dx-components/assets/module_37_routeSheetPagea16183a9758d8ed9a9754fac2b204cf1-dxha73094df3c478eba.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPage14f4ebe2eac1a187dd8373dd56dddee1_bbdbbadcfc14ad84cab24c8ba25409db_routeSidebarPage14f4ebe2eac1a187dd8373dd56dddee1 = makeLoad("/dx-components/assets/module_38_routeSidebarPage14f4ebe2eac1a187dd8373dd56dddee1-dxh927aaead69b3d1c.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPage0f55a60f55d511df94cf384c56851113_132a42b5d9c5cc3d473ae20c43065332_routeSkeletonPage0f55a60f55d511df94cf384c56851113 = makeLoad("/dx-components/assets/module_39_routeSkeletonPage0f55a60f55d511df94cf384c56851113-dxh57f9302b3e7bcca.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPage8c08c8c44c3528e065266fd0d884a2d2_bda16e24068132943117e52d65ee4b73_routeSliderPage8c08c8c44c3528e065266fd0d884a2d2 = makeLoad("/dx-components/assets/module_40_routeSliderPage8c08c8c44c3528e065266fd0d884a2d2-dxh174b9871e66bdc72.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPage587e34b6e55d6efd192e1f4b51239394_fdecd61bb6337ef4ef7e6b3722eae424_routeSwitchPage587e34b6e55d6efd192e1f4b51239394 = makeLoad("/dx-components/assets/module_41_routeSwitchPage587e34b6e55d6efd192e1f4b51239394-dxh58421bb781feba9f.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPage3150cc02be5323488ec95a0d4760b341_d2ae2a33532110ab16a05fc4a0a3e14d_routeTabsPage3150cc02be5323488ec95a0d4760b341 = makeLoad("/dx-components/assets/module_42_routeTabsPage3150cc02be5323488ec95a0d4760b341-dxhff6bdeaa91b26df2.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPagea2a3a61bb1f16965384ddfb38a1a0546_2dfdf8f8fdab1f597509db466997d5ad_routeTextareaPagea2a3a61bb1f16965384ddfb38a1a0546 = makeLoad("/dx-components/assets/module_43_routeTextareaPagea2a3a61bb1f16965384ddfb38a1a0546-dxh92f49beca1a41cf.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPage6af238587bb94ceaff8bf044e2f3bc8b_1f87e49c140bdfba0fc375f588465669_routeToastPage6af238587bb94ceaff8bf044e2f3bc8b = makeLoad("/dx-components/assets/module_44_routeToastPage6af238587bb94ceaff8bf044e2f3bc8b-dxh46351052f5a8f35d.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPage90f4be3f788ff84f490e28744a54a26c_ea32b1386b8b735c972652e3f7470ab6_routeToggleGroupPage90f4be3f788ff84f490e28744a54a26c = makeLoad("/dx-components/assets/module_45_routeToggleGroupPage90f4be3f788ff84f490e28744a54a26c-dxh5c194b3325bdfd6.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePageec63b73df0fd8e04dad9835eda2f5ea0_643ebc861b2f4e7bbe1c19f6df76a8b5_routeTogglePageec63b73df0fd8e04dad9835eda2f5ea0 = makeLoad("/dx-components/assets/module_46_routeTogglePageec63b73df0fd8e04dad9835eda2f5ea0-dxhb5514e3833c53cb1.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPageb0c37c4a84f5753b1641b38f356036ff_2ea6c7f81b05cc7565725f1e06bb40f4_routeToolbarPageb0c37c4a84f5753b1641b38f356036ff = makeLoad("/dx-components/assets/module_47_routeToolbarPageb0c37c4a84f5753b1641b38f356036ff-dxhf33be0e8a7534ec8.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPage0d9fc4c04d18659a5ae5b61e4e3178d4_a7a125d80417a64727d87c1760042ed7_routeTooltipPage0d9fc4c04d18659a5ae5b61e4e3178d4 = makeLoad("/dx-components/assets/module_48_routeTooltipPage0d9fc4c04d18659a5ae5b61e4e3178d4-dxh92d47e64ef1974.wasm", [], fusedImports);
