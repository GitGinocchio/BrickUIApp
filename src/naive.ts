import { create, NButton, NCard, NCollapse, NCollapseItem, NDatePicker, NForm, NFormItem, NGi, NGrid, NInputNumber, NModal, NNotificationProvider, NSelect, NSpace, NSwitch, NTabPane, NTabs, NTag, NTimePicker } from 'naive-ui'

export function createNaiveUi() {
  return create({ components: [
    NButton, NCard, NCard, NSwitch, NGi, NGrid, NSpace, NSelect,
    NFormItem, NForm, NTabPane, NTabs, NNotificationProvider, NTag,
    NModal, NDatePicker, NTimePicker, NInputNumber, NCollapse, NCollapseItem
  ] })
}
