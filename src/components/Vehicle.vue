
<template>
	<div class="item">
		<p title="Patente">{{ plate }}</p>
		<button id="del-btn" @click="delCar(index)" type="button">
			<i class="fa-solid fa-trash"></i>
		</button>
	</div>
</template>

<script>
import { ref } from "vue";

export default {
	props: {
		plate: {
			type: String,
			required: true
		}, 
		index: {
			type: Number,
			required: true,
		},
		delCar: {
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
	border-radius: .4rem;
	color: #ccc;
	margin-left: 1.4rem;
}
</style>
