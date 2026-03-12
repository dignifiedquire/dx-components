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
export const __wasm_split_load_moduleAccordionPageec9d7324ec2aae791cd13dfc9ab3ae87_6f8195b10e58e512e364255efb503489_routeAccordionPageec9d7324ec2aae791cd13dfc9ab3ae87 = makeLoad("/dx-components/assets/module_0_routeAccordionPageec9d7324ec2aae791cd13dfc9ab3ae87-dxh839a2f91dc6b61b7.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPage330e29299bb1fda1b8fb4dcd040a7336_fbda50eb8a134bf8d5efcffd37a197e6_routeAlertDialogPage330e29299bb1fda1b8fb4dcd040a7336 = makeLoad("/dx-components/assets/module_1_routeAlertDialogPage330e29299bb1fda1b8fb4dcd040a7336-dxh67b7e3ab908c16f9.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPage25200d9b1aaa87e53806ce62f749657d_fff67018da6b4efce25738e8fcac24fc_routeAspectRatioPage25200d9b1aaa87e53806ce62f749657d = makeLoad("/dx-components/assets/module_2_routeAspectRatioPage25200d9b1aaa87e53806ce62f749657d-dxh5be18899a979e922.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPageefe3ca438c0a2130b31b1cfc3c0f3562_6528983f6bac95113658790cda96b378_routeAvatarPageefe3ca438c0a2130b31b1cfc3c0f3562 = makeLoad("/dx-components/assets/module_3_routeAvatarPageefe3ca438c0a2130b31b1cfc3c0f3562-dxh44b019bd9245eb38.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePagedeafc00e7b47266d9a91fcc8c8bb85ea_a6164ca9b276624b5ab2170b84b31a91_routeBadgePagedeafc00e7b47266d9a91fcc8c8bb85ea = makeLoad("/dx-components/assets/module_4_routeBadgePagedeafc00e7b47266d9a91fcc8c8bb85ea-dxhc1704ed69b9870ae.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPage0fe7d27dac94ceb3f5d1793473bfc817_c6572426a1503d7e08dc5612db74f064_routeButtonPage0fe7d27dac94ceb3f5d1793473bfc817 = makeLoad("/dx-components/assets/module_5_routeButtonPage0fe7d27dac94ceb3f5d1793473bfc817-dxh4088b3befdf51aae.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPage7d4d08ec29d56ad661f9a2ad11e15f74_fbd1f2cad8695af20de43646f800f523_routeCalendarPage7d4d08ec29d56ad661f9a2ad11e15f74 = makeLoad("/dx-components/assets/module_6_routeCalendarPage7d4d08ec29d56ad661f9a2ad11e15f74-dxh6e5e77f0349f9c6b.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPage399ccba6f3bdc84cd3478ac45b6d5385_eb23444797e2e59cf0eb07eceb252fc1_routeCardPage399ccba6f3bdc84cd3478ac45b6d5385 = makeLoad("/dx-components/assets/module_7_routeCardPage399ccba6f3bdc84cd3478ac45b6d5385-dxhe7f921a28f3f849f.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPage3fbfc37b5d9326a4029f8328a12d2e6c_83c2a2cb7f4e37c31349be206bca54e7_routeCarouselPage3fbfc37b5d9326a4029f8328a12d2e6c = makeLoad("/dx-components/assets/module_8_routeCarouselPage3fbfc37b5d9326a4029f8328a12d2e6c-dxhdc68a2906a204f2.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPage5ed522c40c910c32fe284703da709f31_ec627e097fdfa0b22792ef945931f9f4_routeCheckboxPage5ed522c40c910c32fe284703da709f31 = makeLoad("/dx-components/assets/module_9_routeCheckboxPage5ed522c40c910c32fe284703da709f31-dxhd8709426aef95770.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePage63b3e0fed95373a45c977679b44fc6f1_db76a7404e67d0bf786326bfd8eab0c5_routeCollapsiblePage63b3e0fed95373a45c977679b44fc6f1 = makeLoad("/dx-components/assets/module_10_routeCollapsiblePage63b3e0fed95373a45c977679b44fc6f1-dxhcc7a5bb691eb5d2f.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPage3be3be023367401ddf487bbb8697bb89_a1d2d9d26593db05e2cab2c6f5a002e0_routeComboboxPage3be3be023367401ddf487bbb8697bb89 = makeLoad("/dx-components/assets/module_11_routeComboboxPage3be3be023367401ddf487bbb8697bb89-dxh26ba30cae3383014.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPage4203099da948ec60380adf51703e6777_64de8be3e248b61ec88c1da72aca3f28_routeCommandPage4203099da948ec60380adf51703e6777 = makeLoad("/dx-components/assets/module_12_routeCommandPage4203099da948ec60380adf51703e6777-dxheabc56622d68c3.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemo326ea6bb058882a9e3c67b11ad5e4381_4a2819f07ac411d3741449074349d7c1_routeComponentBlockDemo326ea6bb058882a9e3c67b11ad5e4381 = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemo326ea6bb058882a9e3c67b11ad5e4381-dxhfb6b14e89f27d1f1.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPage88cd26c76e99fbc3aa688c16b25ccf82_435c585256d7fe2bab59944f7838baff_routeContextMenuPage88cd26c76e99fbc3aa688c16b25ccf82 = makeLoad("/dx-components/assets/module_14_routeContextMenuPage88cd26c76e99fbc3aa688c16b25ccf82-dxh60d749411d248c9.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPage4647829d6df9f937565742c20f22eff9_3af75b541f0e555345cf64fada456b0b_routeDatePickerPage4647829d6df9f937565742c20f22eff9 = makeLoad("/dx-components/assets/module_15_routeDatePickerPage4647829d6df9f937565742c20f22eff9-dxh6b932d1bb915140.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPagec1b1dd256f64aee2cf5c9dadc777d36d_07aa9722a9aa67a640e33fa2c75eca44_routeDialogPagec1b1dd256f64aee2cf5c9dadc777d36d = makeLoad("/dx-components/assets/module_16_routeDialogPagec1b1dd256f64aee2cf5c9dadc777d36d-dxhbd1afcb428fef3a6.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPage76da04b605c50eb28e62cceec4fba111_cb5f23269912c30577e6779f6c2b6592_routeDragAndDropListPage76da04b605c50eb28e62cceec4fba111 = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPage76da04b605c50eb28e62cceec4fba111-dxh232b8486a5f3f95d.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPage21d817142e53450b2f4f243781adbf8f_104d8d18c41bf65c4673a467facedba2_routeDrawerPage21d817142e53450b2f4f243781adbf8f = makeLoad("/dx-components/assets/module_18_routeDrawerPage21d817142e53450b2f4f243781adbf8f-dxh4db98b252f99023.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPaged6729406be485288124581a346376e14_c4820e4f250137b0cdbfeb587add8523_routeDropdownMenuPaged6729406be485288124581a346376e14 = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPaged6729406be485288124581a346376e14-dxh668e37f0311f80ed.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPagec2f7c08c16bef6b5a02defdad55d38fa_d8147ab33f362ea5e485a43d333c4b89_routeFormPagec2f7c08c16bef6b5a02defdad55d38fa = makeLoad("/dx-components/assets/module_20_routeFormPagec2f7c08c16bef6b5a02defdad55d38fa-dxhbf7f4b2de5d2b968.wasm", [], fusedImports);
export const __wasm_split_load_moduleHomec3a684bc8f9c04b5e401a87bae222e09_54d7f82e2b2ab78da16a4bf3d2a353b6_routeHomec3a684bc8f9c04b5e401a87bae222e09 = makeLoad("/dx-components/assets/module_21_routeHomec3a684bc8f9c04b5e401a87bae222e09-dxhcb78d5b73cf7f935.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPage0318c158fb014af499e3f116749e2c99_0205ec9d95a64cd91d4ef64d4808f340_routeHoverCardPage0318c158fb014af499e3f116749e2c99 = makeLoad("/dx-components/assets/module_22_routeHoverCardPage0318c158fb014af499e3f116749e2c99-dxh89ca1c149289d24.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPage0cef6dcb4fd937ebcf98b81f136b8db5_f2ecd39033e75870f62e28d286c812e7_routeInputOtpPage0cef6dcb4fd937ebcf98b81f136b8db5 = makeLoad("/dx-components/assets/module_23_routeInputOtpPage0cef6dcb4fd937ebcf98b81f136b8db5-dxh26b8ebb819292916.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPageb44e0f13ddb966b5e3fee304ef16f48b_e670928445c769aec925edd3ffef8fa1_routeInputPageb44e0f13ddb966b5e3fee304ef16f48b = makeLoad("/dx-components/assets/module_24_routeInputPageb44e0f13ddb966b5e3fee304ef16f48b-dxh34edcbe55a83577.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPaged5478213ba14431213bb0ff654d60a72_4448d9648384c5e14b4e84768341a751_routeLabelPaged5478213ba14431213bb0ff654d60a72 = makeLoad("/dx-components/assets/module_25_routeLabelPaged5478213ba14431213bb0ff654d60a72-dxhb4ad4f15811af6a5.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPage5aeee25c0a5fae8a5585230320740094_118bad768b290e539162b90bb71c0cf4_routeMenubarPage5aeee25c0a5fae8a5585230320740094 = makeLoad("/dx-components/assets/module_26_routeMenubarPage5aeee25c0a5fae8a5585230320740094-dxh438a130846aaa3c.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPage0ea83f2dabf3d087225273651095a8ce_985f4406f4cf5d5b8f6f93a8baac9899_routeNavbarPage0ea83f2dabf3d087225273651095a8ce = makeLoad("/dx-components/assets/module_27_routeNavbarPage0ea83f2dabf3d087225273651095a8ce-dxh7eaf9a267a74505f.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPagefd88c91d94d21b7cac0d056788a4f6db_866441ae88f14a4f67f9bdd80d3f1b96_routeNavigationMenuPagefd88c91d94d21b7cac0d056788a4f6db = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPagefd88c91d94d21b7cac0d056788a4f6db-dxhbadd82712552eb5.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPagef16a2c6f10a7863e2a6f3c342e613eb7_75c877defcc827aba5c2a74ea066a011_routePaginationPagef16a2c6f10a7863e2a6f3c342e613eb7 = makeLoad("/dx-components/assets/module_29_routePaginationPagef16a2c6f10a7863e2a6f3c342e613eb7-dxh5bdb386488b9fba1.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPage0e2dba772ccb06bc5d271228d2407837_8a98c54e7266740f04c8e0e3cfb2f9ef_routePopoverPage0e2dba772ccb06bc5d271228d2407837 = makeLoad("/dx-components/assets/module_30_routePopoverPage0e2dba772ccb06bc5d271228d2407837-dxhce22565ff96eed49.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPageae314a7caa92edbc5fd94d091d8e1882_5f9f7b223d04ab31644e5aa5b0382a29_routeProgressPageae314a7caa92edbc5fd94d091d8e1882 = makeLoad("/dx-components/assets/module_31_routeProgressPageae314a7caa92edbc5fd94d091d8e1882-dxh3caf8976f6a97c14.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPage249df8d8aa159130d1b3b957f1c56760_50e3ff6363fef8a50f9481211ae8f46b_routeRadioGroupPage249df8d8aa159130d1b3b957f1c56760 = makeLoad("/dx-components/assets/module_32_routeRadioGroupPage249df8d8aa159130d1b3b957f1c56760-dxh4ec6ff3a1d4f9198.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePagedbe462c4dc1a7e0f16515affbdc86187_c6a04ada8256d07dbbd36d2e64234356_routeResizablePagedbe462c4dc1a7e0f16515affbdc86187 = makeLoad("/dx-components/assets/module_33_routeResizablePagedbe462c4dc1a7e0f16515affbdc86187-dxhf2f4fdce5b35c9e.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPage45e2a1594aa1c0221de848fdb9f0d172_6183bbb08ffad4a9fa3fd1b00b36a771_routeScrollAreaPage45e2a1594aa1c0221de848fdb9f0d172 = makeLoad("/dx-components/assets/module_34_routeScrollAreaPage45e2a1594aa1c0221de848fdb9f0d172-dxh7da1fe17c63674be.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPage35144ef24ab40ba727421fe73d6adbfb_a623223dd3002b85d96fa773dd1abf91_routeSelectPage35144ef24ab40ba727421fe73d6adbfb = makeLoad("/dx-components/assets/module_35_routeSelectPage35144ef24ab40ba727421fe73d6adbfb-dxhf3742179fd989d0.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPageb6f6321642a763e2a10edf852b17dd62_1a7b277a4461f39c1e9b50a65a559af2_routeSeparatorPageb6f6321642a763e2a10edf852b17dd62 = makeLoad("/dx-components/assets/module_36_routeSeparatorPageb6f6321642a763e2a10edf852b17dd62-dxh69989a6f61deaf72.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPage345dd304bda4d7989e5c9a31cd8c0b68_90e9a71a0165de617e77dacc6fd84f0e_routeSheetPage345dd304bda4d7989e5c9a31cd8c0b68 = makeLoad("/dx-components/assets/module_37_routeSheetPage345dd304bda4d7989e5c9a31cd8c0b68-dxh189785849af389a.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPagebf60f871a057c2c3a3e6042d8eddaa01_967151997f6da6464ce65e593521d0f4_routeSidebarPagebf60f871a057c2c3a3e6042d8eddaa01 = makeLoad("/dx-components/assets/module_38_routeSidebarPagebf60f871a057c2c3a3e6042d8eddaa01-dxhfadbd223ebd25f3a.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPage5d40546fbefad844eab924b9fb86d5ef_695d644579c0aef9f0e5910c636cddfb_routeSkeletonPage5d40546fbefad844eab924b9fb86d5ef = makeLoad("/dx-components/assets/module_39_routeSkeletonPage5d40546fbefad844eab924b9fb86d5ef-dxh24f8c3244a7fc1dd.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPage0ecdf736b5660a87344795319b8fe0ef_34057f7833b0c57a6e8dcbfabcdf88e3_routeSliderPage0ecdf736b5660a87344795319b8fe0ef = makeLoad("/dx-components/assets/module_40_routeSliderPage0ecdf736b5660a87344795319b8fe0ef-dxh195acebd23c46f5.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPage23bf653d25a5c61a28312bd9f1a46bee_7fc60dd152fd8f4d3dfa651f096d06cc_routeSwitchPage23bf653d25a5c61a28312bd9f1a46bee = makeLoad("/dx-components/assets/module_41_routeSwitchPage23bf653d25a5c61a28312bd9f1a46bee-dxh2ea678760af4664.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPage31a45ab8d1d891efca155d00fc53ebf1_22429b3496425e51b10f8b04e22e7033_routeTabsPage31a45ab8d1d891efca155d00fc53ebf1 = makeLoad("/dx-components/assets/module_42_routeTabsPage31a45ab8d1d891efca155d00fc53ebf1-dxhb368b9ed5df7555.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPagef15235681626158df9f4636ba4218d7a_13b9045ed2d58a199ee89c8abcbd80b2_routeTextareaPagef15235681626158df9f4636ba4218d7a = makeLoad("/dx-components/assets/module_43_routeTextareaPagef15235681626158df9f4636ba4218d7a-dxh32f5f3c0e3c128e5.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPage61ab33f2b971be1e328496756c3eb110_e024dff43924fc18f718c50764ef1713_routeToastPage61ab33f2b971be1e328496756c3eb110 = makeLoad("/dx-components/assets/module_44_routeToastPage61ab33f2b971be1e328496756c3eb110-dxhf9392a966319dcc2.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPage95df79c1937b78a3a8d1a145a591f094_0c7b85eae5abc8fa14ea2f8753ceb7f0_routeToggleGroupPage95df79c1937b78a3a8d1a145a591f094 = makeLoad("/dx-components/assets/module_45_routeToggleGroupPage95df79c1937b78a3a8d1a145a591f094-dxhe7d299755c4bd0.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePagec8e66694762e14e681ad7a66145865cd_033a551ba62a355cd56837c9cc775a16_routeTogglePagec8e66694762e14e681ad7a66145865cd = makeLoad("/dx-components/assets/module_46_routeTogglePagec8e66694762e14e681ad7a66145865cd-dxh8520789b16ac1a76.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPage0187ecb7ed8aefe192d21bf1d73e9ea8_fb5227f1e9362206ab042c6ab302fbe3_routeToolbarPage0187ecb7ed8aefe192d21bf1d73e9ea8 = makeLoad("/dx-components/assets/module_47_routeToolbarPage0187ecb7ed8aefe192d21bf1d73e9ea8-dxh405bc3d0e95cf35.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPage94135a8ea309ce9eee3de9d633bfd928_8ba19429a2a7f312884a9c5eaeb4bd0b_routeTooltipPage94135a8ea309ce9eee3de9d633bfd928 = makeLoad("/dx-components/assets/module_48_routeTooltipPage94135a8ea309ce9eee3de9d633bfd928-dxh39297bf355f9777f.wasm", [], fusedImports);
