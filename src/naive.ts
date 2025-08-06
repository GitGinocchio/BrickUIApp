import { create, NButton, NCard } from 'naive-ui'

export function createNaiveUi() {
  return create({ components: [NButton, NCard] })
}
