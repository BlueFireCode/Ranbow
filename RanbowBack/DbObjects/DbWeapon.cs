using RanbowBack.DbObjects.Base;

namespace RanbowBack.DbObjects
{
    internal class DbWeapon : DbObject
    {
#nullable enable
        public byte Sights { get; set; }
        public byte Barrels { get; set; }
        public byte Grips { get; set; }
#nullable disable
        public bool Laser { get; set; }
    }
}
