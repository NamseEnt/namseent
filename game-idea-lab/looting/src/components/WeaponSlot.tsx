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
}: WeaponSlotProps) {
  const definition = item ? getItemDefinition(item.definitionId) : null
  const borderColor = definition ? RARITY_COLORS[definition.rarity] : 0x444466

  const drawMainSlot = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: item ? 0x2a2a4a : 0x1a1a3a })
      g.setStrokeStyle({ width: 2, color: borderColor })
      g.roundRect(0, 0, width, height - 30, 6)
      g.fill()
      g.stroke()
    },
    [width, height, item, borderColor]
  )

  const labelStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 12,
    fill: 0x666688,
    align: 'center',
  }), [])

  const nameStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 12,
    fill: 0xffffff,
    align: 'center',
  }), [])

  const attachmentSlots = definition?.attachmentSlots || []
  const slotLabel = slotType === 'weapon1' ? 'Weapon 1' : 'Weapon 2'

  return (
    <pixiContainer x={x} y={y}>
      <pixiContainer eventMode="static" cursor={item ? 'pointer' : 'default'} onPointerDown={item && onDrop ? onDrop : undefined}>
        <pixiGraphics draw={drawMainSlot} />
        {!item && (
          <pixiText
            text={slotLabel}
            x={width / 2}
            y={(height - 30) / 2}
            anchor={{ x: 0.5, y: 0.5 }}
            style={labelStyle}
          />
        )}
        {item && definition && (
          <pixiText
            text={definition.name}
            x={width / 2}
            y={(height - 30) / 2}
            anchor={{ x: 0.5, y: 0.5 }}
            style={nameStyle}
          />
        )}
      </pixiContainer>

      <pixiContainer y={height - 26}>
        {(['scope', 'extendedMag', 'barrel'] as AttachmentType[]).map((attachType, i) => {
          const hasSlot = attachmentSlots.includes(attachType)
          const attachment = item?.attachments?.[attachType]
          const attachDef = attachment ? getItemDefinition(attachment.definitionId) : null
          const attachColor = attachDef ? RARITY_COLORS[attachDef.rarity] : 0x333355

          if (!hasSlot && !item) return null

          return (
            <AttachmentSlotItem
              key={attachType}
              x={i * 54}
              attachType={attachType}
              hasSlot={hasSlot}
              attachment={attachment}
              attachDef={attachDef}
              attachColor={attachColor}
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
  attachDef: ReturnType<typeof getItemDefinition>
  attachColor: number
}

function AttachmentSlotItem({ x, attachType, hasSlot, attachment, attachDef, attachColor }: AttachmentSlotItemProps) {
  const drawAttachmentSlot = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: attachment ? 0x2a2a4a : 0x1a1a3a })
      g.setStrokeStyle({ width: 1, color: hasSlot ? attachColor : 0x222233 })
      g.roundRect(0, 0, 50, 24, 4)
      g.fill()
      g.stroke()
    },
    [attachment, hasSlot, attachColor]
  )

  const textStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 8,
    fill: hasSlot ? (attachment ? 0xffffff : 0x666688) : 0x333355,
    align: 'center',
  }), [hasSlot, attachment])

  return (
    <pixiContainer x={x}>
      <pixiGraphics draw={drawAttachmentSlot} />
      <pixiText
        text={attachment ? attachDef?.name.substring(0, 6) || '' : ATTACHMENT_LABELS[attachType]}
        x={25}
        y={12}
        anchor={{ x: 0.5, y: 0.5 }}
        style={textStyle}
      />
    </pixiContainer>
  )
}
