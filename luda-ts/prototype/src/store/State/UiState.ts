export type UiState = {
  breakWindow: boolean;
  communicationWindow: boolean;
  eatMealWindow: boolean;
  evaluationWindow: boolean;
  manageYoutubeWindow: boolean;
  minijobWindow: boolean;
  scheduleWindow: boolean;
  menuWindow: boolean;
  trainingWindow: boolean;
  notificationWindow: boolean;
  dawnWindow: boolean;
};

export const initialUiState: UiState = {
  breakWindow: false,
  communicationWindow: false,
  eatMealWindow: false,
  evaluationWindow: false,
  manageYoutubeWindow: false,
  minijobWindow: false,
  scheduleWindow: false,
  menuWindow: false,
  trainingWindow: false,
  notificationWindow: false,
  dawnWindow: false,
};
