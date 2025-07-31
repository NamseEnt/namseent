export const CueFlash = () => {
  return (
    <div className="absolute inset-0 pointer-events-none">
      <svg className="w-full h-full" viewBox="0 0 100 100" preserveAspectRatio="none">
        <defs>
          <linearGradient id="flashGradient" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stopColor="#facc15" stopOpacity="0" />
            <stop offset="45%" stopColor="#fbbf24" stopOpacity="0.8" />
            <stop offset="50%" stopColor="#fde047" stopOpacity="1" />
            <stop offset="55%" stopColor="#fbbf24" stopOpacity="0.8" />
            <stop offset="100%" stopColor="#facc15" stopOpacity="0" />
          </linearGradient>
          <linearGradient id="flashGradientH" x1="0%" y1="0%" x2="100%" y2="0%">
            <stop offset="0%" stopColor="#facc15" stopOpacity="0" />
            <stop offset="45%" stopColor="#fbbf24" stopOpacity="0.8" />
            <stop offset="50%" stopColor="#fde047" stopOpacity="1" />
            <stop offset="55%" stopColor="#fbbf24" stopOpacity="0.8" />
            <stop offset="100%" stopColor="#facc15" stopOpacity="0" />
          </linearGradient>
          <filter id="glow">
            <feGaussianBlur stdDeviation="2" result="coloredBlur"/>
            <feMerge>
              <feMergeNode in="coloredBlur"/>
              <feMergeNode in="SourceGraphic"/>
            </feMerge>
          </filter>
        </defs>
        
        {/* 수직 빛줄기 */}
        <path
          d="M 50 0 L 52 45 L 54 48 L 50 50 L 46 48 L 48 45 Z M 50 100 L 48 55 L 46 52 L 50 50 L 54 52 L 52 55 Z"
          fill="url(#flashGradient)"
          filter="url(#glow)"
          className="animate-pulse"
        />
        
        {/* 수평 빛줄기 */}
        <path
          d="M 0 50 L 45 48 L 48 46 L 50 50 L 48 54 L 45 52 Z M 100 50 L 55 52 L 52 54 L 50 50 L 52 46 L 55 48 Z"
          fill="url(#flashGradientH)"
          filter="url(#glow)"
          className="animate-pulse"
        />
        
        {/* 중앙 플래시 */}
        <circle
          cx="50"
          cy="50"
          r="3"
          fill="#fde047"
          filter="url(#glow)"
          className="animate-ping"
        />
      </svg>
    </div>
  )
}