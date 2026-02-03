import { Application, extend } from "@pixi/react";
import { Container, Graphics, Text, TextStyle } from "pixi.js";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";

extend({ Container, Graphics, Text });

const HEX_SIZE = 30;
const GRID_SIZE = 5;

interface CubeCoord {
  q: number;
  r: number;
  s: number;
}

interface CharacterData {
  label: string;
  type: "ally" | "enemy";
  coord: CubeCoord;
  hp: number;
  attack: number;
  range: number;
}

interface AttackAnimation {
  attackerLabel: string;
  targetLabel: string;
  targetCoord: CubeCoord;
  damage: number;
  progress: number;
}

function cubeToOffset(coord: CubeCoord): { col: number; row: number } {
  const col = coord.q;
  const row = coord.r + Math.floor((coord.q - (coord.q & 1)) / 2);
  return { col, row };
}

function cubeToPixel(coord: CubeCoord, size: number) {
  const { col, row } = cubeToOffset(coord);
  const width = size * 2;
  const height = Math.sqrt(3) * size;
  const x = col * width * 0.75;
  const y = row * height + (col % 2 === 1 ? height / 2 : 0);
  return { x: x + 80, y: y + 80 };
}

function getHexCorners(size: number) {
  const corners: { x: number; y: number }[] = [];
  for (let i = 0; i < 6; i++) {
    const angleDeg = 60 * i;
    const angleRad = (Math.PI / 180) * angleDeg;
    corners.push({
      x: size * Math.cos(angleRad),
      y: size * Math.sin(angleRad),
    });
  }
  return corners;
}

function offsetToCube(col: number, row: number): CubeCoord {
  const q = col;
  const r = row - Math.floor((col - (col & 1)) / 2);
  const s = -q - r;
  return { q, r, s };
}

function generateHexGrid(cols: number, rows: number): CubeCoord[] {
  const hexes: CubeCoord[] = [];
  for (let row = 0; row < rows; row++) {
    for (let col = 0; col < cols; col++) {
      hexes.push(offsetToCube(col, row));
    }
  }
  return hexes;
}

function shuffle<T>(array: T[]): T[] {
  const result = [...array];
  for (let i = result.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [result[i], result[j]] = [result[j], result[i]];
  }
  return result;
}

function coordEquals(a: CubeCoord, b: CubeCoord): boolean {
  return a.q === b.q && a.r === b.r && a.s === b.s;
}

const CUBE_DIRECTIONS: CubeCoord[] = [
  { q: +1, r: 0, s: -1 },
  { q: +1, r: -1, s: 0 },
  { q: 0, r: -1, s: +1 },
  { q: -1, r: 0, s: +1 },
  { q: -1, r: +1, s: 0 },
  { q: 0, r: +1, s: -1 },
];

function getNeighbors(coord: CubeCoord): CubeCoord[] {
  return CUBE_DIRECTIONS.map((dir) => ({
    q: coord.q + dir.q,
    r: coord.r + dir.r,
    s: coord.s + dir.s,
  }));
}

function cubeDistance(a: CubeCoord, b: CubeCoord): number {
  return Math.max(
    Math.abs(a.q - b.q),
    Math.abs(a.r - b.r),
    Math.abs(a.s - b.s)
  );
}

function isValidCoord(coord: CubeCoord): boolean {
  const { col, row } = cubeToOffset(coord);
  return col >= 0 && col < GRID_SIZE && row >= 0 && row < GRID_SIZE;
}

function getCharacterAt(
  coord: CubeCoord,
  characters: CharacterData[]
): CharacterData | null {
  return characters.find((c) => c.hp > 0 && coordEquals(c.coord, coord)) || null;
}

function getAdjacentTargets(
  coord: CubeCoord,
  targetType: "ally" | "enemy",
  characters: CharacterData[]
): CharacterData[] {
  const neighbors = getNeighbors(coord);
  return characters.filter(
    (c) =>
      c.hp > 0 &&
      c.type === targetType &&
      neighbors.some((n) => coordEquals(n, c.coord))
  );
}

function findClosestTarget(
  from: CubeCoord,
  targetType: "ally" | "enemy",
  characters: CharacterData[]
): CharacterData | null {
  const targets = characters.filter((c) => c.hp > 0 && c.type === targetType);
  if (targets.length === 0) return null;

  let closest = targets[0];
  let closestDist = cubeDistance(from, closest.coord);

  for (const target of targets) {
    const dist = cubeDistance(from, target.coord);
    if (dist < closestDist) {
      closestDist = dist;
      closest = target;
    }
  }

  return closest;
}

function getStepTowards(
  from: CubeCoord,
  to: CubeCoord,
  characters: CharacterData[]
): CubeCoord | null {
  const neighbors = getNeighbors(from);
  const emptyNeighbors = neighbors.filter(
    (coord) => isValidCoord(coord) && !getCharacterAt(coord, characters)
  );

  if (emptyNeighbors.length === 0) return null;

  let bestCoord = emptyNeighbors[0];
  let bestDistance = cubeDistance(bestCoord, to);

  for (const coord of emptyNeighbors) {
    const distance = cubeDistance(coord, to);
    if (distance < bestDistance) {
      bestDistance = distance;
      bestCoord = coord;
    }
  }

  return bestCoord;
}

interface HexagonProps {
  coord: CubeCoord;
  size: number;
  selected?: boolean;
  highlightType?: "move" | "attack" | null;
  onClick?: () => void;
}

function Hexagon({ coord, size, selected, highlightType, onClick }: HexagonProps) {
  const { x, y } = cubeToPixel(coord, size);

  const draw = useCallback(
    (g: Graphics) => {
      const corners = getHexCorners(size);

      g.clear();

      let strokeColor = 0x333333;
      let fillColor = 0x4a90d9;

      if (selected) {
        strokeColor = 0xffff00;
        fillColor = 0x6ab0e8;
      } else if (highlightType === "move") {
        strokeColor = 0x00ff00;
        fillColor = 0x88ffaa;
      } else if (highlightType === "attack") {
        strokeColor = 0xff0000;
        fillColor = 0xff8888;
      }

      g.setStrokeStyle({ width: 2, color: strokeColor });
      g.setFillStyle({ color: fillColor });

      g.moveTo(corners[0].x, corners[0].y);
      for (let i = 1; i < corners.length; i++) {
        g.lineTo(corners[i].x, corners[i].y);
      }
      g.closePath();
      g.fill();
      g.stroke();
    },
    [size, selected, highlightType]
  );

  return (
    <pixiGraphics
      x={x}
      y={y}
      draw={draw}
      eventMode="static"
      cursor="pointer"
      onPointerDown={onClick}
    />
  );
}

interface CharacterProps {
  coord: CubeCoord;
  size: number;
  type: "ally" | "enemy";
  label: string;
  hp: number;
  offsetX?: number;
  offsetY?: number;
  hpFlash?: boolean;
}

function Character({ coord, size, type, label, hp, offsetX = 0, offsetY = 0, hpFlash }: CharacterProps) {
  const { x, y } = cubeToPixel(coord, size);

  const draw = useCallback(
    (g: Graphics) => {
      g.clear();

      if (type === "ally") {
        g.setFillStyle({ color: 0x00ff00 });
        g.circle(0, 0, size * 0.4);
        g.fill();
      } else {
        g.setFillStyle({ color: 0xff0000 });
        const triSize = size * 0.5;
        g.moveTo(0, -triSize);
        g.lineTo(triSize * 0.866, triSize * 0.5);
        g.lineTo(-triSize * 0.866, triSize * 0.5);
        g.closePath();
        g.fill();
      }
    },
    [size, type]
  );

  const textStyle = useMemo(
    () =>
      new TextStyle({
        fontSize: size * 0.5,
        fill: type === "ally" ? 0x000000 : 0xffffff,
        fontWeight: "bold",
      }),
    [size, type]
  );

  const hpStyle = useMemo(
    () =>
      new TextStyle({
        fontSize: size * 0.3,
        fill: hpFlash ? 0xff0000 : type === "ally" ? 0x004400 : 0xffcccc,
      }),
    [size, type, hpFlash]
  );

  return (
    <pixiContainer x={x + offsetX} y={y + offsetY}>
      <pixiGraphics draw={draw} />
      <pixiText text={label} anchor={{ x: 0.5, y: 0.7 }} style={textStyle} />
      <pixiText text={String(hp)} anchor={{ x: 0.5, y: -0.3 }} style={hpStyle} />
    </pixiContainer>
  );
}

interface TurnQueueProps {
  queue: CharacterData[];
}

function TurnQueue({ queue }: TurnQueueProps) {
  const itemSize = 24;
  const spacing = 8;

  return (
    <pixiContainer x={10} y={10}>
      {queue.map((char, i) => (
        <TurnQueueItem
          key={char.label}
          character={char}
          index={i}
          size={itemSize}
          spacing={spacing}
        />
      ))}
    </pixiContainer>
  );
}

interface TurnQueueItemProps {
  character: CharacterData;
  index: number;
  size: number;
  spacing: number;
}

function TurnQueueItem({ character, index, size, spacing }: TurnQueueItemProps) {
  const x = index * (size + spacing) + size / 2;
  const y = size / 2;

  const draw = useCallback(
    (g: Graphics) => {
      g.clear();

      if (character.type === "ally") {
        g.setFillStyle({ color: 0x00ff00 });
        g.circle(0, 0, size * 0.4);
        g.fill();
      } else {
        g.setFillStyle({ color: 0xff0000 });
        const triSize = size * 0.4;
        g.moveTo(0, -triSize);
        g.lineTo(triSize * 0.866, triSize * 0.5);
        g.lineTo(-triSize * 0.866, triSize * 0.5);
        g.closePath();
        g.fill();
      }
    },
    [size, character.type]
  );

  const textStyle = useMemo(
    () =>
      new TextStyle({
        fontSize: size * 0.4,
        fill: character.type === "ally" ? 0x000000 : 0xffffff,
        fontWeight: "bold",
      }),
    [size, character.type]
  );

  return (
    <pixiContainer x={x} y={y}>
      <pixiGraphics draw={draw} />
      <pixiText text={character.label} anchor={0.5} style={textStyle} />
    </pixiContainer>
  );
}

interface CharacterInfoPanelProps {
  character: CharacterData | null;
}

function CharacterInfoPanel({ character }: CharacterInfoPanelProps) {
  const panelWidth = 150;
  const panelHeight = 145;

  const drawPanel = useCallback(
    (g: Graphics) => {
      g.clear();
      g.setFillStyle({ color: 0x222244, alpha: 0.9 });
      g.setStrokeStyle({ width: 2, color: 0x4444aa });
      g.roundRect(0, 0, panelWidth, panelHeight, 8);
      g.fill();
      g.stroke();
    },
    []
  );

  const titleStyle = useMemo(
    () =>
      new TextStyle({
        fontSize: 14,
        fill: 0xaaaaaa,
      }),
    []
  );

  const nameStyle = useMemo(
    () =>
      new TextStyle({
        fontSize: 20,
        fill: character?.type === "ally" ? 0x00ff00 : 0xff6666,
        fontWeight: "bold",
      }),
    [character?.type]
  );

  const labelStyle = useMemo(
    () =>
      new TextStyle({
        fontSize: 14,
        fill: 0xaaaaaa,
      }),
    []
  );

  const valueStyle = useMemo(
    () =>
      new TextStyle({
        fontSize: 18,
        fill: 0xffffff,
        fontWeight: "bold",
      }),
    []
  );

  if (!character) return null;

  return (
    <pixiContainer x={580} y={80}>
      <pixiGraphics draw={drawPanel} />
      <pixiText x={10} y={10} text="Character" style={titleStyle} />
      <pixiText x={10} y={30} text={character.label.toUpperCase()} style={nameStyle} />
      <pixiText x={10} y={60} text="HP" style={labelStyle} />
      <pixiText x={50} y={58} text={String(character.hp)} style={valueStyle} />
      <pixiText x={10} y={85} text="ATK" style={labelStyle} />
      <pixiText x={50} y={83} text={String(character.attack)} style={valueStyle} />
      <pixiText x={10} y={110} text="RNG" style={labelStyle} />
      <pixiText x={50} y={108} text={String(character.range)} style={valueStyle} />
    </pixiContainer>
  );
}

interface ButtonProps {
  x: number;
  y: number;
  width: number;
  height: number;
  text: string;
  disabled?: boolean;
  active?: boolean;
  onClick: () => void;
}

function Button({ x, y, width, height, text, disabled, active, onClick }: ButtonProps) {
  const draw = useCallback(
    (g: Graphics) => {
      g.clear();
      const bgColor = disabled ? 0x444444 : active ? 0x66aa66 : 0x446688;
      const strokeColor = disabled ? 0x555555 : active ? 0x88cc88 : 0x6688aa;
      g.setFillStyle({ color: bgColor });
      g.setStrokeStyle({ width: 2, color: strokeColor });
      g.roundRect(0, 0, width, height, 4);
      g.fill();
      g.stroke();
    },
    [width, height, disabled, active]
  );

  const textStyle = useMemo(
    () =>
      new TextStyle({
        fontSize: 14,
        fill: disabled ? 0x888888 : 0xffffff,
        fontWeight: "bold",
      }),
    [disabled]
  );

  return (
    <pixiContainer x={x} y={y}>
      <pixiGraphics
        draw={draw}
        eventMode={disabled ? "none" : "static"}
        cursor={disabled ? "default" : "pointer"}
        onPointerDown={disabled ? undefined : onClick}
      />
      <pixiText x={width / 2} y={height / 2} anchor={0.5} text={text} style={textStyle} />
    </pixiContainer>
  );
}

interface ActionPanelProps {
  isAllyTurn: boolean;
  onEndTurn: () => void;
}

function ActionPanel({ isAllyTurn, onEndTurn }: ActionPanelProps) {
  const panelWidth = 150;
  const panelHeight = 50;

  const drawPanel = useCallback((g: Graphics) => {
    g.clear();
    g.setFillStyle({ color: 0x222244, alpha: 0.9 });
    g.setStrokeStyle({ width: 2, color: 0x4444aa });
    g.roundRect(0, 0, panelWidth, panelHeight, 8);
    g.fill();
    g.stroke();
  }, []);

  if (!isAllyTurn) return null;

  return (
    <pixiContainer x={580} y={220}>
      <pixiGraphics draw={drawPanel} />
      <Button
        x={10}
        y={10}
        width={130}
        height={30}
        text="턴 종료"
        onClick={onEndTurn}
      />
    </pixiContainer>
  );
}

const initialCharacters: CharacterData[] = [
  { label: "a", type: "ally", coord: offsetToCube(0, 3), hp: 25, attack: 4, range: 1 },
  { label: "b", type: "ally", coord: offsetToCube(1, 3), hp: 40, attack: 3, range: 1 },
  { label: "c", type: "ally", coord: offsetToCube(0, 4), hp: 20, attack: 2, range: 2 },
  { label: "d", type: "ally", coord: offsetToCube(1, 4), hp: 25, attack: 4, range: 1 },
  { label: "z", type: "enemy", coord: offsetToCube(3, 0), hp: 10, attack: 3, range: 1 },
  { label: "y", type: "enemy", coord: offsetToCube(4, 0), hp: 10, attack: 3, range: 1 },
  { label: "x", type: "enemy", coord: offsetToCube(3, 1), hp: 10, attack: 3, range: 1 },
  { label: "w", type: "enemy", coord: offsetToCube(4, 1), hp: 10, attack: 3, range: 1 },
  { label: "v", type: "enemy", coord: offsetToCube(2, 0), hp: 10, attack: 3, range: 1 },
  { label: "u", type: "enemy", coord: offsetToCube(2, 1), hp: 10, attack: 3, range: 1 },
];

function HexGrid() {
  const hexes = generateHexGrid(GRID_SIZE, GRID_SIZE);
  const [characters, setCharacters] = useState<CharacterData[]>(initialCharacters);
  const [turnQueue] = useState<CharacterData[]>(() => shuffle(initialCharacters));
  const [currentTurnLabel, setCurrentTurnLabel] = useState<string>(() => shuffle(initialCharacters)[0].label);
  const [selectedCoord, setSelectedCoord] = useState<CubeCoord | null>(null);
  const [hasMoved, setHasMoved] = useState(false);
  const [hasAttacked, setHasAttacked] = useState(false);
  const [attackAnimation, setAttackAnimation] = useState<AttackAnimation | null>(null);
  const animationRef = useRef<number | null>(null);

  const aliveQueue = useMemo(() => {
    return turnQueue.filter((char) =>
      characters.find((c) => c.label === char.label && c.hp > 0)
    );
  }, [turnQueue, characters]);

  const rotatedQueue = useMemo(() => {
    if (aliveQueue.length === 0) return [];
    const currentIndex = aliveQueue.findIndex((c) => c.label === currentTurnLabel);
    if (currentIndex === -1) {
      return aliveQueue;
    }
    return [...aliveQueue.slice(currentIndex), ...aliveQueue.slice(0, currentIndex)];
  }, [aliveQueue, currentTurnLabel]);

  const currentCharacter = useMemo(() => {
    if (rotatedQueue.length === 0) return null;
    return characters.find((c) => c.label === rotatedQueue[0]?.label) || null;
  }, [rotatedQueue, characters]);

  const selectedCharacter = useMemo(() => {
    if (!selectedCoord) return null;
    return characters.find((c) => c.hp > 0 && coordEquals(c.coord, selectedCoord)) || null;
  }, [selectedCoord, characters]);

  const isAllyTurn = currentCharacter?.type === "ally";

  const isSelectedCurrentTurn = useMemo(() => {
    return selectedCharacter && currentCharacter &&
      selectedCharacter.label === currentCharacter.label;
  }, [selectedCharacter, currentCharacter]);

  const movableCells = useMemo(() => {
    if (!isSelectedCurrentTurn || !currentCharacter || hasMoved || hasAttacked) {
      return [];
    }
    const cells: CubeCoord[] = [];
    for (let dq = -2; dq <= 2; dq++) {
      for (let dr = -2; dr <= 2; dr++) {
        const ds = -dq - dr;
        if (Math.abs(ds) > 2) continue;
        if (dq === 0 && dr === 0) continue;
        const coord = {
          q: currentCharacter.coord.q + dq,
          r: currentCharacter.coord.r + dr,
          s: currentCharacter.coord.s + ds,
        };
        if (cubeDistance(currentCharacter.coord, coord) <= 2 &&
            isValidCoord(coord) &&
            !getCharacterAt(coord, characters)) {
          cells.push(coord);
        }
      }
    }
    return cells;
  }, [isSelectedCurrentTurn, currentCharacter, hasMoved, hasAttacked, characters]);

  const attackableEnemies = useMemo(() => {
    if (!isSelectedCurrentTurn || !currentCharacter || hasAttacked) {
      return [];
    }
    return characters.filter(
      (c) =>
        c.hp > 0 &&
        c.type === "enemy" &&
        cubeDistance(currentCharacter.coord, c.coord) <= currentCharacter.range
    );
  }, [isSelectedCurrentTurn, currentCharacter, hasAttacked, characters]);

  const advanceTurn = useCallback(() => {
    if (aliveQueue.length === 0) return;
    const currentIndex = aliveQueue.findIndex((c) => c.label === currentTurnLabel);
    const nextIndex = (currentIndex + 1) % aliveQueue.length;
    const nextChar = aliveQueue[nextIndex];
    setCurrentTurnLabel(nextChar.label);
    setHasMoved(false);
    setHasAttacked(false);
    const nextCharData = characters.find((c) => c.label === nextChar.label);
    if (nextCharData?.type === "ally") {
      setSelectedCoord(nextCharData.coord);
    } else {
      setSelectedCoord(null);
    }
  }, [aliveQueue, currentTurnLabel, characters]);

  const enemyTurnProcessed = useRef<string | null>(null);
  const enemyAttackApplied = useRef<string | null>(null);
  const damageApplied = useRef(false);
  const [flashingTarget, setFlashingTarget] = useState<string | null>(null);

  useEffect(() => {
    if (currentCharacter?.type === "ally") {
      setSelectedCoord(currentCharacter.coord);
    }
  }, [currentCharacter?.label]);

  useEffect(() => {
    if (!attackAnimation) {
      damageApplied.current = false;
      return;
    }

    const attacker = characters.find((c) => c.label === attackAnimation.attackerLabel);
    if (!attacker) return;

    const startTime = performance.now();
    const duration = 400;

    const animate = (currentTime: number) => {
      const elapsed = currentTime - startTime;
      const progress = Math.min(elapsed / duration, 1);

      setAttackAnimation((prev) => (prev ? { ...prev, progress } : null));

      if (progress >= 0.5 && !damageApplied.current) {
        damageApplied.current = true;
        setFlashingTarget(attackAnimation.targetLabel);

        setCharacters((prev) =>
          prev.map((c) =>
            c.label === attackAnimation.targetLabel
              ? { ...c, hp: Math.max(0, c.hp - attackAnimation.damage) }
              : c
          )
        );

        setTimeout(() => setFlashingTarget(null), 200);
      }

      if (progress < 1) {
        animationRef.current = requestAnimationFrame(animate);
      } else {
        setAttackAnimation(null);
      }
    };

    animationRef.current = requestAnimationFrame(animate);

    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [attackAnimation?.attackerLabel, attackAnimation?.targetLabel]);

  useEffect(() => {
    if (!currentCharacter || currentCharacter.type !== "enemy") {
      enemyTurnProcessed.current = null;
      return;
    }

    if (enemyTurnProcessed.current === currentCharacter.label) {
      return;
    }

    enemyTurnProcessed.current = currentCharacter.label;
    enemyAttackApplied.current = null;
    const enemyLabel = currentCharacter.label;

    const timer = setTimeout(() => {
      setCharacters((prevCharacters) => {
        let newCharacters = [...prevCharacters];
        const enemyIndex = newCharacters.findIndex((c) => c.label === enemyLabel);
        if (enemyIndex === -1 || newCharacters[enemyIndex].hp <= 0) {
          return prevCharacters;
        }

        const enemy = newCharacters[enemyIndex];

        const adjacentAllies = getAdjacentTargets(enemy.coord, "ally", newCharacters);

        if (adjacentAllies.length === 0) {
          const closestAlly = findClosestTarget(enemy.coord, "ally", newCharacters);
          if (closestAlly) {
            const newCoord = getStepTowards(enemy.coord, closestAlly.coord, newCharacters);
            if (newCoord) {
              newCharacters = newCharacters.map((c) =>
                c.label === enemy.label ? { ...c, coord: newCoord } : c
              );
            }
          }
        }

        const updatedEnemy = newCharacters.find((c) => c.label === enemyLabel);
        if (updatedEnemy) {
          const adjacentAlliesAfterMove = getAdjacentTargets(
            updatedEnemy.coord,
            "ally",
            newCharacters
          );

          if (adjacentAlliesAfterMove.length > 0 && enemyAttackApplied.current !== enemyLabel) {
            enemyAttackApplied.current = enemyLabel;
            const target = adjacentAlliesAfterMove[0];
            newCharacters = newCharacters.map((c) =>
              c.label === target.label
                ? { ...c, hp: Math.max(0, c.hp - enemy.attack) }
                : c
            );
          }
        }

        return newCharacters;
      });

      advanceTurn();
    }, 1000);

    return () => clearTimeout(timer);
  }, [currentCharacter?.label, advanceTurn]);

  const handleHexClick = useCallback(
    (coord: CubeCoord) => {
      if (attackAnimation) return;

      if (isSelectedCurrentTurn && currentCharacter) {
        const isMovableCell = movableCells.some((c) => coordEquals(c, coord));
        if (isMovableCell && !hasMoved && !hasAttacked) {
          setCharacters((prev) =>
            prev.map((c) =>
              c.label === currentCharacter.label ? { ...c, coord } : c
            )
          );
          setSelectedCoord(coord);
          setHasMoved(true);
          return;
        }

        const targetEnemy = attackableEnemies.find((e) => coordEquals(e.coord, coord));
        if (targetEnemy && !attackAnimation && !hasAttacked) {
          setAttackAnimation({
            attackerLabel: currentCharacter.label,
            targetLabel: targetEnemy.label,
            targetCoord: targetEnemy.coord,
            damage: currentCharacter.attack,
            progress: 0,
          });
          setHasAttacked(true);
          return;
        }
      }

      setSelectedCoord(coord);
    },
    [isSelectedCurrentTurn, currentCharacter, movableCells, attackableEnemies, hasMoved, hasAttacked, attackAnimation]
  );

  const handleEndTurn = useCallback(() => {
    advanceTurn();
  }, [advanceTurn]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.code === "Space" && isAllyTurn && !attackAnimation) {
        e.preventDefault();
        advanceTurn();
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [isAllyTurn, attackAnimation, advanceTurn]);

  const getHighlightType = useCallback(
    (coord: CubeCoord): "move" | "attack" | null => {
      if (movableCells.some((c) => coordEquals(c, coord))) return "move";
      if (attackableEnemies.some((e) => coordEquals(e.coord, coord))) return "attack";
      return null;
    },
    [movableCells, attackableEnemies]
  );

  return (
    <pixiContainer>
      {hexes.map((coord) => (
        <Hexagon
          key={`${coord.q},${coord.r},${coord.s}`}
          coord={coord}
          size={HEX_SIZE}
          selected={selectedCoord !== null && coordEquals(coord, selectedCoord)}
          highlightType={getHighlightType(coord)}
          onClick={() => handleHexClick(coord)}
        />
      ))}
      {characters.filter((char) => char.hp > 0).map((char) => {
        let offsetX = 0;
        let offsetY = 0;

        if (attackAnimation && char.label === attackAnimation.attackerLabel) {
          const attacker = characters.find((c) => c.label === attackAnimation.attackerLabel);
          if (attacker) {
            const attackerPos = cubeToPixel(attacker.coord, HEX_SIZE);
            const targetPos = cubeToPixel(attackAnimation.targetCoord, HEX_SIZE);
            const dx = targetPos.x - attackerPos.x;
            const dy = targetPos.y - attackerPos.y;

            const t = attackAnimation.progress < 0.5
              ? attackAnimation.progress * 2
              : (1 - attackAnimation.progress) * 2;

            offsetX = dx * t * 0.7;
            offsetY = dy * t * 0.7;
          }
        }

        const isFlashing = flashingTarget === char.label;

        return (
          <Character
            key={char.label}
            coord={char.coord}
            size={HEX_SIZE}
            type={char.type}
            label={char.label}
            hp={char.hp}
            offsetX={offsetX}
            offsetY={offsetY}
            hpFlash={isFlashing}
          />
        );
      })}
      <TurnQueue queue={rotatedQueue} />
      <CharacterInfoPanel character={selectedCharacter} />
      <ActionPanel isAllyTurn={isAllyTurn} onEndTurn={handleEndTurn} />
    </pixiContainer>
  );
}

function App() {
  useEffect(() => {
    const handleContextMenu = (e: MouseEvent) => e.preventDefault();
    document.addEventListener("contextmenu", handleContextMenu);
    return () => document.removeEventListener("contextmenu", handleContextMenu);
  }, []);

  return (
    <Application background="#1a1a2e" resizeTo={window}>
      <HexGrid />
    </Application>
  );
}

export default App;
