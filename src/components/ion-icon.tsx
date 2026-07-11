type IonIconProps = {
  name: string
  variant: 'filled' | 'outline' | 'sharp'
  size?: 'small' | 'large'
  strokeWidth?: number,
  className?: string
  style?: React.CSSProperties
}

export default function IonIcon({ name, variant, size = 'small', strokeWidth = 32, className, style = {} }: IonIconProps) {
  const iconName = variant === 'filled' ? name : `${name}-${variant}`

  return (
    // @ts-expect-error
    <ion-icon name={iconName} size={size} style={{ '--ionicon-stroke-width': strokeWidth, ...style }} className={className}></ion-icon>
  )
}
