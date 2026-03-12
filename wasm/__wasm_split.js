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
export const __wasm_split_load_moduleAccordionPage389e8050ae3b50e8427365a4e9086f90_74e9e4c1d95548bdcf593584518e94d4_routeAccordionPage389e8050ae3b50e8427365a4e9086f90 = makeLoad("/dx-components/assets/module_0_routeAccordionPage389e8050ae3b50e8427365a4e9086f90-dxh327f1cb231375cfe.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPagef9e04d97e31a0b9dc3c712853028673f_0acc7dd94d6836d985fa8a3bd747b303_routeAlertDialogPagef9e04d97e31a0b9dc3c712853028673f = makeLoad("/dx-components/assets/module_1_routeAlertDialogPagef9e04d97e31a0b9dc3c712853028673f-dxhf86f97682e0c61c.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPage2111fe0bb49a08279a4de24a339c8b8e_c840b5e53914edfb9e64d7d9611dc564_routeAspectRatioPage2111fe0bb49a08279a4de24a339c8b8e = makeLoad("/dx-components/assets/module_2_routeAspectRatioPage2111fe0bb49a08279a4de24a339c8b8e-dxhc7d340ff5d804954.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPage995dcf7b510496ea3d10799d08762023_bcd70f919888a33ba6394a970f2c91f1_routeAvatarPage995dcf7b510496ea3d10799d08762023 = makeLoad("/dx-components/assets/module_3_routeAvatarPage995dcf7b510496ea3d10799d08762023-dxh1a12ebb27eb58d67.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePage0025eb43fff2d38fb8b17372b5f60f42_3fac923142f0247c2d357c6cb546002e_routeBadgePage0025eb43fff2d38fb8b17372b5f60f42 = makeLoad("/dx-components/assets/module_4_routeBadgePage0025eb43fff2d38fb8b17372b5f60f42-dxhaa2926dc9cce051.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPageb811b0e132552ad3777f9e5550d26d91_8a471d2214611be7762b4e7d5c6b937e_routeButtonPageb811b0e132552ad3777f9e5550d26d91 = makeLoad("/dx-components/assets/module_5_routeButtonPageb811b0e132552ad3777f9e5550d26d91-dxh512fec35a4ee559e.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPagee29b006bf9bea547a9416f860f365103_f354f4444f1507cef800d1bf67badedf_routeCalendarPagee29b006bf9bea547a9416f860f365103 = makeLoad("/dx-components/assets/module_6_routeCalendarPagee29b006bf9bea547a9416f860f365103-dxhae324ebbf174c298.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPagea0c81b09e91618612200a2f553d12ccf_2519089d51a3a4f7465de64e646fc879_routeCardPagea0c81b09e91618612200a2f553d12ccf = makeLoad("/dx-components/assets/module_7_routeCardPagea0c81b09e91618612200a2f553d12ccf-dxh4e709f72f6c24178.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPage194d7b997176ec692a8059f350646f21_5ba16d69dd60ad3fb5d0800b4192930a_routeCarouselPage194d7b997176ec692a8059f350646f21 = makeLoad("/dx-components/assets/module_8_routeCarouselPage194d7b997176ec692a8059f350646f21-dxhf84bcf3a4490e7.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPage25b803e1ff483900d9ffff22a2f89877_244031947dade22220ac1baf77eb69f9_routeCheckboxPage25b803e1ff483900d9ffff22a2f89877 = makeLoad("/dx-components/assets/module_9_routeCheckboxPage25b803e1ff483900d9ffff22a2f89877-dxh1fcc4fa2429c9c0.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePage671b958855ee82b7396e3c403c98f847_a713781348fbcb4ee6b8140188ba5f37_routeCollapsiblePage671b958855ee82b7396e3c403c98f847 = makeLoad("/dx-components/assets/module_10_routeCollapsiblePage671b958855ee82b7396e3c403c98f847-dxhe9b282d7b55ba425.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPagef4a8c27a57d25ccf1003fcd40a177424_a36d90c87e2aa34bd98dc473d0fc88d2_routeComboboxPagef4a8c27a57d25ccf1003fcd40a177424 = makeLoad("/dx-components/assets/module_11_routeComboboxPagef4a8c27a57d25ccf1003fcd40a177424-dxh7adae10cd2d95d8.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPagec7d4e8b277ad38158c8c1236bda7c1bd_c668662f8a77baa420ddaec30b65fc56_routeCommandPagec7d4e8b277ad38158c8c1236bda7c1bd = makeLoad("/dx-components/assets/module_12_routeCommandPagec7d4e8b277ad38158c8c1236bda7c1bd-dxhbe80ac94dea229db.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemod094a98ed568fda2258ce2fa8694c2b7_d8e4e5f9ebaab1d5c4a443f45b341473_routeComponentBlockDemod094a98ed568fda2258ce2fa8694c2b7 = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemod094a98ed568fda2258ce2fa8694c2b7-dxhc62233de1f08e24.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPage51037c40495b7a02d9d42de0f33e9b26_665fa5ec1ceb6aa6665bd1efbaa7ec0b_routeContextMenuPage51037c40495b7a02d9d42de0f33e9b26 = makeLoad("/dx-components/assets/module_14_routeContextMenuPage51037c40495b7a02d9d42de0f33e9b26-dxhbe39099f68fe34.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPage70b33a9e0b53036d2c94333b1a20a8f2_23dcbc17c32a2c11fd68f697c84ad67e_routeDatePickerPage70b33a9e0b53036d2c94333b1a20a8f2 = makeLoad("/dx-components/assets/module_15_routeDatePickerPage70b33a9e0b53036d2c94333b1a20a8f2-dxh3e3e4548c6ea663.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPage4486074d8909ab6d0cc8bd74d4909c3d_0472a7fd46376d9842b7e9c2ee23f3bd_routeDialogPage4486074d8909ab6d0cc8bd74d4909c3d = makeLoad("/dx-components/assets/module_16_routeDialogPage4486074d8909ab6d0cc8bd74d4909c3d-dxh9a9d9058eb7e3be0.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPage155a3f55dae106d798f15b9e01380133_5c893eac0e4b83fee3205e9cafb232a1_routeDragAndDropListPage155a3f55dae106d798f15b9e01380133 = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPage155a3f55dae106d798f15b9e01380133-dxh8d3ded5569fcaad.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPage817927d74b3a249310432e1144bd9b33_1d31fe9a6d32270e7f4607db3ab24b24_routeDrawerPage817927d74b3a249310432e1144bd9b33 = makeLoad("/dx-components/assets/module_18_routeDrawerPage817927d74b3a249310432e1144bd9b33-dxh8cffd192c243c880.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPage68907104f195daca0a313c12613e29cb_42c00a00ba59588e6df4a77875608502_routeDropdownMenuPage68907104f195daca0a313c12613e29cb = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPage68907104f195daca0a313c12613e29cb-dxhd6138054a367fc5c.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPage09c9309205f0ebfaf9bcb7d3aa6fbf24_4f73704585ce41ee4b3a6ad2db689241_routeFormPage09c9309205f0ebfaf9bcb7d3aa6fbf24 = makeLoad("/dx-components/assets/module_20_routeFormPage09c9309205f0ebfaf9bcb7d3aa6fbf24-dxh5f2d589d7c75fce8.wasm", [], fusedImports);
export const __wasm_split_load_moduleHomeeb32f7779897acc09d37ea8a0c561ef6_262f19081e563580c7c9c509aea617fe_routeHomeeb32f7779897acc09d37ea8a0c561ef6 = makeLoad("/dx-components/assets/module_21_routeHomeeb32f7779897acc09d37ea8a0c561ef6-dxhccd0686bd542acb.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPage7fce33f437f0675b43882634cd63e608_0272f5a559594e084d294b0bd7902c3d_routeHoverCardPage7fce33f437f0675b43882634cd63e608 = makeLoad("/dx-components/assets/module_22_routeHoverCardPage7fce33f437f0675b43882634cd63e608-dxh9da5a3b3a417b97c.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPagec7ad4c89595efbe4e3286b8c632bccb3_b08b984f2ea62d493255fd79243d9885_routeInputOtpPagec7ad4c89595efbe4e3286b8c632bccb3 = makeLoad("/dx-components/assets/module_23_routeInputOtpPagec7ad4c89595efbe4e3286b8c632bccb3-dxhc08ec1960ab9813.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPagead6273beb2155fa715ae4d57df88a1a0_89b27e5eff50986da286a1b85252b396_routeInputPagead6273beb2155fa715ae4d57df88a1a0 = makeLoad("/dx-components/assets/module_24_routeInputPagead6273beb2155fa715ae4d57df88a1a0-dxhac1a4e96c51d6ee6.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPagee6c325e9ef4b2a795cd29408e0a5afc2_bf2bba64b3e728c795d28fe8be82a220_routeLabelPagee6c325e9ef4b2a795cd29408e0a5afc2 = makeLoad("/dx-components/assets/module_25_routeLabelPagee6c325e9ef4b2a795cd29408e0a5afc2-dxh7c3298813b1bb55.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPagecc285bc52fb9cb3935bc16729ea3fd24_e27b1843113643c1afa22c2b8e93ee7f_routeMenubarPagecc285bc52fb9cb3935bc16729ea3fd24 = makeLoad("/dx-components/assets/module_26_routeMenubarPagecc285bc52fb9cb3935bc16729ea3fd24-dxh9f479c7e23798883.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPage872efbd5031cf69a67011add17611716_bb7d6e55b615ca6be4a2142322c654ff_routeNavbarPage872efbd5031cf69a67011add17611716 = makeLoad("/dx-components/assets/module_27_routeNavbarPage872efbd5031cf69a67011add17611716-dxh8abd41d0f58b438c.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPage6018f23b27f0c00da26a4b001b922c9b_6c01deaba5e8464c3e682d15ef8db193_routeNavigationMenuPage6018f23b27f0c00da26a4b001b922c9b = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPage6018f23b27f0c00da26a4b001b922c9b-dxhf489d7121398e5b5.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPage55bd7433559bf206b981fb5bf5ccc23c_b7a5c591aec704f393ec6fabeac2e193_routePaginationPage55bd7433559bf206b981fb5bf5ccc23c = makeLoad("/dx-components/assets/module_29_routePaginationPage55bd7433559bf206b981fb5bf5ccc23c-dxh7e9cb97b1ab699cb.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPage3ba3ad637701ffad58e96b03ed3e9b1a_2e933cb8a523466bd3c2816ead7b33b3_routePopoverPage3ba3ad637701ffad58e96b03ed3e9b1a = makeLoad("/dx-components/assets/module_30_routePopoverPage3ba3ad637701ffad58e96b03ed3e9b1a-dxh16998d731eed357.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPagec49679e6bcd5a666f928eeee5ea6bbc9_8f6dfd683416b83c68052e7f315856a4_routeProgressPagec49679e6bcd5a666f928eeee5ea6bbc9 = makeLoad("/dx-components/assets/module_31_routeProgressPagec49679e6bcd5a666f928eeee5ea6bbc9-dxh80a4e483ed5c23c.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPagedaa7d234b389b3f0d062aaebb5c9c238_b92a81cbf1a60cd51a1751195a1b2a86_routeRadioGroupPagedaa7d234b389b3f0d062aaebb5c9c238 = makeLoad("/dx-components/assets/module_32_routeRadioGroupPagedaa7d234b389b3f0d062aaebb5c9c238-dxhd82a98486175be7.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePage73843614834d9c57bea9df2b788523e4_44e02824ebecb281e030d03a7a861ec7_routeResizablePage73843614834d9c57bea9df2b788523e4 = makeLoad("/dx-components/assets/module_33_routeResizablePage73843614834d9c57bea9df2b788523e4-dxhbd7e5eb14050ea40.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPage874a609d0c0bd278f71a646f9963852f_be09dbdbb1cb5252687d2e27c149bac9_routeScrollAreaPage874a609d0c0bd278f71a646f9963852f = makeLoad("/dx-components/assets/module_34_routeScrollAreaPage874a609d0c0bd278f71a646f9963852f-dxh1c9f421cdd2cd53.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPage3dc397400a141cb582ef3d1606cfd1f3_5b06792c3f6a5b2723800ef0cb601624_routeSelectPage3dc397400a141cb582ef3d1606cfd1f3 = makeLoad("/dx-components/assets/module_35_routeSelectPage3dc397400a141cb582ef3d1606cfd1f3-dxh3f706e42257848dd.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPage87a3fdffd96c0e79b30980f6824596df_bf81612e01dea9b5df26ef84cc2fef23_routeSeparatorPage87a3fdffd96c0e79b30980f6824596df = makeLoad("/dx-components/assets/module_36_routeSeparatorPage87a3fdffd96c0e79b30980f6824596df-dxhbe992c1c8f61fc1.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPage17760410515e2257df29c6b7e05d90e4_c918fd4192a7a2754647de5b88543e23_routeSheetPage17760410515e2257df29c6b7e05d90e4 = makeLoad("/dx-components/assets/module_37_routeSheetPage17760410515e2257df29c6b7e05d90e4-dxha28fd1cb53341c53.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPagee53ee2071bdeafb27806e640da5f0c83_61f7d3ffb2b4d01222135904d7e3794e_routeSidebarPagee53ee2071bdeafb27806e640da5f0c83 = makeLoad("/dx-components/assets/module_38_routeSidebarPagee53ee2071bdeafb27806e640da5f0c83-dxh6565c811fe96ec7a.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPage8a1bb0462eb6fad47d85771af39e0e76_f14803d163032f692ef06c92689b98b4_routeSkeletonPage8a1bb0462eb6fad47d85771af39e0e76 = makeLoad("/dx-components/assets/module_39_routeSkeletonPage8a1bb0462eb6fad47d85771af39e0e76-dxh9ac01717b35c29ca.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPage83d15b78e5b92a3c0a9452aeac1aae62_20e6565bef9d391a0e3c9ba4767f37a1_routeSliderPage83d15b78e5b92a3c0a9452aeac1aae62 = makeLoad("/dx-components/assets/module_40_routeSliderPage83d15b78e5b92a3c0a9452aeac1aae62-dxh7b3197ea92f79f.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPage71cb446b73fd53b93c969cd864a1521b_b7191776640ab2c6a12b2cefa9412ffc_routeSwitchPage71cb446b73fd53b93c969cd864a1521b = makeLoad("/dx-components/assets/module_41_routeSwitchPage71cb446b73fd53b93c969cd864a1521b-dxh4ba2b6b283dba272.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPagedc35415eecc9c841d12853b4fca3a473_d1aa2493214247d4983b66c7e10f3c42_routeTabsPagedc35415eecc9c841d12853b4fca3a473 = makeLoad("/dx-components/assets/module_42_routeTabsPagedc35415eecc9c841d12853b4fca3a473-dxh287dae215198c3.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPaged80fcb03338b8340a69b36da01431aaa_a4c8189984f29ba67daf1a9157605d13_routeTextareaPaged80fcb03338b8340a69b36da01431aaa = makeLoad("/dx-components/assets/module_43_routeTextareaPaged80fcb03338b8340a69b36da01431aaa-dxhed3bbbe621b17dad.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPagea90ec60e26b0128030b7351ffda81b3d_003bf39ab4a5747ad8262b8c552109ab_routeToastPagea90ec60e26b0128030b7351ffda81b3d = makeLoad("/dx-components/assets/module_44_routeToastPagea90ec60e26b0128030b7351ffda81b3d-dxh7c8ebefedba8c6f3.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPage45a74dec555aaa78611a8ecad233d28a_d979497dd12d7883056a35090a05394f_routeToggleGroupPage45a74dec555aaa78611a8ecad233d28a = makeLoad("/dx-components/assets/module_45_routeToggleGroupPage45a74dec555aaa78611a8ecad233d28a-dxh164b7fdce7aba0a.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePage590f43fe436aece6ed4ada5acecf2467_93fafa11deb10b9024ddb7a00b1d3d16_routeTogglePage590f43fe436aece6ed4ada5acecf2467 = makeLoad("/dx-components/assets/module_46_routeTogglePage590f43fe436aece6ed4ada5acecf2467-dxh52a3c72ce773c3ae.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPage1ad1cace04a709370da6f49a05f4c8ca_ca0f1e461bf699987a30f6fba4a363de_routeToolbarPage1ad1cace04a709370da6f49a05f4c8ca = makeLoad("/dx-components/assets/module_47_routeToolbarPage1ad1cace04a709370da6f49a05f4c8ca-dxha07da298ba22b060.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPage0937c413b3544410c049bfe25a77f189_877e5df3e9b474f0472598b6677b5dec_routeTooltipPage0937c413b3544410c049bfe25a77f189 = makeLoad("/dx-components/assets/module_48_routeTooltipPage0937c413b3544410c049bfe25a77f189-dxhc1f4088c2b222b9.wasm", [], fusedImports);
