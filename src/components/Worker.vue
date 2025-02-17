<template>
	<div id="card" class="closed">
		<button id="closeBtn" @click="toggleCard($event)"><svg-icon type="mdi" :path="mdiArrowExpandDown"/></button>
		<div id="header">
			<p title="Nombre"><svg-icon type="mdi" :path="mdiAccount"/>{{ data.name }}</p>
			<p title="DNI"><svg-icon type="mdi" :path="mdiCardAccountDetails"/>{{ data.dni }}</p>
			<p title="Telefono"><svg-icon type="mdi" :path="mdiPhone"/>{{ data.phone }}</p>
			<p title="Direccion"><svg-icon type="mdi" :path="mdiHome"/>{{ data.address }}</p>
			<p title="Salario"><svg-icon type="mdi" :path="mdiCashRegister"/>{{ data.salary }}</p>
		</div>
		<button title="checkPay" class="bottom_btns" id="checkBtn" @click="getPays(data)"><svg-icon type="mdi" :path="mdiCurrencyUsd"/></button>
		<button title="Eliminar" class="bottom_btns" id="delBtn" @click="delWorker(data)"><svg-icon type="mdi" :path="mdiTrashCan"/></button>
	</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiArrowExpandDown, mdiAccount, mdiCardAccountDetails, mdiPhone, mdiHome,
		mdiCashRegister, mdiTrashCan, mdiCurrencyUsd} from '@mdi/js';

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
	setup(props, { emit }) {
		const toggleCard = async(e) => {
			let cardParent = e.target.parentElement;
			while (cardParent.tagName.toLowerCase() != 'div') {
				cardParent = cardParent.parentElement
			}
			cardParent.classList.toggle('closed');
		};

		const getPays = (data) => {
			emit('selectWorker', data)
		}
		
		const delWorker = async(worker) => {
			let userCheck = confirm("Do you want to delete worker?")
			if (!userCheck) {return 0}
			let log = await invoke('remove_worker', {'dni': worker.dni})
			alert(log)
			emit('refresh-workers')
		}

		return {
			toggleCard,
			getPays,
			delWorker,
			// Icons
			mdiArrowExpandDown, mdiAccount, mdiCardAccountDetails, mdiPhone, mdiHome, 
			mdiCashRegister, mdiTrashCan, mdiCurrencyUsd,
		}
	},
};

</script>

<style scoped>
#card {
	margin: 2rem 0;
	position: relative;
	background: #333;
	display: flex;
	flex-direction: column;
	width: 90%;
	height: 260px;
	justify-content: start;
	align-items: center;
	align-self: center;
	border-radius: .5rem;
	overflow-x: hidden;
	overflow-y: scroll; 
	transition: height .7s ease;
}
#card.closed {
	height: 3.2rem;
	overflow-y: hidden; 
}
#closeBtn {
	display: flex;
	align-items: center;
	justify-content: center;
	height: 2.2rem;
	width: 2.2rem;
	position: absolute;
	right: .6rem;
	margin-top: .5rem;
	border: none;
	cursor: pointer;
	border-radius: .5rem;
	transition: transform .3s;
	transform: rotateZ(180deg);
	z-index: 2;
}
#card.closed > #closeBtn {
	transform: rotateZ(0deg);
}
#closeBtn:focus {
	all: none
}
#header {
	display: grid;
	grid-column-gap: 2rem;
	grid-template-columns: 1fr 1fr;
	justify-items: center;
	align-items: center;
}
#header > p {
	cursor: default;
	display: flex;
	justify-items: center;
	align-items: center;
}
.bottom_btns {
	display: flex;
	align-items: center;
	justify-content: center;
	height: 2.2rem;
	width: 2.2rem;
	position: absolute;
	bottom: .5rem;
	border: none;
	cursor: pointer;
	border-radius: .5rem;
}
#checkBtn { right: .6rem }
#delBtn { left: .6rem }
</style>
