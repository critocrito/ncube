import React, {useEffect, useState} from "react";
import fetchMock, {FetchMockStatic} from "fetch-mock";
import {useMachine} from "@xstate/react";

import {AppProvider, WorkspaceProvider} from "../lib/context";
import PubSub from "../lib/pubsub";
import appMachine from "../machines/app";
import workspaceMachine from "../machines/workspace";
import {localWorkspace as workspace} from "./data";
import {HttpDataResponse} from "../lib/http";

fetchMock.config.overwriteRoutes = true;

interface FetchMockProps<T> {
  matcher: string;
  response: HttpDataResponse<T> | number;
  method?: "GET" | "POST" | "PUT" | "DELETE";
  children?: React.ReactNode;
}

export const FetchMock = <T extends unknown>({
  matcher,
  response,
  method = "GET",
  children,
}: FetchMockProps<T>) => {
  const [mock, setMock] = useState<FetchMockStatic | undefined>();

  useEffect(() => {
    setMock(
      fetchMock.mock({
        url: `http://127.0.0.1:40666/api${matcher}`,
        response,
        method,
      }),
    );

    return () => {
      if (mock) mock.restore();
    };
  }, [matcher, response]);

  return <>{mock && children}</>;
};

interface WrapperProps {
  children?: React.ReactNode;
}

export const Wrapper = ({children}: WrapperProps) => {
  const [appState, appSend] = useMachine(appMachine, {
    context: {
      workspaces: [],
      pubsub: new PubSub(),
    },
    services: {
      listWorkspaces: async (_ctx, _ev) => [],
      fetchWorkspace: async (_ctx, _ev) => {},
      deleteWorkspace: async (_ctx, _ev) => {},
    },
  });

  const [workspaceState, workspaceSend] = useMachine(workspaceMachine, {
    context: {
      workspace,
      dataStats: {
        total: 0,
        sources: 0,
        segments: 0,
        // videos: 0,
      },
      sourceStats: {
        total: 0,
        types: 0,
      },
    },
    services: {
      fetchStats: async () => ({
        dataStats: {
          total: 1,
          sources: 2,
          segments: 3,
          // videos: 0,
        },
        sourceStats: {
          total: 4,
          types: 5,
        },
      }),
    },
  });

  return (
    <AppProvider value={[appState, appSend]}>
      <WorkspaceProvider value={[workspaceState, workspaceSend]}>
        {children}
      </WorkspaceProvider>
    </AppProvider>
  );
};
