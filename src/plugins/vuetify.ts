//import Vuetify, {
//  VBtn,
//  VCard,
//  VCardActions,
//  VCardText,
//  VCardTitle,
//  VChip,
//  VCol,
//  VContainer,
//  VIcon,
//  VList,
//  VListItem,
//  VListItemContent,
//  VListItemSubtitle,
//  VListItemTitle,
//  VProgressCircular,
//  VProgressLinear,
//  VRow,
//  VExpansionPanel,
//  VExpansionPanelHeader,
//  VExpansionPanelContent,
//  VDataIterator,
//  VSheet,
//  VCheckbox,
//} from "vuetify/lib";
//
//Vue.use(Vuetify, {
//  components: {
//    VBtn,
//    VIcon,
//    VCol,
//    VRow,
//    VProgressCircular,
//    VCard,
//    VCardText,
//    VCardTitle,
//    VCardActions,
//    VContainer,
//    VProgressLinear,
//    VList,
//    VListItem,
//    VListItemContent,
//    VListItemTitle,
//    VListItemSubtitle,
//    VChip,
//    VExpansionPanel,
//    VExpansionPanelHeader,
//    VExpansionPanelContent,
//    VDataIterator,
//    VSheet,
//    VCheckbox,
//  },
//});

//import Vue from "vue";
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

export const vuetify = createVuetify({
  components,
  directives,
  theme: {
    themes: {
      light: {
        colors: {

          primary: "#3f50b5",
          secondary: "#f44336",
          accent: "#82B1FF",
          error: "#FF5252",
          info: "#2196F3",
          success: "#4CAF50",
          warning: "#FFC107",
        },
      },
    },
  },
  //icons: {
  //iconfont: "mdi",
  //},
})
//export default new Vuetify({
//});
