<template>
  <div id="menu" :class="{ hidden: isMenuHidden }">
		<button id="toggle-menu" @click="toggleMenu()"><svg-icon type="mdi" :path="mdiChevronLeft"/></button>
    <ul id="views">
			<button @click="navigateTo('budgets')" id="budgets"><svg-icon type="mdi" :path="mdiNoteText"/><p>Presupuesto</p></button>
			<button @click="navigateTo('orders')" id="orders"><svg-icon type="mdi" :path="mdiNoteCheck"/><p>Orden de Trabajo</p></button>
      <button @click="navigateTo('history')" id="history"><svg-icon type="mdi" :path="mdiClock"/><p>Historico</p></button>
      <hr>
      <button @click="navigateTo('customers')" id="customers"><svg-icon type="mdi" :path="mdiAccount"/><p>Clientes</p></button>
      <button @click="navigateTo('vehicles')" id="vehicles"><svg-icon type="mdi" :path="mdiCar"/><p>Vehiculos</p></button>
			<button @click="navigateTo('inventory')" id="inventory"><svg-icon type="mdi" :path="mdiFileTable"/><p>Inventario</p></button>
      <hr>
      <button @click="navigateTo('balance')" id="balance"><svg-icon type="mdi" :path="mdiCashRegister"/><p>Balance</p></button>
      <button @click="navigateTo('workers')" id="workers"><svg-icon type="mdi" :path="mdiAccountHardHat"/><p>Trabajadores</p></button>
    </ul>
  </div>
</template>

<script>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiChevronLeft, mdiNoteText, mdiNoteCheck, mdiClock, mdiAccount, mdiCar, mdiFileTable,
				mdiCashRegister, mdiAccountHardHat, mdiShopping} from '@mdi/js';

export default {
  name: 'Menu',
  components: {
    SvgIcon
  },
  setup() {
    // Navegación con Vue Router
    const router = useRouter();
    const navigateTo = (routeName) => {
			let views = document.getElementById('views');
			let children = views.children;

			for (let i = 0; i < children.length; i++) {
					children[i].style.background = '#24c8db20';
					children[i].onmouseenter = () => children[i].style.background = '#44e8fb80';
					children[i].onmouseleave = () => children[i].style.background = '#24c8db20';
			}
      router.push({ name: routeName });
			let clicked = document.getElementById(routeName);
			clicked.style.background = '#44e8fb80';
			clicked.onmouseleave = () => {};
    };

    const isMenuHidden = ref(false);
    // Función para cerrar el menú
    const toggleMenu = () => {
      isMenuHidden.value = !isMenuHidden.value;
      localStorage.setItem('isMenuHidden', isMenuHidden.value);
    };

    // Devolver las funciones y estados al template
    return {
      isMenuHidden,
      navigateTo,
      toggleMenu,
			// Icons
			mdiChevronLeft, mdiNoteText, mdiNoteCheck, mdiClock, mdiAccount, mdiCar, mdiFileTable,
			mdiCashRegister, mdiAccountHardHat,	mdiShopping
    };
  },
};
</script>

<style scoped>
* {
  list-style: none;
  color: #eee;
}
#menu {
  position: relative;
  left: 0;
  top: 0;
  background: #1a1a1a;
  margin: 0px;
  padding: 0px;
  height: 100%;
  width: 240px;
  transition: width .5s ease;
}
#menu.hidden {
  position: relative;
  width: 100px;
}
ul {
  display: flex;
  flex-direction: column;
  justify-content: top;
  align-items: center;
  padding: 0px;
  width: 100%;
  height: 90%;
}
#menu > ul > button {
	display: flex;
	align-items: center;
	justify-content: space-around;
  font-size: 1.2em;
  margin: 4px 0px;
  width: 90%;
	height: 50px;
  background: #24c8db20;
  border-radius: .3em;
  transition: background .5s, width .5s;
  outline: none;
  border: none;
	overflow: hidden;
	white-space: nowrap;
	cursor: pointer;
}
#menu > ul > button:hover {
  background: #44e8fb80;
}
#menu.hidden > ul > button {
	width: 70%;
}
#menu.hidden > ul > button > p {
	display: none;
}
#menu.hidden > ul > button > svg {
	display: block;
}
hr {
  width: 10%;
}
#config {
  position: absolute;
  bottom: 1rem;
}
#toggle-menu {
  height: 30px;
  width: 30px;
  border-radius: 50%;
  border: none;
  outline: none;
  display: flex;
  position: absolute;
	cursor: pointer;
  top: 1rem;
  left: 14.5rem;
  background: #44e8fb80;
  align-items: center;
  justify-content: center;
  font-size: 25px;
  color: #fff;
  transition: all .5s
}
#menu.hidden > #toggle-menu {
  left: 5.8rem;
  transform: rotateZ(180deg)
}
</style>
