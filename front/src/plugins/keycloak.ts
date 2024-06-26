import Keycloak, { KeycloakProfile } from 'keycloak-js'

class Auth {
  private readonly keycloak: Keycloak

  static async build(): Promise<Auth> {
    const keycloak = new Keycloak()
    keycloak.onTokenExpired = () => keycloak.updateToken(30) //TODO: show notification with .catch(() => notify-user-somehow);
    await keycloak.init({
      checkLoginIframe: false,
      pkceMethod: 'S256',
    })
    return new Auth(keycloak)
  }

  private constructor(keycloak: Keycloak) {
    this.keycloak = keycloak
  }

  public isAuthenticated(): boolean {
    return this.keycloak.authenticated || false
  }

  public async login(redirectUri: string): Promise<void> {
    await this.keycloak.login({
      redirectUri: (redirectUri)
    })
  }

  public putTokenInLocalStorage() {
    const accessToken = this.keycloak.token;
    localStorage.setItem('accessToken', accessToken!);
  }

  public removeTokenFromLocalStorage() {
    localStorage.removeItem('accessToken');
  }

  public async logout(redirectUri?: string): Promise<void> {
    this.removeTokenFromLocalStorage()
    return this.keycloak.logout({
      redirectUri: redirectUri
    })
  }

  public async bearerToken(): Promise<string | undefined> {
    await this.keycloak.updateToken(10)
    return this.keycloak.token
  }

  public userRoles(): string[] {
    return this.keycloak.tokenParsed?.realm_access?.roles ?? []
  }

  public isAdmin(): boolean {
    return this.keycloak.tokenParsed?.realm_access?.roles.includes('admin') ?? false
  }

  public isUser(): boolean {
    return this.keycloak.tokenParsed?.realm_access?.roles.includes('user') ?? false
  }

  public async userInfo(): Promise<KeycloakProfile | undefined> {
    await this.keycloak.loadUserProfile();
    return this.keycloak?.profile;
  }

  public userId(): string | undefined {
    return this.keycloak.subject;
  }

  public getRegisterUrl(): string {
    return this.keycloak.createRegisterUrl()
  }

  public getLoginUrl(): string {
    return this.keycloak.createLoginUrl()
  }
}

const instance: Promise<Auth> = (async () => {
  return await Auth.build()
})()

export default instance
