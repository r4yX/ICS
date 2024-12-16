<template>
  <div id="main">
		<div id="header">
			<h2>Clientes</h2>
			<button @click="toggleCustomer"><svg-icon type="mdi" :path="mdiPlus"/></button>
		</div>
		<component :is="customerComponent" @destroy="customerComponent = null"/>
  </div>
</template>

<script>
import { shallowRef } from 'vue';
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiPlus } from '@mdi/js';
import CreateCustomer from "../components/CreateCustomer.vue";

export default {
  name: 'Works',
  components: {
    SvgIcon,
		CreateCustomer
  },
	setup() {
		const customerComponent = shallowRef(null);

		const toggleCustomer = () => {
			if (!customerComponent.value) {
        customerComponent.value = CreateCustomer;
      } else {
        customerComponent.value = null;
      }
		};
		return {
			mdiPlus,
			toggleCustomer,
			customerComponent,
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
