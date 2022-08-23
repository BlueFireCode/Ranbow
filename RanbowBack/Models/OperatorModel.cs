using RanbowBack.Enums;
using RanbowBack.Models.Base;

namespace RanbowBack.Models
{
    public class OperatorModel : BaseModel
    {
        public string ImageUrl { get; set; }
        public bool IsSelected { get; set; } = true;
        public WeaponModel Primary1 { get; set; }
        public WeaponModel Primary2 { get; set; }
        public WeaponModel Primary3 { get; set; }
        public WeaponModel Secondary1 { get; set; }
        public WeaponModel Secondary2 { get; set; }
        public WeaponModel Secondary3 { get; set; }
        public GadgetModel Gadget1 { get; set; }
        public GadgetModel Gadget2 { get; set; }
        public GadgetModel Gadget3 { get; set; }
#nullable enable
        public string? Description { get; set; }
        public Side Side { get; set; }
#nullable disable
    }
}
