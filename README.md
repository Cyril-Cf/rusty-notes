# rusty-notes

WIP -  toy project (Actix web - Vue.js - Keycloak - GraphQL - WebSocket)
The aim is to have a todo list with a social aspect (joined notes between users)

Features:
- Real-time notification system
- Real-time list update between users
- Permission management for shared list (view and update)

Setup to self-host :
- create the three .env (root, inside back, inside front) from the .env.dist files
- docker compose up
- wait for the keycloak to be launched, create the rusty_notes realm and the main user admin of the realm
configure the realm also with front url, groups, add the keycloak PK for this realm to the .env of the back folder
