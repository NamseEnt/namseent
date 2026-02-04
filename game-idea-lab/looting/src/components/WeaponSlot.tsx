import { useCallback, useMemo } from 'react'
import { Graphics, TextStyle } from 'pixi.js'
import type { ItemInstance, WeaponSlotType, AttachmentType } from '../types/items'
import { RARITY_COLORS } from '../types/items'
import { getItemDefinition } from '../data/itemDefinitions'

interface WeaponSlotProps {
  x: number
  y: number
  width: number
  height: number
  slotType: WeaponSlotType
  item: ItemInstance | null
  onDrop?: () => void
  onAttachmentDragStart?: (attachmentType: AttachmentType) => void
  onWeaponDrop?: () => void
  isDragging?: boolean
}

const ATTACHMENT_LABELS: Record<AttachmentType, string> = {
  scope: 'Scope',
  extendedMag: 'Mag',
  barrel: 'Barrel',
}

export function WeaponSlot({
  x,
  y,
  width,
  height,
  slotType,
  item,
  onDrop,
  onAttachmentDragStart,
  onWeaponDrop,
  isDragging,
}: WeaponSlotProps) {
  const definition = item ? getItemDefinition(item.definitionId) : null
  const borderColor = definition ? RARITY_COLORS[definition.rarity] : 0x444466

  const labelStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 18,
    fill: 0x666688,
    align: 'center',
  }), [])

  const nameStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 18,
    fill: 0xffffff,
    align: 'center',
  }), [])

  const attachmentSlots = definition?.attachmentSlots || []
  const slotLabel = slotType === 'weapon1' ? 'Weapon 1' : 'Weapon 2'

  const handleWeaponClick = useCallback(() => {
    if (isDragging && onWeaponDrop) {
      onWeaponDrop()
    } else if (item && onDrop) {
      onDrop()
    }
  }, [isDragging, onWeaponDrop, item, onDrop])

  const dropHighlightColor = isDragging && item && definition?.attachmentSlots ? 0x44ff44 : borderColor

  const drawMainSlotWithHighlight = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: item ? 0x2a2a4a : 0x1a1a3a })
      g.setStrokeStyle({ width: isDragging && item ? 3 : 2, color: dropHighlightColor })
      g.roundRect(0, 0, width, height - 45, 6)
      g.fill()
      g.stroke()
    },
    [width, height, item, dropHighlightColor, isDragging]
  )

  return (
    <pixiContainer x={x} y={y}>
      <pixiContainer eventMode="static" cursor={item || isDragging ? 'pointer' : 'default'} onPointerDown={handleWeaponClick}>
        <pixiGraphics draw={drawMainSlotWithHighlight} />
        {!item && (
          <pixiText
            text={slotLabel}
            x={width / 2}
            y={(height - 45) / 2}
            anchor={{ x: 0.5, y: 0.5 }}
            style={labelStyle}
          />
        )}
        {item && definition && (
          <pixiText
            text={definition.name}
            x={width / 2}
            y={(height - 45) / 2}
            anchor={{ x: 0.5, y: 0.5 }}
            style={nameStyle}
          />
        )}
      </pixiContainer>

      <pixiContainer y={height - 39}>
        {(['scope', 'extendedMag', 'barrel'] as AttachmentType[]).map((attachType, i) => {
          const hasSlot = attachmentSlots.includes(attachType)
          const attachment = item?.attachments?.[attachType]
          const attachDef = attachment ? getItemDefinition(attachment.definitionId) : null
          const attachColor = attachDef ? RARITY_COLORS[attachDef.rarity] : 0x333355

          if (!hasSlot && !item) return null

          return (
            <AttachmentSlotItem
              key={attachType}
              x={i * 81}
              attachType={attachType}
              hasSlot={hasSlot}
              attachment={attachment}
              attachDef={attachDef}
              attachColor={attachColor}
              onDragStart={attachment && onAttachmentDragStart ? () => onAttachmentDragStart(attachType) : undefined}
            />
          )
        })}
      </pixiContainer>
    </pixiContainer>
  )
}

interface AttachmentSlotItemProps {
  x: number
  attachType: AttachmentType
  hasSlot: boolean
  attachment: ItemInstance | null | undefined
  attachDef: ReturnType<typeof getItemDefinition> | null
  attachColor: number
  onDragStart?: () => void
}

function AttachmentSlotItem({ x, attachType, hasSlot, attachment, attachDef, attachColor, onDragStart }: AttachmentSlotItemProps) {
  const drawAttachmentSlot = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: attachment ? 0x2a2a4a : 0x1a1a3a })
      g.setStrokeStyle({ width: 1, color: hasSlot ? attachColor : 0x222233 })
      g.roundRect(0, 0, 75, 36, 6)
      g.fill()
      g.stroke()
    },
    [attachment, hasSlot, attachColor]
  )

  const textStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 12,
    fill: hasSlot ? (attachment ? 0xffffff : 0x666688) : 0x333355,
    align: 'center',
  }), [hasSlot, attachment])

  return (
    <pixiContainer
      x={x}
      eventMode={attachment ? 'static' : 'auto'}
      cursor={attachment ? 'grab' : 'default'}
      onPointerDown={onDragStart}
    >
      <pixiGraphics draw={drawAttachmentSlot} />
      <pixiText
        text={attachment ? attachDef?.name.substring(0, 6) || '' : ATTACHMENT_LABELS[attachType]}
        x={37}
        y={18}
        anchor={{ x: 0.5, y: 0.5 }}
        style={textStyle}
      />
    </pixiContainer>
  )
}
