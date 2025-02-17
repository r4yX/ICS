<template>
  <div id="main">
		<div id="header">
			<h2>Clientes</h2>
			<button @click="toggleCustomer"><svg-icon type="mdi" :path="mdiPlus"/></button>
		</div>
		<component 
			:is="customerComponent" 
			:data="selectedCustomer"
			@destroy="customerComponent = null"
			@refresh-customers="updateCustomers"
			@clearCustomer="selectCustomer" /> 
		<ul>
			<Client
				v-for="(client, index) in clients"
				:key="index"
				:data="client"
				:index="index"
				@edit="toggleCustomer"
				@select-customer="selectCustomer" /> 
		</ul>
  </div>
</template>

<script>
import { ref, shallowRef, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiPlus } from '@mdi/js';
import CreateCustomer from "@components/CreateCustomer.vue";
import Client from "@components/Client.vue"

export default {
  name: 'Customers',
  components: {
    SvgIcon,
		CreateCustomer,
		Client,
		Client
  },
	setup() {
		const clients = ref([]);
		const selectedCustomer = ref({});
		const customerComponent = shallowRef(null);

		const toggleCustomer = () => {
			if (!customerComponent.value) {
        customerComponent.value = CreateCustomer;
      } else {
        customerComponent.value = null;
      }
		};

		const updateCustomers = async() => {
			let log = await invoke('obtain_customers')
			clients.value = log
		}
		const selectCustomer = (data) => {
			selectedCustomer.value = data
		}
		onMounted(updateCustomers);
		return {
			mdiPlus,
			clients,
			customerComponent,
			selectedCustomer,
			// Functions
			toggleCustomer,
			updateCustomers,
			selectCustomer
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
