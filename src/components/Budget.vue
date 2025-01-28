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
	  <button v-if="data.pay_date == 'none'" title="Guardar PDF" class="budget_btns" id="pdfBtn" @click="createPDF()"><svg-icon type="mdi" :path="mdiFilePdfBox"/></button>
		<button v-if="data.paid == 'none'" title="Aprobar Presupuesto" class="budget_btns" id="checkBtn" @click="createBudget()"><svg-icon type="mdi" :path="mdiCheck"/></button>
		<button v-if="data.paid != 'none' && data.pay_date == 'none'" title="Acreditar pago" class="budget_btns" id="checkBtn" @click="payOrder()"><svg-icon type="mdi" :path="mdiCashPlus"/></button>
		<button v-if="data.pay_date != 'none'" title="Eliminar del historial" class="budget_btns" id="checkBtn" @click="deleteHistory()"><svg-icon type="mdi" :path="mdiClockMinus"/></button>
	</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiAccount, mdiCar, mdiHelpCircle, mdiRoadVariant, mdiArrowExpandDown, mdiArrowCollapseUp, mdiFilePdfBox,
	mdiCheck, mdiCurrencyUsd, mdiCashCheck, mdiCashPlus, mdiClockMinus, mdiCashRegister} from '@mdi/js';

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
			emit('refresh-budgets');
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

		// Budget & Order Function to create PDF
		const createPDF = async() => {
			let title = props.data.paid == "none" ? "Presupuesto" : "Orden"
			let now = new Date();
			let date_now = now.toLocaleString('en-CA', {
				hour12: false, month: '2-digit', day: '2-digit', year: 'numeric',
				hour: '2-digit', minute: '2-digit', second: '2-digit'});
			date_now = date_now.replace(',', '')

			let details_list = await invoke('obtain_details', {'id': props.data.id})
			let log = await invoke('create_pdf', {
				"info": {'id': props.data.id, 'title': title, 'date': date_now, 'total':props.data.total},
				'client':{"name": props.data.customer, "dni":"12345678", "phone":"54-3482-500112",
				"concept":props.data.concept}, 'vehicle':{"plate": props.data.vehicle, "maker": "Chevrolet",
				"model": "Astra", "kilometrage":props.data.kilometrage}, 'details': details_list})
		}

		// History Funtion (History.vue)
		const deleteHistory = async() => {
			let log = await invoke('delete_history', {'id': props.data.id})
			alert(log)
			emit('refresh-history')
		}

		return {
			details,
			toggleCard,
			createBudget,
			payOrder,
			createPDF,
			deleteHistory,
			// Icons
			mdiAccount, mdiCar, mdiHelpCircle, mdiRoadVariant,
			mdiArrowExpandDown, mdiCheck, mdiCurrencyUsd, mdiCashCheck,
			mdiCashPlus, mdiClockMinus, mdiCashRegister, mdiFilePdfBox
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
.budget_btns {
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
#checkBtn {
	right: .6rem;
}
#pdfBtn {
	left: .6rem;
}
</style>
