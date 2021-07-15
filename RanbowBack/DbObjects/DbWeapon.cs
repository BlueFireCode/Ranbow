using RanbowBack.DbObjects.Base;

namespace RanbowBack.DbObjects
{
	internal class DbWeapon : DbObject
	{
#nullable enable
		public string? Sights { get; set; } 
		public string? Barrels { get; set; }
		public string? Grips { get; set; }
#nullable disable
		public bool Laser { get; set; }
	}
}
