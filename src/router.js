import { createRouter, createWebHistory } from 'vue-router';
import Budgets from './views/Budgets.vue';
import Orders from './views/Orders.vue';
import History from './views/History.vue'; 
import Customers from './views/Customers.vue'; 
import Vehicles from './views/Vehicles.vue'; 
import Inventory from './views/Inventory.vue'; 
import Balance from './views/Balance.vue'; 
import Workers from './views/Workers.vue'; 

const routes = [
  { path: '/', name: 'budgets', component: Budgets },
  { path: '/orders', name: 'orders', component: Orders },
  { path: '/history', name: 'history', component: History },
  { path: '/customers', name: 'customers', component: Customers },
  { path: '/vehicles', name: 'vehicles', component: Vehicles },
  { path: '/inventory', name: 'inventory', component: Inventory },
  { path: '/balance', name: 'balance', component: Balance },
  { path: '/workers', name: 'workers', component: Workers },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

document.body.style.margin = '0';
document.body.style.padding = '0';

export default router;
