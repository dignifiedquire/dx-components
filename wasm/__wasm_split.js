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
export const __wasm_split_load_moduleAccordionPage81f9d48b7f6daf061414f3622603601d_a98a4e5eedc8d80a278bfed505a2c271_routeAccordionPage81f9d48b7f6daf061414f3622603601d = makeLoad("/dx-components/assets/module_0_routeAccordionPage81f9d48b7f6daf061414f3622603601d-dxh6279dea556b598dd.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPagef883aa481a46f297adc63d6bdb402a72_5e16ed803a6557f4177c597488980625_routeAlertDialogPagef883aa481a46f297adc63d6bdb402a72 = makeLoad("/dx-components/assets/module_1_routeAlertDialogPagef883aa481a46f297adc63d6bdb402a72-dxh28e579f5f644ef.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPage6e9c76eef6346e86681ac4e664809fea_8e10d82e2f92175fb39b162dc8a79ac5_routeAspectRatioPage6e9c76eef6346e86681ac4e664809fea = makeLoad("/dx-components/assets/module_2_routeAspectRatioPage6e9c76eef6346e86681ac4e664809fea-dxha1c5ff4278972368.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPage91d94f3931e5b0352123ac6f8f2fa86b_65048688514063cce6efd32b2232f9b2_routeAvatarPage91d94f3931e5b0352123ac6f8f2fa86b = makeLoad("/dx-components/assets/module_3_routeAvatarPage91d94f3931e5b0352123ac6f8f2fa86b-dxhb8416d1abe51476.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePage26c1576436d40b72e9e8031b1b162fb4_64a7178a76977cf9d7834e54b41c5c66_routeBadgePage26c1576436d40b72e9e8031b1b162fb4 = makeLoad("/dx-components/assets/module_4_routeBadgePage26c1576436d40b72e9e8031b1b162fb4-dxh7db97f48a98a4eac.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPagef73ed3fe1b698377d06eaa31255bde97_03c48283a996203d1417764982d3dc26_routeButtonPagef73ed3fe1b698377d06eaa31255bde97 = makeLoad("/dx-components/assets/module_5_routeButtonPagef73ed3fe1b698377d06eaa31255bde97-dxha080a0c756d21779.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPagefde93d88a1529b6e4af5c9b2b1b3e027_1ace36d40ee5d5a8875641100bf590d7_routeCalendarPagefde93d88a1529b6e4af5c9b2b1b3e027 = makeLoad("/dx-components/assets/module_6_routeCalendarPagefde93d88a1529b6e4af5c9b2b1b3e027-dxha4bf712dc6d0b789.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPage41edbae3982d1314de6969e34af40595_fedce9e030fdf01f635a9c764066f2e8_routeCardPage41edbae3982d1314de6969e34af40595 = makeLoad("/dx-components/assets/module_7_routeCardPage41edbae3982d1314de6969e34af40595-dxh89b89eec18f3803d.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPage8b34f01cf8f066b55a7760f4c236b9f3_27d7706b3c28a81ac47d007b7e68ffb5_routeCarouselPage8b34f01cf8f066b55a7760f4c236b9f3 = makeLoad("/dx-components/assets/module_8_routeCarouselPage8b34f01cf8f066b55a7760f4c236b9f3-dxh4a4b8a226223a4a.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPage56781074519c0f7d9c02706ea439df78_34e505e369467702f70be66d83cb69a0_routeCheckboxPage56781074519c0f7d9c02706ea439df78 = makeLoad("/dx-components/assets/module_9_routeCheckboxPage56781074519c0f7d9c02706ea439df78-dxh845c4b9e4e357012.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePagec41124f8515e35a666a7768bb12b1379_76336fdc8fc1dcdeb16e7a779f5d1e5e_routeCollapsiblePagec41124f8515e35a666a7768bb12b1379 = makeLoad("/dx-components/assets/module_10_routeCollapsiblePagec41124f8515e35a666a7768bb12b1379-dxhf19234873f666b1.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPage0dcb77bcf28669e0873384af01820cb1_bb70e78b67643e758356d6ffda7b10e4_routeComboboxPage0dcb77bcf28669e0873384af01820cb1 = makeLoad("/dx-components/assets/module_11_routeComboboxPage0dcb77bcf28669e0873384af01820cb1-dxh7939a38694ff67fc.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPageb133c91540b74793f360796f13be6802_8473c29c9f486e40c3c049f771ebc179_routeCommandPageb133c91540b74793f360796f13be6802 = makeLoad("/dx-components/assets/module_12_routeCommandPageb133c91540b74793f360796f13be6802-dxhb3e1e48df981a63.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemoff007053497f66c5ebe071e341a7ee0b_d22ea16b6653f5c45e67f73641f87b2a_routeComponentBlockDemoff007053497f66c5ebe071e341a7ee0b = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemoff007053497f66c5ebe071e341a7ee0b-dxh5842f6ad61d6375.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPageaba89d30a12b6bab3aa62547fa6cb4ab_d08415e87c818251d4c9386144aa83ec_routeContextMenuPageaba89d30a12b6bab3aa62547fa6cb4ab = makeLoad("/dx-components/assets/module_14_routeContextMenuPageaba89d30a12b6bab3aa62547fa6cb4ab-dxhf9f3f4a6524cef50.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPage3369bd12706b63ed2bbe9b7a20dc34b5_dd53872674f1f0cd669f5b154c4506f2_routeDatePickerPage3369bd12706b63ed2bbe9b7a20dc34b5 = makeLoad("/dx-components/assets/module_15_routeDatePickerPage3369bd12706b63ed2bbe9b7a20dc34b5-dxh8d81cf343192faf.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPage7ff282ab1d247a476773d241ec9ae66e_904f64c894bfcfaf7c679a12be1c75e5_routeDialogPage7ff282ab1d247a476773d241ec9ae66e = makeLoad("/dx-components/assets/module_16_routeDialogPage7ff282ab1d247a476773d241ec9ae66e-dxh903479587d7aa4b.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPageaf468eb5c6a89dae2038a2f7a529b396_b84e8f449992b90f54df3e7d1c1a96e9_routeDragAndDropListPageaf468eb5c6a89dae2038a2f7a529b396 = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPageaf468eb5c6a89dae2038a2f7a529b396-dxha294de5ba4e45cca.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPage7380b80a5dee676b82949d94a43afdbf_d8e4a1edc05d8e2a47a673ed680e2176_routeDrawerPage7380b80a5dee676b82949d94a43afdbf = makeLoad("/dx-components/assets/module_18_routeDrawerPage7380b80a5dee676b82949d94a43afdbf-dxha0bf778d25f2c634.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPage62f41af7cf198c48a88f9c22a6e78f78_a28a7bf7af899ef69ee5a2ee1d74f200_routeDropdownMenuPage62f41af7cf198c48a88f9c22a6e78f78 = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPage62f41af7cf198c48a88f9c22a6e78f78-dxh44501fb31e2f25d.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPage810506d4139867768276f2b56806412c_899f1d4cb31eaf07bf0913b44d21388a_routeFormPage810506d4139867768276f2b56806412c = makeLoad("/dx-components/assets/module_20_routeFormPage810506d4139867768276f2b56806412c-dxh5d57d4d664696442.wasm", [], fusedImports);
export const __wasm_split_load_moduleHomecd328d9ac69af763cb23608535aeecfd_e2bd2ba08b841f279a88840da1810697_routeHomecd328d9ac69af763cb23608535aeecfd = makeLoad("/dx-components/assets/module_21_routeHomecd328d9ac69af763cb23608535aeecfd-dxh6f9af3c56052b3c.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPage5b17eca61b1e861335e06880c1189ac5_1b050d733201092f0a58af51e1c2f318_routeHoverCardPage5b17eca61b1e861335e06880c1189ac5 = makeLoad("/dx-components/assets/module_22_routeHoverCardPage5b17eca61b1e861335e06880c1189ac5-dxhc2cf4a376728c6b0.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPageafcc8823cf1facbc70eabdb1845a77d4_7c90b548e6c11da672d1eec3c95492d7_routeInputOtpPageafcc8823cf1facbc70eabdb1845a77d4 = makeLoad("/dx-components/assets/module_23_routeInputOtpPageafcc8823cf1facbc70eabdb1845a77d4-dxh32ace1c6c310dd46.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPage030a98e7bd12e3ede43836650109f675_8a104b6218a2dc902edb57802fa9c616_routeInputPage030a98e7bd12e3ede43836650109f675 = makeLoad("/dx-components/assets/module_24_routeInputPage030a98e7bd12e3ede43836650109f675-dxh246cfc3141cc460.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPage0943170fc30de6599d4fd35580c5016d_7cbb48a695abfc059a12f5e226168582_routeLabelPage0943170fc30de6599d4fd35580c5016d = makeLoad("/dx-components/assets/module_25_routeLabelPage0943170fc30de6599d4fd35580c5016d-dxh75146b694d389fc7.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPageee15110de2145434a4df100d8d64deb2_3dd1b13660d7c3c68ea650c040ac98ad_routeMenubarPageee15110de2145434a4df100d8d64deb2 = makeLoad("/dx-components/assets/module_26_routeMenubarPageee15110de2145434a4df100d8d64deb2-dxh8a731de451e1a5e.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPage2c0a9fcd31dbf67355bc01ee2ae54ec6_6a3baf5226cd2795f285b09cc3163ee2_routeNavbarPage2c0a9fcd31dbf67355bc01ee2ae54ec6 = makeLoad("/dx-components/assets/module_27_routeNavbarPage2c0a9fcd31dbf67355bc01ee2ae54ec6-dxh13ced851b0fdf77.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPage96d804185ebf161db5d469798fc82d4e_537ea53910cac6c6e7de07289ba6e6e0_routeNavigationMenuPage96d804185ebf161db5d469798fc82d4e = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPage96d804185ebf161db5d469798fc82d4e-dxh5b2aabacdda473dd.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPage4aec1c6b9e5925811a4cb3e0c241e147_f360c156ed038eaabee407215300f097_routePaginationPage4aec1c6b9e5925811a4cb3e0c241e147 = makeLoad("/dx-components/assets/module_29_routePaginationPage4aec1c6b9e5925811a4cb3e0c241e147-dxh3712a8db64d2112.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPage0a8a3ce514c8a519c3c013fe0ab63ccd_e3a1d9496a187e9149e3390b7292ae42_routePopoverPage0a8a3ce514c8a519c3c013fe0ab63ccd = makeLoad("/dx-components/assets/module_30_routePopoverPage0a8a3ce514c8a519c3c013fe0ab63ccd-dxh9f6b67c74ec8e6dc.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPage6f042416fa3ee6c18ede3373e030d6cb_410e9be1231ac0f4dddbe061e6af5abb_routeProgressPage6f042416fa3ee6c18ede3373e030d6cb = makeLoad("/dx-components/assets/module_31_routeProgressPage6f042416fa3ee6c18ede3373e030d6cb-dxhed4e7383e21970be.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPage01915b7d0e6ac95ebe65b129415eb363_6ef432ac680da06d5f5a2363a34cb3f2_routeRadioGroupPage01915b7d0e6ac95ebe65b129415eb363 = makeLoad("/dx-components/assets/module_32_routeRadioGroupPage01915b7d0e6ac95ebe65b129415eb363-dxhf715e537ece2df67.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePagedb82d7d52120f12d2a06ff7c55b429c8_61caf61f886b9f428d57893aa7e41189_routeResizablePagedb82d7d52120f12d2a06ff7c55b429c8 = makeLoad("/dx-components/assets/module_33_routeResizablePagedb82d7d52120f12d2a06ff7c55b429c8-dxh3445468830e53db2.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPagea635c1a76a39bd05167eabae692a3a54_a4ab1de1e6ab82dc274deb9583ff8180_routeScrollAreaPagea635c1a76a39bd05167eabae692a3a54 = makeLoad("/dx-components/assets/module_34_routeScrollAreaPagea635c1a76a39bd05167eabae692a3a54-dxh737788ab53591b2.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPage24d3c764c94d530582be7500e5200f34_b05e19e14b80eea1f4e59544d6c2447c_routeSelectPage24d3c764c94d530582be7500e5200f34 = makeLoad("/dx-components/assets/module_35_routeSelectPage24d3c764c94d530582be7500e5200f34-dxhe95438719267035.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPage5592baa85a7aa0254b2b247b6acac7ee_621e20e978a47c1e521d1be38decbf14_routeSeparatorPage5592baa85a7aa0254b2b247b6acac7ee = makeLoad("/dx-components/assets/module_36_routeSeparatorPage5592baa85a7aa0254b2b247b6acac7ee-dxh5b5f81aa7885e14.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPage1e5e72c8c3ed1df1d90eefc88070d1e3_e4d0aefbef4536b9fc33db10135a7c3e_routeSheetPage1e5e72c8c3ed1df1d90eefc88070d1e3 = makeLoad("/dx-components/assets/module_37_routeSheetPage1e5e72c8c3ed1df1d90eefc88070d1e3-dxh637675ffdb89d6ea.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPagec60289c3e79c7c1d5e843b6094c892fb_e34b82553f7696193f50e34a994a38ca_routeSidebarPagec60289c3e79c7c1d5e843b6094c892fb = makeLoad("/dx-components/assets/module_38_routeSidebarPagec60289c3e79c7c1d5e843b6094c892fb-dxh27172a5f477b050.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPage5abdf0e6fc23c07ff9a24825aa5515dc_e79cce8103d885a9ecd91b25d7e68904_routeSkeletonPage5abdf0e6fc23c07ff9a24825aa5515dc = makeLoad("/dx-components/assets/module_39_routeSkeletonPage5abdf0e6fc23c07ff9a24825aa5515dc-dxh1ba774df22f2ed68.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPage8e60dc68c4572812906b16459d0ab07e_4f8b0e614c12ae4723cd5631fad2370c_routeSliderPage8e60dc68c4572812906b16459d0ab07e = makeLoad("/dx-components/assets/module_40_routeSliderPage8e60dc68c4572812906b16459d0ab07e-dxh6dafb3aaf84e1d4c.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPageea90d8c943d77db0d4cd94122a3a2a01_162faf241aa6f3c1ec993230b192efe1_routeSwitchPageea90d8c943d77db0d4cd94122a3a2a01 = makeLoad("/dx-components/assets/module_41_routeSwitchPageea90d8c943d77db0d4cd94122a3a2a01-dxha45d13fcd072cffa.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPage13fe0185d1cfb33444e80aea7ff20fb0_569f2ecad87fb9124701e6d2ad240b4b_routeTabsPage13fe0185d1cfb33444e80aea7ff20fb0 = makeLoad("/dx-components/assets/module_42_routeTabsPage13fe0185d1cfb33444e80aea7ff20fb0-dxhb87d36c2d32430fa.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPage80acb3fa60903a4eefa2f3f918b370e5_f2b10bfda576ed391b420d7b8ece9013_routeTextareaPage80acb3fa60903a4eefa2f3f918b370e5 = makeLoad("/dx-components/assets/module_43_routeTextareaPage80acb3fa60903a4eefa2f3f918b370e5-dxh47b89d9fb8f136e9.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPagefb5274d7e5c63697ae541578fe5e0ee0_a4e0807c04b5fdc6ca6a3f0a96015b70_routeToastPagefb5274d7e5c63697ae541578fe5e0ee0 = makeLoad("/dx-components/assets/module_44_routeToastPagefb5274d7e5c63697ae541578fe5e0ee0-dxhb82b6c4b53387050.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPage38fc1d3350d5fa647891ebcdaaf439cc_154497df082fa60ae9f210f86be15541_routeToggleGroupPage38fc1d3350d5fa647891ebcdaaf439cc = makeLoad("/dx-components/assets/module_45_routeToggleGroupPage38fc1d3350d5fa647891ebcdaaf439cc-dxh954529671f4c198f.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePage828024e0b9007462764e0a3f1ab7c8c3_5c2fbf7a9bc5620901c0f416e9d9908a_routeTogglePage828024e0b9007462764e0a3f1ab7c8c3 = makeLoad("/dx-components/assets/module_46_routeTogglePage828024e0b9007462764e0a3f1ab7c8c3-dxh718dde1f9bb21c98.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPagefbdc89fcf9f7a4992582d890c68d5797_3b376704e535ad619969b2202fe588a3_routeToolbarPagefbdc89fcf9f7a4992582d890c68d5797 = makeLoad("/dx-components/assets/module_47_routeToolbarPagefbdc89fcf9f7a4992582d890c68d5797-dxhcb1055afb0343e2.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPage43be70b134f8a46f6b957470c2ac1bb9_873b3f7a76bf6b327b28b7db490f763e_routeTooltipPage43be70b134f8a46f6b957470c2ac1bb9 = makeLoad("/dx-components/assets/module_48_routeTooltipPage43be70b134f8a46f6b957470c2ac1bb9-dxh20f72842809c17b4.wasm", [], fusedImports);
