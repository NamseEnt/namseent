import { LoadingStateWithTimeout } from "./type";

export function checkLoadingTimeout<
  LoadingState extends LoadingStateWithTimeout,
>(props: { state?: LoadingState; timeoutMs?: number; now?: number }) {
  if (!props.state?.isLoading) {
    return;
  }

  const { state, timeoutMs = 5000, now = Date.now() } = props;

  if (now > state.startedAt + timeoutMs) {
    state.isLoading = false;
    state.errorCode = "Timeout";
    return;
  }
}
