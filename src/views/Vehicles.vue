<template>
  <div id="main">
		<div id="header">
			<h2>Vehiculos</h2>
			<button @click="toggleVehicle"><svg-icon type="mdi" :path="mdiPlus"/></button>
		</div>
		<component :is="VehicleComponent" @destroy="VehicleComponent = null"/>
		<Vehicle
        v-for="(vehicle, index) in vehicles"
        :key="index"
        :data="vehicle"
        :index="index"
				@refresh-budgets="updateVehicles"
		/>
  </div>
</template>

<script>
import { ref, onMounted, shallowRef } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiPlus } from '@mdi/js';
import CreateVehicle from "@components/CreateVehicle.vue";
import Vehicle from "@components/Vehicle.vue";

export default {
  name: 'Vehicles',
  components: {
    SvgIcon,
		CreateVehicle,
		Vehicle
  },
	setup() {
		const VehicleComponent = shallowRef(null);
		const vehicles = ref([]);

		const toggleVehicle = () => {
			if (!VehicleComponent.value) {
        VehicleComponent.value = CreateVehicle;
      } else {
        VehicleComponent.value = null;
      }
		};

		const updateVehicles = async() => {
			let log = await invoke('obtain_vehicles', {'plate': "none"});
			vehicles.value = log;
		}
		onMounted(updateVehicles)
		return {
			vehicles,
			mdiPlus,
			toggleVehicle,
			VehicleComponent,
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
