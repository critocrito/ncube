import {useMachine} from "@xstate/react";
import fetchMock, {FetchMockStatic} from "fetch-mock";
import React, {useEffect, useState} from "react";

import {NcubeProvider, WorkspaceProvider} from "../lib/context";
import {HttpDataResponse} from "../lib/http";
import PubSub from "../lib/pubsub";
import ncubeMachine from "../machines/ncube";
import workspaceMachine from "../machines/workspace";
import {localWorkspace as workspace} from "./data";

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
  }, [mock, matcher, response, method]);

  return <>{mock && children}</>;
};

interface WrapperProps {
  children?: React.ReactNode;
}

export const Wrapper = ({children}: WrapperProps) => {
  const [ncubeState, ncubeSend] = useMachine(ncubeMachine, {
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
    <NcubeProvider value={[ncubeState, ncubeSend]}>
      <WorkspaceProvider value={[workspaceState, workspaceSend]}>
        {children}
      </WorkspaceProvider>
    </NcubeProvider>
  );
};
