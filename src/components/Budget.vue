<template>
	<div id="card" class="closed">
		<button id="closeBtn" @click="toggleCard($event)"><svg-icon type="mdi" :path="mdiArrowExpandDown"/></button>
		<div id="header">
			<p title="Cliente"><svg-icon type="mdi" :path="mdiAccount"/>{{ data.customer }}</p>
			<p title="Patente"><svg-icon type="mdi" :path="mdiCar"/>{{ data.vehicle }}</p>
			<p title="Total"><svg-icon type="mdi" :path="mdiCurrencyUsd"/>{{ data.total }}</p>
			<p title="Kilometraje"><svg-icon type="mdi" :path="mdiRoadVariant"/>{{ data.kilometrage }}</p>
			<p v-if="data.paid != 'none'" title="Pagado"><svg-icon type="mdi" :path="mdiCashCheck"/>{{ data.paid }}</p>
			<p v-if="data.pay_date != 'none'" title="Fecha de pago"><svg-icon type="mdi" :path="mdiCashRegister"/>{{ data.pay_date }}</p>
		</div>
		 <table v-if="details && details.length > 0">
			 <tbody>
				 <tr>
					 <td>Item</td>
					 <td>Precio</td>
					 <td>Cant</td>
					 <td>Subtotal</td>
					 <td>IVA</td>
					 <td>Total</td>
				 </tr>
				 <tr v-for="(detail, index) in details" :key="index">
					 <td>{{ detail.item }}</td>
					 <td>{{ detail.price }}</td>
					 <td>{{ detail.cant }}</td>
					 <td>{{ detail.subtotal }}</td>
					 <td>{{ detail.iva }}</td>
					 <td>{{ detail.total }}</td>
				 </tr>
			 </tbody>
		 </table>
		<button v-if="data.paid == 'none'" title="Aprobar Presupuesto" id="checkBtn" @click="createBudget()"><svg-icon type="mdi" :path="mdiCheck"/></button>
		<button v-if="data.paid != 'none' && data.pay_date == 'none'" title="Acreditar pago" id="checkBtn" @click="payOrder()"><svg-icon type="mdi" :path="mdiCashPlus"/></button>
		<button v-if="data.pay_date != 'none'" title="Eliminar del historial" id="checkBtn" @click="deleteHistory()"><svg-icon type="mdi" :path="mdiClockMinus"/></button>
	</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiAccount, mdiCar, mdiHelpCircle, mdiRoadVariant, mdiArrowExpandDown, mdiArrowCollapseUp,	mdiCheck, mdiCurrencyUsd, mdiCashCheck, mdiCashPlus, mdiClockMinus, mdiCashRegister} from '@mdi/js';

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
	setup(props) {
		const details = ref()
		const toggleCard = async(e) => {
			details.value = await invoke('obtain_details', {'id': props.data.id})

			let cardParent = e.target.parentElement;
			while (cardParent.tagName.toLowerCase() != 'div') {
				cardParent = cardParent.parentElement
			}
			cardParent.classList.toggle('closed');
		};
		// Budget Function (Budgets.vue)
		const createBudget = async() => {
			let paid = parseInt(prompt("El cliente ha pagado ($):", "0"))
			let log = await invoke('create_order', {'id': props.data.id, 'paid': paid})
			alert(log)
		}

		// Order Function (Orders.vue)
		const payOrder = async() => {
			let pay = parseInt(prompt("El cliente ha pagado ($):", "0"))
			let paid = parseInt(props.data.paid)
			// If customer paid total of cost
			if (props.data.total == pay+paid) {
				let now = new Date();
				let pay_date = now.toLocaleString('en-CA', {
					hour12: false, month: '2-digit', day: '2-digit', year: 'numeric',
					hour: '2-digit', minute: '2-digit', second: '2-digit'});
				pay_date = pay_date.replace(',', '')

				let log = await invoke('create_history', {'id': props.data.id, 'payDate': pay_date})
				alert(log)
			} else {
				console.log('menor a la deuda');
				let log = await invoke('pay_order', {'id': props.data.id, 'paid': paid+pay})
				alert(log)
			}
		}

		// History Funtion (History.vue)
		const deleteHistory = async() => {
			console.log(props.data.id)
		}

		return {
			details,
			toggleCard,
			createBudget,
			payOrder,
			deleteHistory,
			// Icons
			mdiAccount, mdiCar, mdiHelpCircle, mdiRoadVariant,
			mdiArrowExpandDown, mdiCheck, mdiCurrencyUsd, mdiCashCheck,
			mdiCashPlus, mdiClockMinus, mdiCashRegister
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
	height: 360px;
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
	grid-template-columns: 1fr 1fr 1fr;
	justify-items: center;
	align-items: center;
}
#header > p {
	cursor: default;
	display: flex;
	justify-items: center;
	align-items: center;
}
table {
	margin: 2rem 1rem;
}
td {
	width: auto;
	border-radius: .2rem;
	padding: 3px 6px;
}
tr:first-child {
	background-color: #7f7f7f;
}
tr {
	background-color: #555;
}
tr:nth-child(even) {
	background-color: #232323;
}
#checkBtn {
	display: flex;
	align-items: center;
	justify-content: center;
	height: 2.2rem;
	width: 2.2rem;
	position: absolute;
	right: .6rem;
	bottom: .5rem;
	border: none;
	cursor: pointer;
	border-radius: .5rem;
}
</style>
