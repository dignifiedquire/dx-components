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
export const __wasm_split_load_moduleAccordionPagecb8f3e3c7a963c474a61adb537ecfaae_d2bb2ab209282cd998bb50347bed0497_routeAccordionPagecb8f3e3c7a963c474a61adb537ecfaae = makeLoad("/dx-components/assets/module_0_routeAccordionPagecb8f3e3c7a963c474a61adb537ecfaae-dxh61ee88da08e777.wasm", [], fusedImports);
export const __wasm_split_load_moduleAlertDialogPage54f04b5883e4e52847e2996a3a2822a1_2a4aa16b5742b0c230d32839a6049bb9_routeAlertDialogPage54f04b5883e4e52847e2996a3a2822a1 = makeLoad("/dx-components/assets/module_1_routeAlertDialogPage54f04b5883e4e52847e2996a3a2822a1-dxh37d828a798e54f69.wasm", [], fusedImports);
export const __wasm_split_load_moduleAspectRatioPage0d77a3d9940bcc934ad6783311099c06_401686239e167b4676bc3c1d9ca0ac6f_routeAspectRatioPage0d77a3d9940bcc934ad6783311099c06 = makeLoad("/dx-components/assets/module_2_routeAspectRatioPage0d77a3d9940bcc934ad6783311099c06-dxh1f9c8597bda2d916.wasm", [], fusedImports);
export const __wasm_split_load_moduleAvatarPage2b6fc1605f82cdb46bfc2eaa789e86f8_611a1f42c0f4560459e4bed5f135c127_routeAvatarPage2b6fc1605f82cdb46bfc2eaa789e86f8 = makeLoad("/dx-components/assets/module_3_routeAvatarPage2b6fc1605f82cdb46bfc2eaa789e86f8-dxhb5e773275172cadf.wasm", [], fusedImports);
export const __wasm_split_load_moduleBadgePaged3f3146088ce435da096418611b8d79e_37bd2ca9f0180a8922ff71d28a7bd522_routeBadgePaged3f3146088ce435da096418611b8d79e = makeLoad("/dx-components/assets/module_4_routeBadgePaged3f3146088ce435da096418611b8d79e-dxhf61f8ee892b45df.wasm", [], fusedImports);
export const __wasm_split_load_moduleButtonPaged8fa98e0b2cb2203f8e30da752ef551d_08c37a4bafe306badf5edb94450fac21_routeButtonPaged8fa98e0b2cb2203f8e30da752ef551d = makeLoad("/dx-components/assets/module_5_routeButtonPaged8fa98e0b2cb2203f8e30da752ef551d-dxhe24d1e648e3bb072.wasm", [], fusedImports);
export const __wasm_split_load_moduleCalendarPageace182f4e83d2ca82541b49d3f693988_10bee68bf4256dffe5328fcc663b5f8b_routeCalendarPageace182f4e83d2ca82541b49d3f693988 = makeLoad("/dx-components/assets/module_6_routeCalendarPageace182f4e83d2ca82541b49d3f693988-dxh9a33c1ae4ce7ee8f.wasm", [], fusedImports);
export const __wasm_split_load_moduleCardPage849192316785a841d81eb9c358534161_d8188ec3205fbc24b9a3477cdc226e43_routeCardPage849192316785a841d81eb9c358534161 = makeLoad("/dx-components/assets/module_7_routeCardPage849192316785a841d81eb9c358534161-dxh379ac7a2babac9.wasm", [], fusedImports);
export const __wasm_split_load_moduleCarouselPage82229703aab58a45ee51757555ed87e0_f3fd27ce6c76e501f0b33e1cbe3fcdbc_routeCarouselPage82229703aab58a45ee51757555ed87e0 = makeLoad("/dx-components/assets/module_8_routeCarouselPage82229703aab58a45ee51757555ed87e0-dxh15b5d2c59546b1bf.wasm", [], fusedImports);
export const __wasm_split_load_moduleCheckboxPagec29ee664525e4549d83de79d6960aa9e_78b0b03b825024436e0352889a534694_routeCheckboxPagec29ee664525e4549d83de79d6960aa9e = makeLoad("/dx-components/assets/module_9_routeCheckboxPagec29ee664525e4549d83de79d6960aa9e-dxhb822ce683f4f9dfc.wasm", [], fusedImports);
export const __wasm_split_load_moduleCollapsiblePage4d318181e2abc4264cfcc8b90be9e3dc_f3bcf1b38e69e136566b527a84322a73_routeCollapsiblePage4d318181e2abc4264cfcc8b90be9e3dc = makeLoad("/dx-components/assets/module_10_routeCollapsiblePage4d318181e2abc4264cfcc8b90be9e3dc-dxh76cf4fa33d1f6dcf.wasm", [], fusedImports);
export const __wasm_split_load_moduleComboboxPagec66f8cb1c0a40db4c1260210b692db0a_a13370c6d827966088dcb876a8145a9f_routeComboboxPagec66f8cb1c0a40db4c1260210b692db0a = makeLoad("/dx-components/assets/module_11_routeComboboxPagec66f8cb1c0a40db4c1260210b692db0a-dxh438f96d575d6cfcc.wasm", [], fusedImports);
export const __wasm_split_load_moduleCommandPagefbd9e3f2bf11094183215a1a385b2a5f_cf7359c8b3581b0aa8c04b08a1b17862_routeCommandPagefbd9e3f2bf11094183215a1a385b2a5f = makeLoad("/dx-components/assets/module_12_routeCommandPagefbd9e3f2bf11094183215a1a385b2a5f-dxh66d748b0cfad2c65.wasm", [], fusedImports);
export const __wasm_split_load_moduleComponentBlockDemoeef9e07eee9286c0b3d51d1320564876_735c88c996fc9d3bce42b50bc9cf4354_routeComponentBlockDemoeef9e07eee9286c0b3d51d1320564876 = makeLoad("/dx-components/assets/module_13_routeComponentBlockDemoeef9e07eee9286c0b3d51d1320564876-dxhbf844eddbec7379.wasm", [], fusedImports);
export const __wasm_split_load_moduleContextMenuPage56b2917125a55cc9aa1e9ce0e3a88e46_81f508064bcf740a877ce30bc9ecbb48_routeContextMenuPage56b2917125a55cc9aa1e9ce0e3a88e46 = makeLoad("/dx-components/assets/module_14_routeContextMenuPage56b2917125a55cc9aa1e9ce0e3a88e46-dxhfb697218d4511cf.wasm", [], fusedImports);
export const __wasm_split_load_moduleDatePickerPage55943de148f1903d6ab387c6dadcc085_ba272ef0269d95f03c187d7d29170be7_routeDatePickerPage55943de148f1903d6ab387c6dadcc085 = makeLoad("/dx-components/assets/module_15_routeDatePickerPage55943de148f1903d6ab387c6dadcc085-dxh40938a2b1bf60a2.wasm", [], fusedImports);
export const __wasm_split_load_moduleDialogPage25b913e01617b7fb37be4e703452a579_4a8ed117d9bb8633dba13b689be81ae9_routeDialogPage25b913e01617b7fb37be4e703452a579 = makeLoad("/dx-components/assets/module_16_routeDialogPage25b913e01617b7fb37be4e703452a579-dxh7234fc07dee6cd5.wasm", [], fusedImports);
export const __wasm_split_load_moduleDragAndDropListPage39711f43206c65fc45e6bc662889bd10_408ea0c8ac2bb7adbdafa404253df147_routeDragAndDropListPage39711f43206c65fc45e6bc662889bd10 = makeLoad("/dx-components/assets/module_17_routeDragAndDropListPage39711f43206c65fc45e6bc662889bd10-dxhd0ce13f7c5afc3c.wasm", [], fusedImports);
export const __wasm_split_load_moduleDrawerPage66b514a8e0c6cd3e1ebb8d450a1f0406_1e47f2f58f7c08d2a52b256fa242b46f_routeDrawerPage66b514a8e0c6cd3e1ebb8d450a1f0406 = makeLoad("/dx-components/assets/module_18_routeDrawerPage66b514a8e0c6cd3e1ebb8d450a1f0406-dxhaf39b796a8367c2.wasm", [], fusedImports);
export const __wasm_split_load_moduleDropdownMenuPagefb7b2ae3744ea6c5fd14883c6d797f02_e7962639e6841118108e8f7f3aaea260_routeDropdownMenuPagefb7b2ae3744ea6c5fd14883c6d797f02 = makeLoad("/dx-components/assets/module_19_routeDropdownMenuPagefb7b2ae3744ea6c5fd14883c6d797f02-dxhf67cd31b83577346.wasm", [], fusedImports);
export const __wasm_split_load_moduleFormPageca3a19dc7d9c0d374c82bfd2957371a4_7100626429bfc272135fc2994fa92d39_routeFormPageca3a19dc7d9c0d374c82bfd2957371a4 = makeLoad("/dx-components/assets/module_20_routeFormPageca3a19dc7d9c0d374c82bfd2957371a4-dxhda71e711285b339.wasm", [], fusedImports);
export const __wasm_split_load_moduleHomee7a87df0a6d867dc64110b155bb2f251_725cc1244df4bd01dabb029c4f4d94b5_routeHomee7a87df0a6d867dc64110b155bb2f251 = makeLoad("/dx-components/assets/module_21_routeHomee7a87df0a6d867dc64110b155bb2f251-dxha7ad98e6c59f1989.wasm", [], fusedImports);
export const __wasm_split_load_moduleHoverCardPagecfc098b80add6dfe14e7f32147a3109c_4d51234af26fb614981b71df298ba639_routeHoverCardPagecfc098b80add6dfe14e7f32147a3109c = makeLoad("/dx-components/assets/module_22_routeHoverCardPagecfc098b80add6dfe14e7f32147a3109c-dxh6c54f1f08155d9b.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputOtpPage09a39911765e6e4ccd5c449fad9804ff_4372b706ebbaf6361eea659862027943_routeInputOtpPage09a39911765e6e4ccd5c449fad9804ff = makeLoad("/dx-components/assets/module_23_routeInputOtpPage09a39911765e6e4ccd5c449fad9804ff-dxh98b5367dba956f59.wasm", [], fusedImports);
export const __wasm_split_load_moduleInputPageebb3d92e6b0d959e9d188e6e1fcb4a82_fea6b9afcef3e52d393489a36d89e8a0_routeInputPageebb3d92e6b0d959e9d188e6e1fcb4a82 = makeLoad("/dx-components/assets/module_24_routeInputPageebb3d92e6b0d959e9d188e6e1fcb4a82-dxh2e55779f8dfd98bc.wasm", [], fusedImports);
export const __wasm_split_load_moduleLabelPage7c683b62016d49a74528c35adca20811_b8d52da67919d052909dbdb210784e9d_routeLabelPage7c683b62016d49a74528c35adca20811 = makeLoad("/dx-components/assets/module_25_routeLabelPage7c683b62016d49a74528c35adca20811-dxh2f65311188274899.wasm", [], fusedImports);
export const __wasm_split_load_moduleMenubarPagea4bb3234a6b426ded5eac0146bb6bff9_873d1dcd6d6401e8d504e6ea8eec74b4_routeMenubarPagea4bb3234a6b426ded5eac0146bb6bff9 = makeLoad("/dx-components/assets/module_26_routeMenubarPagea4bb3234a6b426ded5eac0146bb6bff9-dxh4e6e0cdf674eeb6.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavbarPage804789e4d4091d860972eeefc3af3cbf_9ce5d09a8c590b2311e6047dc8fe1c4d_routeNavbarPage804789e4d4091d860972eeefc3af3cbf = makeLoad("/dx-components/assets/module_27_routeNavbarPage804789e4d4091d860972eeefc3af3cbf-dxhd9b45a8694aedd40.wasm", [], fusedImports);
export const __wasm_split_load_moduleNavigationMenuPage15b270cb5aff4980a6ff6b24d2776a43_f2c1f015f080164b7c746cf661292357_routeNavigationMenuPage15b270cb5aff4980a6ff6b24d2776a43 = makeLoad("/dx-components/assets/module_28_routeNavigationMenuPage15b270cb5aff4980a6ff6b24d2776a43-dxh93926ccf1fc6fda8.wasm", [], fusedImports);
export const __wasm_split_load_modulePaginationPage20ee2929979495d9e843dd638dfbcaba_64805b421de943efacf36df2e0e4e04b_routePaginationPage20ee2929979495d9e843dd638dfbcaba = makeLoad("/dx-components/assets/module_29_routePaginationPage20ee2929979495d9e843dd638dfbcaba-dxhfea5a075d5674b35.wasm", [], fusedImports);
export const __wasm_split_load_modulePopoverPage0d70055e69e9202b23646439d2795eee_df93f6aa6b1d354ecddf312f2a16f6d1_routePopoverPage0d70055e69e9202b23646439d2795eee = makeLoad("/dx-components/assets/module_30_routePopoverPage0d70055e69e9202b23646439d2795eee-dxh7afb4a521ba7368.wasm", [], fusedImports);
export const __wasm_split_load_moduleProgressPage6fadf9ad36decdc354bd91bd91be678c_8c1f6df5ff818f936c5d74dbc6983508_routeProgressPage6fadf9ad36decdc354bd91bd91be678c = makeLoad("/dx-components/assets/module_31_routeProgressPage6fadf9ad36decdc354bd91bd91be678c-dxh7dadae8fc6bfe44f.wasm", [], fusedImports);
export const __wasm_split_load_moduleRadioGroupPage6d957b9872196ce6716cccdafd09e85a_165f62d305bfdf57ba847803e8338bbe_routeRadioGroupPage6d957b9872196ce6716cccdafd09e85a = makeLoad("/dx-components/assets/module_32_routeRadioGroupPage6d957b9872196ce6716cccdafd09e85a-dxh88e11ea64a6b60bf.wasm", [], fusedImports);
export const __wasm_split_load_moduleResizablePage8c2add3d300fdd3fd6984599cbc46f9c_01bae7115cda65a564dd71c3010a3293_routeResizablePage8c2add3d300fdd3fd6984599cbc46f9c = makeLoad("/dx-components/assets/module_33_routeResizablePage8c2add3d300fdd3fd6984599cbc46f9c-dxhcf9c6e3d19b9c36.wasm", [], fusedImports);
export const __wasm_split_load_moduleScrollAreaPage77be9ce8d110907239be6f90ffe863a1_3d4ba91cfc400ff83676435891298373_routeScrollAreaPage77be9ce8d110907239be6f90ffe863a1 = makeLoad("/dx-components/assets/module_34_routeScrollAreaPage77be9ce8d110907239be6f90ffe863a1-dxh587fe2db1621c542.wasm", [], fusedImports);
export const __wasm_split_load_moduleSelectPagecf71009dcd277a9865dcbdbff506e323_af21b09c5479ec1d0b4526dd3fd056aa_routeSelectPagecf71009dcd277a9865dcbdbff506e323 = makeLoad("/dx-components/assets/module_35_routeSelectPagecf71009dcd277a9865dcbdbff506e323-dxhe0c75b45a481ddf2.wasm", [], fusedImports);
export const __wasm_split_load_moduleSeparatorPagef7c520e0600e0527bff9bde6c2777bf6_ce99ab5326775d0baad3d98ca710fa26_routeSeparatorPagef7c520e0600e0527bff9bde6c2777bf6 = makeLoad("/dx-components/assets/module_36_routeSeparatorPagef7c520e0600e0527bff9bde6c2777bf6-dxh5241fd9a147f46e.wasm", [], fusedImports);
export const __wasm_split_load_moduleSheetPagedfe5f675b81f13dad5595ef8c77ecf8f_1234b9629b0ae7d6aaa86b4ec63c5119_routeSheetPagedfe5f675b81f13dad5595ef8c77ecf8f = makeLoad("/dx-components/assets/module_37_routeSheetPagedfe5f675b81f13dad5595ef8c77ecf8f-dxhbbe4851f84a53c4a.wasm", [], fusedImports);
export const __wasm_split_load_moduleSidebarPage5228ecb2137be5478483c62482fa9344_838a18097ff06bcfce8592516cd07183_routeSidebarPage5228ecb2137be5478483c62482fa9344 = makeLoad("/dx-components/assets/module_38_routeSidebarPage5228ecb2137be5478483c62482fa9344-dxh533d1f921aa0f8a5.wasm", [], fusedImports);
export const __wasm_split_load_moduleSkeletonPage91c46a0e2aac16cd07ee0bcb64ba3648_300c9dc469518e55d177270f1843b0e7_routeSkeletonPage91c46a0e2aac16cd07ee0bcb64ba3648 = makeLoad("/dx-components/assets/module_39_routeSkeletonPage91c46a0e2aac16cd07ee0bcb64ba3648-dxhdcb6b91fdcd976.wasm", [], fusedImports);
export const __wasm_split_load_moduleSliderPagebac5eb3e197a40f08f213dbd9a30059a_dd033a49e7402399ddc19c973bf4dbf5_routeSliderPagebac5eb3e197a40f08f213dbd9a30059a = makeLoad("/dx-components/assets/module_40_routeSliderPagebac5eb3e197a40f08f213dbd9a30059a-dxhd3ae4b5b4785f9c2.wasm", [], fusedImports);
export const __wasm_split_load_moduleSwitchPagee4ae73844c56e08b5a1ce4cc93080869_18ca40bf52507c5d2e7a5021d7f9540a_routeSwitchPagee4ae73844c56e08b5a1ce4cc93080869 = makeLoad("/dx-components/assets/module_41_routeSwitchPagee4ae73844c56e08b5a1ce4cc93080869-dxh2ce4fda5334283b8.wasm", [], fusedImports);
export const __wasm_split_load_moduleTabsPage6c9e3b0226a3c6900752862bd426c59c_b99dd7169d62fcee2c870fc40fd30d28_routeTabsPage6c9e3b0226a3c6900752862bd426c59c = makeLoad("/dx-components/assets/module_42_routeTabsPage6c9e3b0226a3c6900752862bd426c59c-dxh30908f5b7ebfb97.wasm", [], fusedImports);
export const __wasm_split_load_moduleTextareaPage102231cf62fd91bdbbad72109cf590ac_894855d0e59d958ae90259ed7fe062b1_routeTextareaPage102231cf62fd91bdbbad72109cf590ac = makeLoad("/dx-components/assets/module_43_routeTextareaPage102231cf62fd91bdbbad72109cf590ac-dxh3aa8b6ae93ef813b.wasm", [], fusedImports);
export const __wasm_split_load_moduleToastPage4d68494868b8c376f4f5928cd2c6bd90_1bff386fd68e0a3928c433797331a39e_routeToastPage4d68494868b8c376f4f5928cd2c6bd90 = makeLoad("/dx-components/assets/module_44_routeToastPage4d68494868b8c376f4f5928cd2c6bd90-dxhb75126712aa42d90.wasm", [], fusedImports);
export const __wasm_split_load_moduleToggleGroupPaged76a204c3972e51a29776be4c8913b0f_0b093f2adbf39a6be25c92f4858c8b9d_routeToggleGroupPaged76a204c3972e51a29776be4c8913b0f = makeLoad("/dx-components/assets/module_45_routeToggleGroupPaged76a204c3972e51a29776be4c8913b0f-dxh21a19f3221491f.wasm", [], fusedImports);
export const __wasm_split_load_moduleTogglePage27ba8b625ace17523d954a64db17dd73_6b203285d23adc9ecf4baa191b2de2e3_routeTogglePage27ba8b625ace17523d954a64db17dd73 = makeLoad("/dx-components/assets/module_46_routeTogglePage27ba8b625ace17523d954a64db17dd73-dxhc7b66db07ab6a22c.wasm", [], fusedImports);
export const __wasm_split_load_moduleToolbarPage6f30282d234c4bfb1d2fb5d59b77af09_418dc38f51c84194d908c34a931907ab_routeToolbarPage6f30282d234c4bfb1d2fb5d59b77af09 = makeLoad("/dx-components/assets/module_47_routeToolbarPage6f30282d234c4bfb1d2fb5d59b77af09-dxh9d7eea2053c8bdf0.wasm", [], fusedImports);
export const __wasm_split_load_moduleTooltipPage4b11b259ee6d105b37b9c52efe687a83_86c8f36903101a067f591fa6215eab19_routeTooltipPage4b11b259ee6d105b37b9c52efe687a83 = makeLoad("/dx-components/assets/module_48_routeTooltipPage4b11b259ee6d105b37b9c52efe687a83-dxh7b32eada54cf1dc2.wasm", [], fusedImports);
