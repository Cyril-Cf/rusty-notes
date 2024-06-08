import { createRouter, createWebHistory, RouteLocationRaw } from 'vue-router'
import authPromise from '@/plugins/keycloak'

const routes = [
  {
    path: '/',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'home',
        component: () => import('@/views/Home.vue'),
      },
    ],
  },
  {
    path: '/single_list/:listId',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'single_list',
        component: () => import('@/views/SingleList.vue'),
        meta: {
          isAuthenticated: true,
          // requiredRole: ['admin', 'user'],
        },
      },
    ],
  },
  {
    path: '/subscription_more_infos/:redirectUri',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'subscription_more_infos',
        component: () => import('@/views/SubscriptionMoreInfos.vue'),
        meta: {
          isAuthenticated: true,
          // requiredRole: ['admin', 'user'],
        },
      },
    ],
  },
  {
    path: '/my_notes',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'my_notes',
        component: () => import('@/views/MyLists.vue'),
        meta: {
          isAuthenticated: true,
          // requiredRole: ['admin', 'user'],
        },
      },
    ],
  },
  {
    path: '/my_friends',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'my_friends',
        component: () => import('@/views/MyFriends.vue'),
        meta: {
          isAuthenticated: true,
          // requiredRole: ['admin', 'user'],
        },
      },
    ],
  },
  {
    path: '/add_list',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'add_list',
        component: () => import('@/views/NewList.vue'),
        meta: {
          isAuthenticated: true,
          // requiredRole: ['admin', 'user'],
        },
      },
    ],
  },
  {
    path: '/unauthorized',
    component: () => import('@/layouts/default/Default.vue'),
    children: [
      {
        path: '',
        name: 'unauthorized',
        component: () => import('@/views/Unauthorized.vue'),
        meta: {
          isAuthenticated: true
        },
      },
    ],
  },
  {
    path: "/:catchAll(.*)",
    name: "not-found",
    component: () => import('@/views/NotFound.vue'),
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
})

router.beforeEach((to, from, next) => {
  if (!to.meta.isAuthenticated) {
    next()
  } else {
    authPromise.then(async auth => {
      if (auth.isAuthenticated() && to.path !== '/unauthorized') {
        if (!to.meta?.requiredRole || to.meta?.requiredRole?.some(r => auth.userRoles().includes(r))) {
          auth.putTokenInLocalStorage();
          next();
        } else {
          auth.putTokenInLocalStorage();
          next('/unauthorized')
        }
      } else if (auth.isAuthenticated() && to.path === '/unauthorized') {
        auth.putTokenInLocalStorage();
        next()
      } else {
        const redirect: RouteLocationRaw = { query: { ...to.query, 'sync_me': null } }
        const redirectUri = `${location.origin}${router.resolve(redirect, to).href}`
        auth.login(redirectUri)
      }
    })
  }
})

router.beforeEach((to, from, next) => {
  if ('sync_me' in to.query) {
    authPromise.then(async auth => {
      // console.log('Authenticated')
    }).finally(() => {
      delete to.query.sync_me // remove sync_me query parameter to avoid endless recursion
      next({ path: to.path, query: to.query, params: to.params, replace: true })
    })
  } else {
    next()
  }
})

export default router
