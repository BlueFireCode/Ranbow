
using RanbowBack.Enums;
using RanbowBack.Models.Base;

namespace RanbowBack.Models
{
    public class WeaponModel : BaseModel
    {
        public Sights Sights { get; set; }
        public Barrels Barrels { get; set; }
        public Grips Grips { get; set; }
        public bool Laser { get; set; }
    }
}
