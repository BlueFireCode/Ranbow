using RanbowBack.Enums;
using RanbowBack.Models.Base;
using System.Collections.Generic;

namespace RanbowBack.Models
{
	class WeaponModel : BaseModel
	{
		public List<Sights> Sights { get; set; }
		public List<Barrels> Barrels { get; set; }
		public List<Grips> Grips { get; set; }
		public bool Laser { get; set; }
	}
}
