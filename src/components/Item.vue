<template>
	<div class="item">
		<p title="Item">{{ detail.item }}</p>
		<p title="Precio">${{ detail.price }}</p>
		<p title="Cantidad">{{ detail.cant }}</p>
		<p title="Tipo">{{ detail.tipo }}</p>
		<p title="SubTotal">${{ detail.stotal }}</p>
		<p title="IVA ($)">${{ detail.iva }}</p>
		<p title="Total">${{ detail.total }}</p>
		<button id="del-btn" @click="delDetail(index)" type="button">
			<i class="fa-solid fa-trash"></i>
		</button>
	</div>
</template>

<script>
import { ref } from "vue";

export default {
	props: {
		detail: {
			type: Object,
			required: true
		}, 
		index: {
			type: Number,
			required: true,
		},
		delDetail: {
			type: Function,
			required: true,
		},
	},
};
</script>

<style scoped>
.item {
	width: 100%;
	display: grid;
	gap: 10px;
	grid-template-columns: 64px 74px 63px 100px 130px 64px 54px auto;
	justify-content: start;
	align-items: center;
}
.item > * {
	padding: 0 10px;
}

#del-btn {
	cursor: pointer;
	border: none;
	height: 2rem;
	width: 2rem;
	background-color: #a33;
	border-radius: .3rem;
	color: #ccc;
}
</style>
