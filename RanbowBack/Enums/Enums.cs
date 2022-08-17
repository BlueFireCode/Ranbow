using System;

namespace RanbowBack.Enums
{
    [Flags]
    public enum Sights
    {
        Iron_Sights = 1,
        Sight = 2,
        Scope_1point5x = 4,
        Scope_2point0x = 8,
        Scope_2point5x = 16,
        Scope_3point0x = 32
    }

    [Flags]
    public enum Barrels
    {
        Blank = 1,
        Extended_Barrel = 2,
        Suppressor = 4,
        Muzzle_Break = 8,
        Compensator = 16,
        Flash_Hider = 32
    }

    [Flags]
    public enum Grips
    {
        Blank = 1,
        Vertical_Grip = 2,
        Angled_Grip = 4
    }

    public enum Side
    {
        Defense = 0,
        Attack = 1
    }
}
