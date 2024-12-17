<template>
  <div id="main">
		<div id="header">
			<h2>Inventario</h2>
			<button @click="toggleItem"><svg-icon type="mdi" :path="mdiPlus"/></button>
		</div>
		<component :is="ItemComponent" @destroy="ItemComponent = null"/>
  </div>
</template>

<script>
import { shallowRef } from 'vue';
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiPlus } from '@mdi/js';
import CreateItem from "@components/CreateItem.vue";

export default {
  name: 'Inventory',
  components: {
    SvgIcon,
		CreateItem
  },
	setup() {
		const ItemComponent = shallowRef(null);

		const toggleItem = () => {
			if (!ItemComponent.value) {
        ItemComponent.value = CreateItem;
      } else {
        ItemComponent.value = null;
      }
		};
		return {
			mdiPlus,
			toggleItem,
			ItemComponent,
		}
	}
};
</script>

<style scoped>
#main {
  display: flex;
  flex-direction: column;
	background: #222;
	color: #ddd;
}
#header {
  display: flex;
  justify-content: space-between;
}
#header > h2 {
  margin-left: 3rem;
}
#header > button {
	display: flex;
	align-items: center;
	justify-content: center;
  margin: .4rem;
  height: 2rem;
  width:  2rem;
  outline: none;
  border: none;
  border-radius: .4rem;
	cursor: pointer;
  background: #24c8db20;
	transition: background .3s;
}
#header > button:hover {
  background: #44e8fb80;
}
</style>
