using RanbowBack.Enums;
using RanbowBack.DbObjects.Base;
using System.Collections.Generic;

namespace RanbowBack.DbObjects
{
	class DbWeapon : DbObject
	{
		public string Sights { get; set; }
		public string Barrels { get; set; }
		public string Grips { get; set; }
		public bool Laser { get; set; }
	}
}
