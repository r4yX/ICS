<template>
  <div id="main">
		<div id="header">
			<h2>Inventario</h2>
			<button @click="toggleItem"><svg-icon type="mdi" :path="mdiPlus"/></button>
		</div>
		<component :is="ItemComponent" @destroy="ItemComponent = null"/>
		<ul>
		<Item
        v-for="(item, index) in inventory"
        :key="index"
        :data="item"
        :index="index"
				@refresh-items="updateInventory"
				/>
		</ul>
  </div>
</template>

<script>
import { ref, onMounted, shallowRef } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiPlus } from '@mdi/js';
import CreateItem from "@components/CreateItem.vue";
import Item from "@components/Item.vue";

export default {
  name: 'Inventory',
  components: {
    SvgIcon,
		CreateItem,
		Item
  },
	setup() {
		const inventory = ref([]);
		const ItemComponent = shallowRef(null);

		const toggleItem = () => {
			if (!ItemComponent.value) {
        ItemComponent.value = CreateItem;
      } else {
        ItemComponent.value = null;
      }
		};
		
		const updateInventory = async() => {
			let log = await invoke('obtain_items', {'tipo':"none"})
			inventory.value = log
		}
		onMounted(updateInventory)
		return {
			inventory,
			updateInventory,
			ItemComponent,
			toggleItem,
			mdiPlus,
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
	overflow-y: scroll;
}
#header {
  display: flex;
  justify-content: space-between;
}
#header > h2 {
  margin-left: 2rem;
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
