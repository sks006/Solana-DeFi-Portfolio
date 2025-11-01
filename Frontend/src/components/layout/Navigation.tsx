import { LayoutDashboard, ArrowLeftRight, Shield } from "lucide-react";
import { Link, useLocation } from "react-router-dom";
import { cn } from "@/lib/utils";

const navItems = [
  { icon: LayoutDashboard, label: "Portfolio", path: "/" },
  { icon: ArrowLeftRight, label: "Swap", path: "/swap" },
  { icon: Shield, label: "Risk Alerts", path: "/risk" },
];

export const Navigation = () => {
  const location = useLocation();
  
  return (
    <nav className="border-b border-border/50 bg-card/20 backdrop-blur-sm">
      <div className="container mx-auto px-4">
        <div className="flex gap-1">
          {navItems.map((item) => {
            const Icon = item.icon;
            const isActive = location.pathname === item.path;
            
            return (
              <Link
                key={item.path}
                to={item.path}
                className={cn(
                  "flex items-center gap-2 px-4 py-3 text-sm font-medium transition-all relative",
                  isActive
                    ? "text-primary"
                    : "text-muted-foreground hover:text-foreground"
                )}
              >
                <Icon className="w-4 h-4" />
                {item.label}
                {isActive && (
                  <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-primary" />
                )}
              </Link>
            );
          })}
        </div>
      </div>
    </nav>
  );
};
