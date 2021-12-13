import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Dialog,
  DialogContent,
  Grid,
} from "@material-ui/core";
import React from "react";
import MinijobSchedule from "../../../schedule/schedules/MinijobSchedule";
import TrainingSchedule, {
  TrainingType,
} from "../../../schedule/schedules/TrainingSchedule";
import { Schedule } from "../../../schedule/type";
import ScheduleButton from "./ScheduleButton";
import vocalBeginner from "../../../../public/image/training/thumbnail/vocalBeginner.png";
import vocalIntermediate from "../../../../public/image/training/thumbnail/vocalIntermediate.png";
import vocalAdvanced from "../../../../public/image/training/thumbnail/vocalAdvanced.png";
import danceBeginner from "../../../../public/image/training/thumbnail/danceBeginner.png";
import danceIntermediate from "../../../../public/image/training/thumbnail/danceIntermediate.png";
import danceAdvanced from "../../../../public/image/training/thumbnail/danceAdvanced.png";
import weightBeginner from "../../../../public/image/training/thumbnail/weightBeginner.png";
import weightIntermediate from "../../../../public/image/training/thumbnail/weightIntermediate.png";
import weightAdvanced from "../../../../public/image/training/thumbnail/weightAdvanced.png";
import songWriteBeginner from "../../../../public/image/training/thumbnail/songWriteBeginner.png";
import songWriteIntermediate from "../../../../public/image/training/thumbnail/songWriteIntermediate.png";
import songWriteAdvanced from "../../../../public/image/training/thumbnail/songWriteAdvanced.png";
import error from "../../../../public/image/error.png";
import BreakSchedule from "../../../schedule/schedules/BreakSchedule";

type ScheduleSelectorWindowProps = {
  open: boolean;
  onSelect: (schedule: Schedule) => void;
};

const trainingThumbnailSourceMap: Record<
  TrainingType,
  Record<number, string>
> = {
  vocal: {
    0: vocalBeginner,
    1: vocalIntermediate,
    2: vocalAdvanced,
  },
  dance: {
    0: danceBeginner,
    1: danceIntermediate,
    2: danceAdvanced,
  },
  weight: {
    0: weightBeginner,
    1: weightIntermediate,
    2: weightAdvanced,
  },
  songWrite: {
    0: songWriteBeginner,
    1: songWriteIntermediate,
    2: songWriteAdvanced,
  },
};

export default function ScheduleSelectorWindow(
  props: ScheduleSelectorWindowProps,
) {
  const { open, onSelect } = props;
  return (
    <Dialog open={open} style={{ zIndex: 1305 }}>
      <DialogContent>
        <Accordion>
          <AccordionSummary>트레이닝</AccordionSummary>
          <AccordionDetails>
            <Grid container spacing={2}>
              <ScheduleButton
                title="초급보컬"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["vocal"][0]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "vocal",
                      difficulty: 0,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="중급보컬"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["vocal"][1]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "vocal",
                      difficulty: 1,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="고급보컬"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["vocal"][2]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "vocal",
                      difficulty: 2,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="초급작곡"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["songWrite"][0]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "songWrite",
                      difficulty: 0,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="중급작곡"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["songWrite"][1]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "songWrite",
                      difficulty: 1,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="고급작곡"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["songWrite"][2]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "songWrite",
                      difficulty: 2,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="초급댄스"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["dance"][0]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "dance",
                      difficulty: 0,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="중급댄스"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["dance"][1]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "dance",
                      difficulty: 1,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="고급댄스"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["dance"][2]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "dance",
                      difficulty: 2,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="초급웨이트"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["weight"][0]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "weight",
                      difficulty: 0,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="중급웨이트"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["weight"][1]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "weight",
                      difficulty: 1,
                    }),
                  )
                }
              />
              <ScheduleButton
                title="고급웨이트"
                content="대충 느낌있는 글"
                thumbnail={trainingThumbnailSourceMap["weight"][2]}
                clickHandler={() =>
                  onSelect(
                    new TrainingSchedule({
                      subtype: "weight",
                      difficulty: 2,
                    }),
                  )
                }
              />
            </Grid>
          </AccordionDetails>
        </Accordion>
        <Accordion>
          <AccordionSummary>아르바이트</AccordionSummary>
          <AccordionDetails>
            <Grid container spacing={2}>
              <ScheduleButton
                title="행사"
                content="빵댕이 한번 흔들어 제껴봐!"
                thumbnail={error}
                clickHandler={() =>
                  onSelect(
                    new MinijobSchedule({
                      subtype: "event",
                    }),
                  )
                }
              />
              <ScheduleButton
                title="인형탈"
                content="이 안에는 인형만큼 귀여운 하연이가 있어요"
                thumbnail={error}
                clickHandler={() =>
                  onSelect(
                    new MinijobSchedule({
                      subtype: "mascotSuit",
                    }),
                  )
                }
              />
              <ScheduleButton
                title="축가"
                content="ㅊㅊ"
                thumbnail={error}
                clickHandler={() =>
                  onSelect(
                    new MinijobSchedule({
                      subtype: "weddingSong",
                    }),
                  )
                }
              />
              <ScheduleButton
                title="피팅모델"
                content="ㅇ"
                thumbnail={error}
                clickHandler={() =>
                  onSelect(
                    new MinijobSchedule({
                      subtype: "fittingModel",
                    }),
                  )
                }
              />
            </Grid>
          </AccordionDetails>
        </Accordion>
        <Accordion>
          <AccordionSummary>휴식</AccordionSummary>
          <AccordionDetails>
            <Grid container spacing={2}>
              <ScheduleButton
                title="자유시간"
                content="나는... 뽀로로... 노는게 제일 좋아..."
                thumbnail={error}
                clickHandler={() =>
                  onSelect(
                    new BreakSchedule({
                      name: "자유시간",
                    }),
                  )
                }
              />
            </Grid>
          </AccordionDetails>
        </Accordion>
      </DialogContent>
    </Dialog>
  );
}
