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
export const __wasm_split_load_moduleAccordionPage9e695420f77c3c4eb660d82e93ad8cb3_9eb952d5783c4f1b23530dc4a1d61a97_routeAccordionPage9e695420f77c3c4eb660d82e93ad8cb3 = makeLoad("/dx-components/assets/module_0_routeAccordionPage9e695420f77c3c4eb660d82e93ad8cb3-dxh9a315c998013783c.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPage405168e04c7b559947478e2e7e7aec3f_cddd5f6ee718fc515578e5cb73cca54c_routeAlertDialogPage405168e04c7b559947478e2e7e7aec3f = makeLoad("/dx-components/assets/module_1_routeAlertDialogPage405168e04c7b559947478e2e7e7aec3f-dxhb3ba1822a53fad8.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPageed1a3b351355daa62f05c4f1b6bbce3b_dd86090d25bce572e037449e5a4b419f_routeAspectRatioPageed1a3b351355daa62f05c4f1b6bbce3b = makeLoad("/dx-components/assets/module_2_routeAspectRatioPageed1a3b351355daa62f05c4f1b6bbce3b-dxh37b5d3331fbb8ba.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPage07eff45f36ef7cf0a8610102fb206bff_1f8de88a7927a78be6d36371cb8c8fee_routeAvatarPage07eff45f36ef7cf0a8610102fb206bff = makeLoad("/dx-components/assets/module_3_routeAvatarPage07eff45f36ef7cf0a8610102fb206bff-dxhf79fc2a546182be2.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePageb611f529567b755f946e9fd1bee19944_9eaac336d898eb7a7a5d45dc1770f556_routeBadgePageb611f529567b755f946e9fd1bee19944 = makeLoad("/dx-components/assets/module_4_routeBadgePageb611f529567b755f946e9fd1bee19944-dxh7dc7c66ea318c719.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPage13a58fc5e2828b0487de589b6fbc20b5_589881873ccab84ca249d4975c09c001_routeButtonPage13a58fc5e2828b0487de589b6fbc20b5 = makeLoad("/dx-components/assets/module_5_routeButtonPage13a58fc5e2828b0487de589b6fbc20b5-dxha68a57719207579.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPage91e5703fdf78ba5583b783da42eb9473_f14e51644d4941856ba3bb1e63c55671_routeCalendarPage91e5703fdf78ba5583b783da42eb9473 = makeLoad("/dx-components/assets/module_6_routeCalendarPage91e5703fdf78ba5583b783da42eb9473-dxh6c14220acd77820.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPaged86225d8f487025b7c9dd1e350a3b467_93cfe5eea4e802e6d1497c84519206c4_routeCardPaged86225d8f487025b7c9dd1e350a3b467 = makeLoad("/dx-components/assets/module_7_routeCardPaged86225d8f487025b7c9dd1e350a3b467-dxhc1f34622c3caa027.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPageb9852852341b79ce608d59dd7ac781e6_e90e2203c33f13fcfa9ff90d8bc9a519_routeCarouselPageb9852852341b79ce608d59dd7ac781e6 = makeLoad("/dx-components/assets/module_8_routeCarouselPageb9852852341b79ce608d59dd7ac781e6-dxh2dc5d59aa4822e56.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPagec663e3fbc6d7450dae6a91a954565bda_5c6dde5c7caedf7269712fb9eb95c21b_routeCheckboxPagec663e3fbc6d7450dae6a91a954565bda = makeLoad("/dx-components/assets/module_9_routeCheckboxPagec663e3fbc6d7450dae6a91a954565bda-dxhda60ff698b57b85b.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePage592686b6850b1c5d8964527614bfdbff_f8b0fabbe1e21ae7ba7620cf46bf3362_routeCollapsiblePage592686b6850b1c5d8964527614bfdbff = makeLoad("/dx-components/assets/module_10_routeCollapsiblePage592686b6850b1c5d8964527614bfdbff-dxh4089394c9aa34e0.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPage3127ef6a27e70b37a3ec96eda74171ae_a0a185a3a3bd3db6378359c4df0ef5d1_routeComboboxPage3127ef6a27e70b37a3ec96eda74171ae = makeLoad("/dx-components/assets/module_11_routeComboboxPage3127ef6a27e70b37a3ec96eda74171ae-dxhca5bd912a0882840.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPage63e14c1ec57b3e1c4df266703e78353f_a1873aa141fd006cb66ce0c2fce3496e_routeCommandPage63e14c1ec57b3e1c4df266703e78353f = makeLoad("/dx-components/assets/module_12_routeCommandPage63e14c1ec57b3e1c4df266703e78353f-dxhe8b2e783daddc62e.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemo2612c92cf363ffb32b0f11235e435607_f72cbdf7a2c6ec49eb97b44fe10837ee_routeComponentBlockDemo2612c92cf363ffb32b0f11235e435607 = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemo2612c92cf363ffb32b0f11235e435607-dxh22c0b382ff2043cc.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPagec5e9183ba6f24aad92b1f566e8cf3620_b8e3f9b0046d9666cdfc7228687b9fcc_routeContextMenuPagec5e9183ba6f24aad92b1f566e8cf3620 = makeLoad("/dx-components/assets/module_14_routeContextMenuPagec5e9183ba6f24aad92b1f566e8cf3620-dxh7718abeda92c7aeb.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPage2b11529a8cdd8d82b657a4fddba0d21b_9e9f1222d7a728419dbb1938ee75f9f6_routeDatePickerPage2b11529a8cdd8d82b657a4fddba0d21b = makeLoad("/dx-components/assets/module_15_routeDatePickerPage2b11529a8cdd8d82b657a4fddba0d21b-dxhd21a3a1fdd5d77f9.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPageb19150eaa6fdf028045f5b9206378cbf_ded46269935752d80893d502cf3080a8_routeDialogPageb19150eaa6fdf028045f5b9206378cbf = makeLoad("/dx-components/assets/module_16_routeDialogPageb19150eaa6fdf028045f5b9206378cbf-dxhfabccefdae5332f7.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPagee3800557bd248b8f07ca81375aa58b6d_90227702d7eb36d3967c4e052540428e_routeDragAndDropListPagee3800557bd248b8f07ca81375aa58b6d = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPagee3800557bd248b8f07ca81375aa58b6d-dxh15e4a6700c2cdfc.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPagee0d90da917465441628c8812b7466a45_34273df2d11b3d9bde267bb5a0453e80_routeDrawerPagee0d90da917465441628c8812b7466a45 = makeLoad("/dx-components/assets/module_18_routeDrawerPagee0d90da917465441628c8812b7466a45-dxhefb85e37a521ac6.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPagefca177f41085430ba464b9ad1318fb8a_bfd3473280990d6711d77852ba7baae2_routeDropdownMenuPagefca177f41085430ba464b9ad1318fb8a = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPagefca177f41085430ba464b9ad1318fb8a-dxhc31b244ee2f9feb.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPagea22d84213baa278f80170ae8e27c421b_42c655bbad0baac77dcec111fa50a655_routeFormPagea22d84213baa278f80170ae8e27c421b = makeLoad("/dx-components/assets/module_20_routeFormPagea22d84213baa278f80170ae8e27c421b-dxh7176c3c1fc32a47e.wasm", [], fusedImports);
export const __wasm_split_load_moduleHomeedcc8c6fe4a7e9b25a339f8bea277785_fc72632604dbb3fae1edf9b2de07b48b_routeHomeedcc8c6fe4a7e9b25a339f8bea277785 = makeLoad("/dx-components/assets/module_21_routeHomeedcc8c6fe4a7e9b25a339f8bea277785-dxh8e764a312bdb7556.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPage96f028f8b76f515f7f30eff16e3a5130_37b4d0b73dc2385f6c111bddbd898b91_routeHoverCardPage96f028f8b76f515f7f30eff16e3a5130 = makeLoad("/dx-components/assets/module_22_routeHoverCardPage96f028f8b76f515f7f30eff16e3a5130-dxhd0a11c911a6993f1.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPagef3e27f2dc913279be2fd1c402d5d03a1_d5248bf274846ba8210605386029a61b_routeInputOtpPagef3e27f2dc913279be2fd1c402d5d03a1 = makeLoad("/dx-components/assets/module_23_routeInputOtpPagef3e27f2dc913279be2fd1c402d5d03a1-dxh66294de9d314f44.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPage59c06bd4f29c28b70ef77a379e66cdd7_ba94393afc9f9713d3c6154465f39680_routeInputPage59c06bd4f29c28b70ef77a379e66cdd7 = makeLoad("/dx-components/assets/module_24_routeInputPage59c06bd4f29c28b70ef77a379e66cdd7-dxh74ab3f55e7e352aa.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPage77c86ff778baafed94f84725d65b1269_cd1d5e1efdc5e8f9bd716890a065ac43_routeLabelPage77c86ff778baafed94f84725d65b1269 = makeLoad("/dx-components/assets/module_25_routeLabelPage77c86ff778baafed94f84725d65b1269-dxhbfabc1187986dd0.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPage053333438f49c666546419a0f18e2aca_ded05573723f8782206931937fe81305_routeMenubarPage053333438f49c666546419a0f18e2aca = makeLoad("/dx-components/assets/module_26_routeMenubarPage053333438f49c666546419a0f18e2aca-dxh3db2636d124ec2b8.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPagebefca6784a674ea3a6a4b03b8cbd3b55_2cfc1c4c33721085891da2f9283b073e_routeNavbarPagebefca6784a674ea3a6a4b03b8cbd3b55 = makeLoad("/dx-components/assets/module_27_routeNavbarPagebefca6784a674ea3a6a4b03b8cbd3b55-dxh643a6061455ac4ca.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPageef1837aa59c0714b88d7b127023a9ab8_c72cf4cd5b79dcf0040576776759a094_routeNavigationMenuPageef1837aa59c0714b88d7b127023a9ab8 = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPageef1837aa59c0714b88d7b127023a9ab8-dxh3d138f4bef1e86c.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPage3c876e83918657161037586f50edc04a_979ffbefe04f90cdae079567bff5fbc1_routePaginationPage3c876e83918657161037586f50edc04a = makeLoad("/dx-components/assets/module_29_routePaginationPage3c876e83918657161037586f50edc04a-dxh672cf8441db18285.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPage50f07ca4c1b36905dc6d82fa4546582f_a8821cca78433d1238ffe177a7bf30ec_routePopoverPage50f07ca4c1b36905dc6d82fa4546582f = makeLoad("/dx-components/assets/module_30_routePopoverPage50f07ca4c1b36905dc6d82fa4546582f-dxh7ae6ec4e341eed54.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPagea62ca38e8da84bd27ff0aaa9e47a0b5b_d5e4ac68e99e1b6822bbfb51277270d7_routeProgressPagea62ca38e8da84bd27ff0aaa9e47a0b5b = makeLoad("/dx-components/assets/module_31_routeProgressPagea62ca38e8da84bd27ff0aaa9e47a0b5b-dxhac4f01a5497197e.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPage0c33fb30a54fc566415cbbb186d4ca0d_107caaa87db3573509c82438632fc884_routeRadioGroupPage0c33fb30a54fc566415cbbb186d4ca0d = makeLoad("/dx-components/assets/module_32_routeRadioGroupPage0c33fb30a54fc566415cbbb186d4ca0d-dxh26ad734220858497.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePage8662542e808aa10394d0f6a455aa8c6d_a627a110f12cb26c93ef87216c13ee12_routeResizablePage8662542e808aa10394d0f6a455aa8c6d = makeLoad("/dx-components/assets/module_33_routeResizablePage8662542e808aa10394d0f6a455aa8c6d-dxh5dc49d453b67ccf2.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPagea3c652e886b66bbbf2ed6030287f333c_04fd43b233a9e00693fec4d2068a5789_routeScrollAreaPagea3c652e886b66bbbf2ed6030287f333c = makeLoad("/dx-components/assets/module_34_routeScrollAreaPagea3c652e886b66bbbf2ed6030287f333c-dxheb79ef5595ec4215.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPagecd3b95ac07befec0ddedc331c5264445_aab5c10df73b7c729b69e75d5a2378fb_routeSelectPagecd3b95ac07befec0ddedc331c5264445 = makeLoad("/dx-components/assets/module_35_routeSelectPagecd3b95ac07befec0ddedc331c5264445-dxheee43447bfb74026.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPage129105b1772ef047108f0afc9ec0ff7e_683e6d844aebed39aa28e0af04c527a1_routeSeparatorPage129105b1772ef047108f0afc9ec0ff7e = makeLoad("/dx-components/assets/module_36_routeSeparatorPage129105b1772ef047108f0afc9ec0ff7e-dxheea521a715fa332d.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPageaab445222602c576cbe17890db7b966d_4ab9f2fde6ce18c71de1e3a4a45c7b79_routeSheetPageaab445222602c576cbe17890db7b966d = makeLoad("/dx-components/assets/module_37_routeSheetPageaab445222602c576cbe17890db7b966d-dxhcac63ba44f70436f.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPage8dd501fd7e27598522f62a10845a84d2_b0266b719fd55957c192e012febd0cb2_routeSidebarPage8dd501fd7e27598522f62a10845a84d2 = makeLoad("/dx-components/assets/module_38_routeSidebarPage8dd501fd7e27598522f62a10845a84d2-dxh2f971546595a3d55.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPage1bd0d6ced404401cf4adbc9004ea4bcb_7dea1ebe91d3c0bfd004e6c135fb066f_routeSkeletonPage1bd0d6ced404401cf4adbc9004ea4bcb = makeLoad("/dx-components/assets/module_39_routeSkeletonPage1bd0d6ced404401cf4adbc9004ea4bcb-dxhdbc58191f013321f.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPage24010b92e542c5bb5b4e111300cb07f4_6f15c4161a2a7a9742b0a803acbfe7f9_routeSliderPage24010b92e542c5bb5b4e111300cb07f4 = makeLoad("/dx-components/assets/module_40_routeSliderPage24010b92e542c5bb5b4e111300cb07f4-dxh8a5d4a5f0a7236e.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPage89e755c8f542341952e10d2751815340_5bde3d05a540875b730fa2d1e8c06065_routeSwitchPage89e755c8f542341952e10d2751815340 = makeLoad("/dx-components/assets/module_41_routeSwitchPage89e755c8f542341952e10d2751815340-dxh2a87e33c1fb4ddc8.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPage305ff65e7e6a9b963130eb101c31d1b0_7bba97504538a51f0a134c5ad16796e3_routeTabsPage305ff65e7e6a9b963130eb101c31d1b0 = makeLoad("/dx-components/assets/module_42_routeTabsPage305ff65e7e6a9b963130eb101c31d1b0-dxhfc7a9af74aa28d36.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPage38fb6cb78ee122d4dcdafbcc21ecbe00_15583e912d778ed1285c8ca87a612add_routeTextareaPage38fb6cb78ee122d4dcdafbcc21ecbe00 = makeLoad("/dx-components/assets/module_43_routeTextareaPage38fb6cb78ee122d4dcdafbcc21ecbe00-dxhd4f8aafffa06882.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPage01c79fffabb835ddd390c51a10a1b481_f22aa5cf7e6934eba4304dffb09cb195_routeToastPage01c79fffabb835ddd390c51a10a1b481 = makeLoad("/dx-components/assets/module_44_routeToastPage01c79fffabb835ddd390c51a10a1b481-dxhb9f218dafac03478.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPagef5f2b640e3cf713c1d3bbdf3e193b94b_8875a1a7bbd90a393025142b12f77b18_routeToggleGroupPagef5f2b640e3cf713c1d3bbdf3e193b94b = makeLoad("/dx-components/assets/module_45_routeToggleGroupPagef5f2b640e3cf713c1d3bbdf3e193b94b-dxhed154abd2b9d9ba.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePage8ffbe73994af7ddbde8850513b5ff35c_f0a14914dfd9d5df933a74b967176576_routeTogglePage8ffbe73994af7ddbde8850513b5ff35c = makeLoad("/dx-components/assets/module_46_routeTogglePage8ffbe73994af7ddbde8850513b5ff35c-dxhc5c90e0f96f372b.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPage8eb5aa1fb709412a1312a8fcbb68a87b_050fa1a403ae7255f3bc2fa6f750af3c_routeToolbarPage8eb5aa1fb709412a1312a8fcbb68a87b = makeLoad("/dx-components/assets/module_47_routeToolbarPage8eb5aa1fb709412a1312a8fcbb68a87b-dxh19ddb2b6814089e.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPagea839fa9d3ba044f35cdf17be5361157c_dc387684108abf3beb3bf57950a1fbde_routeTooltipPagea839fa9d3ba044f35cdf17be5361157c = makeLoad("/dx-components/assets/module_48_routeTooltipPagea839fa9d3ba044f35cdf17be5361157c-dxha8dc64e8bb94a818.wasm", [], fusedImports);
