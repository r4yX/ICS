<template>
	<div id="card" class="closed">
		<button id="closeBtn" @click="toggleCard($event)"><svg-icon type="mdi" :path="mdiArrowExpandDown"/></button>
		<div id="header">
			<p title="Cliente"><svg-icon type="mdi" :path="mdiAccount"/>Jhon Doe</p>
			<p title="Patente"><svg-icon type="mdi" :path="mdiCar"/>ABC 123</p>
			<p title="Concepto"><svg-icon type="mdi" :path="mdiHelpCircle"/>Reparación</p>
			<p title="Kilometraje"><svg-icon type="mdi" :path="mdiRoadVariant"/>32.000</p>
		</div>
		<table id="charges">
			<tbody>
				<tr>
					<td>Item</td>
					<td>Precio</td>
					<td>Cantidad</td>
					<td>Tipo</td>
					<td>Subtotal</td>
					<td>IVA</td>
					<td>Total</td>
				</tr>
				<tr>
					<td>Bomba Inyectora</td>
					<td>$40.000,00</td>
					<td>1</td>
					<td>Producto</td>
					<td>$40.000,00</td>
					<td>$8.400,00</td>
					<td>$48.400,00</td>
				</tr>
				<tr>
					<td>Bomba Inyectora</td>
					<td>$40.000,00</td>
					<td>1</td>
					<td>Producto</td>
					<td>$40.000,00</td>
					<td>$8.400,00</td>
					<td>$48.400,00</td>
				</tr>
			</tbody>
		</table>
		<table>
			<tbody>
				<tr>
					<td>Total: $96.800,00</td>
					<td>Pagado: $50.800,00</td>
					<td>Debe: $46.000,00</td>
				</tr>
			</tbody>
		</table>
		<button title="Añadir pago" id="payBtn" @click="Pagar($event)"><svg-icon type="mdi" :path="mdiCurrencyUsd"/></button>
		<button title="Descargar PDF" id="pdfBtn" @click="downloadPDF($event)"><svg-icon type="mdi" :path="mdiFilePdfBox"/></button>
	</div>
</template>

<script>
import { ref } from "vue";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiAccount, mdiCar, mdiHelpCircle, mdiRoadVariant, mdiArrowExpandDown, mdiArrowCollapseUp,
				mdiFilePdfBox, mdiCurrencyUsd} from '@mdi/js';

export default {
	props: {
		detail: {
			type: Object,
			required: true
		}, 
	},
	components: {
		SvgIcon,
	},
	setup() {
		const toggleCard = (e) => {
			let cardParent = e.target.parentElement;
			while (cardParent.tagName.toLowerCase() != 'div') {
				cardParent = cardParent.parentElement
			}
			cardParent.classList.toggle('closed');
		};
		const downloadPDF = () => {
			console.log('descarga')
		}

		return {
			toggleCard,
			downloadPDF,
			// Icons
			mdiAccount,
			mdiCar,
			mdiHelpCircle,
			mdiRoadVariant,
			mdiArrowExpandDown,
			mdiFilePdfBox,
			mdiCurrencyUsd,
		}
	},
};

</script>

<style scoped>
#card {
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
	grid-column-gap: 8rem;
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
table {
	margin: 1rem 1rem;
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
#payBtn {
	display: flex;
	align-items: center;
	justify-content: center;
	height: 2.2rem;
	width: 2.2rem;
	position: absolute;
	left: .6rem;
	bottom: .5rem;
	border: none;
	cursor: pointer;
	border-radius: .5rem;
}
#card.closed > #payBtn {
	display: none;
}
#pdfBtn {
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
