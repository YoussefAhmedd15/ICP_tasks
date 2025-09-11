import React, { createContext, useContext, useState, useEffect } from 'react';
import { AuthClient } from '@dfinity/auth-client';

const AuthContext = createContext();

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

function getIdentityProviderUrl() {
  const isLocalhost =
    typeof window !== 'undefined' && /localhost|127\.0\.0\.1/.test(window.location.hostname);
  // Local II canister (rdmx6-jaaaa-aaaaa-aaadq-cai) via hostname routing
  if (isLocalhost) {
    return 'http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943';
  }
  // Hosted II on mainnet
  return 'https://identity.ic0.app';
}

export const AuthProvider = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [principal, setPrincipal] = useState(null);
  const [authClient, setAuthClient] = useState(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    initializeAuth();
  }, []);

  const initializeAuth = async () => {
    try {
      const client = await AuthClient.create();
      setAuthClient(client);

      const isAuth = await client.isAuthenticated();
      setIsAuthenticated(isAuth);
      if (isAuth) {
        const identity = client.getIdentity();
        setPrincipal(identity.getPrincipal().toString());
      }
    } catch (error) {
      console.error('Failed to initialize auth client:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const login = async () => {
    if (!authClient) return;
    try {
      const identityProviderUrl = getIdentityProviderUrl();
      await authClient.login({
        identityProvider: identityProviderUrl,
        windowOpenerFeatures: 'left=100,top=100,width=480,height=720,toolbar=0,location=0,menubar=0,scrollbars=1,resizable=1',
        derivationOrigin: window.location.origin,
        onSuccess: () => {
          const identity = authClient.getIdentity();
          setPrincipal(identity.getPrincipal().toString());
          setIsAuthenticated(true);
        },
        onError: (error) => {
          console.error('Login failed:', error);
        },
      });
    } catch (error) {
      console.error('Login error:', error);
    }
  };

  const logout = async () => {
    try {
      if (authClient) {
        await authClient.logout();
      }
      setIsAuthenticated(false);
      setPrincipal(null);
    } catch (error) {
      console.error('Logout error:', error);
    }
  };

  const value = {
    isAuthenticated,
    principal,
    login,
    logout,
    isLoading,
    authClient,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};
