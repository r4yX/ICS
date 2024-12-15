import { createRouter, createWebHistory } from 'vue-router';
import Budgets from './views/Budgets.vue';
import Works from './views/Works.vue';
import History from './views/History.vue'; 
import Customers from './views/Customers.vue'; 
import Vehicles from './views/Vehicles.vue'; 
import Inventory from './views/Inventory.vue'; 
import Balance from './views/Balance.vue'; 
import Workers from './views/Workers.vue'; 
import Shop from './views/Shop.vue'; 
import Config from './views/Config.vue'; 

const routes = [
  { path: '/', name: 'budgets', component: Budgets },
  { path: '/works', name: 'works', component: Works },
  { path: '/history', name: 'history', component: History },
  { path: '/customers', name: 'customers', component: Customers },
  { path: '/vehicles', name: 'vehicles', component: Vehicles },
  { path: '/inventory', name: 'inventory', component: Inventory },
  { path: '/balance', name: 'balance', component: Balance },
  { path: '/workers', name: 'workers', component: Workers },
  { path: '/shopping', name: 'shopping', component: Shop },
  { path: '/config', name: 'config', component: Config },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

document.body.style.margin = '0';
document.body.style.padding = '0';

export default router;
