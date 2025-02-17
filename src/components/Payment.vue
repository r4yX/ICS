<template>
	<div id="carta">
		<p title="Direccion"><svg-icon type="mdi" :path="mdiCalendar"/>{{ data.date }}</p>
		<p title="Salario"><svg-icon type="mdi" :path="mdiCashRegister"/>{{ data.amount.replace(".",",") }}</p>
	</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiCalendar, mdiCashRegister} from '@mdi/js';

export default {
	props: {
		data: {
			type: Object,
			required: true
		}, 
	},
	components: {
		SvgIcon,
	},
	setup() {
		return {
			// Icons
			mdiCalendar, mdiCashRegister
		}
	},
};

</script>

<style scoped>
#carta {
	margin: 2rem 0;
	position: relative;
	background: #333;
	display: flex;
	flex-direction: column;
	width: 170px;
	height: 140px;
	justify-content: space-around;
	align-items: center;
	border-radius: .5rem;
	overflow-x: hidden;
	overflow-y: scroll; 
	transition: height .7s ease;
}
#carta p {
	display: flex;
	align-items: center;
}
</style>
