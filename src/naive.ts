import { create, NButton, NCard, NForm, NFormItem, NGi, NGrid, NModal, NNotificationProvider, NSelect, NSpace, NSwitch, NTabPane, NTabs, NTag } from 'naive-ui'

export function createNaiveUi() {
  return create({ components: [
    NButton, NCard, NCard, NSwitch, NGi, NGrid, NSpace, NSelect,
    NFormItem, NForm, NTabPane, NTabs, NNotificationProvider, NTag,
    NModal
  ] })
}
