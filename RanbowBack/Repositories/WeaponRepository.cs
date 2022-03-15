using System;
using System.Diagnostics;
using System.Collections.Generic;

using MySql.Data.MySqlClient;

using RanbowBack.Enums;
using RanbowBack.Models;
using RanbowBack.DbObjects;
using RanbowBack.Repositories.Base;
using RanbowBack.Config;

namespace RanbowBack.Repositories
{
	public class WeaponRepository : BaseRepository
	{
		public WeaponRepository(string connectionString) : base(connectionString){ }

		public static List<WeaponModel> Weapons
        {
			get => _weapons.Count < 1 ? new WeaponRepository(Configuration.Instance.Config.ConnectionString).GetAll() : _weapons;
        }

		private static List<WeaponModel> _weapons = new();
		public List<WeaponModel> GetAll()
        {
			List<DbWeapon> dbObjectList = new();

            try
            {
				DbCommand.CommandText = "SELECT * FROM Weapon;";
				DbCommand.Parameters.Clear();
				DbConnection.Open();
				using (MySqlDataReader reader = DbCommand.ExecuteReader())
                {
                    while (reader.Read())
                    {
						dbObjectList.Add(new()
						{
							ID = (int)reader["ID"],
							Name = (string)reader["Name"],
#nullable enable
							Sights = (byte)reader["Sight"],
							Barrels = (byte)reader["Barrel"],
							Grips = (byte)reader["Grip"],
#nullable disable
							Laser = (ulong)reader["Laser"] != 0
						});
                    }
                }
				DbConnection.Close();
            }
			catch (Exception e)
            {
				//TODO: Add logging
				Debug.WriteLine(e.Message);
			}
            finally
            {
				if (DbConnection.State == System.Data.ConnectionState.Open)
				{
					DbConnection.Close();
				}
			}

			if (dbObjectList.Count != 0)
			{
				foreach (var item in dbObjectList)
				{
					_weapons.Add(new()
					{
						ID = item.ID,
						Name = item.Name,
						Sights = (Sights)item.Sights,
						Barrels = (Barrels)item.Barrels,
						Grips = (Grips)item.Grips,
						Laser = item.Laser
					});
				}
				return _weapons;
			}
			_weapons = new();
			return _weapons;
        }

		public bool Read(int? id, out WeaponModel entity)
		{
			if (id is null)
			{
				entity = null;
				return true;
			}
			bool result = false;
			DbWeapon dbitem = null;

			try
			{
				DbCommand.CommandText = "SELECT * FROM Weapon WHERE ID = @id;";
				DbCommand.Parameters.Clear();
				DbCommand.Parameters.AddWithValue("id", id);
				DbConnection.Open();
				using (MySqlDataReader reader = DbCommand.ExecuteReader())
				{
					reader.Read();
					dbitem = new()
					{
						ID = (int)id,
						Name = (string)reader["Name"],
#nullable enable
						Sights = (byte)reader["Sight"],
						Barrels = (byte)reader["Barrel"],
						Grips = (byte)reader["Grip"],
#nullable disable
						Laser = (ulong)reader["Laser"] != 0
					};
				}
				DbConnection.Close();

				result = true;
			}
			catch (Exception e)
			{
				//TODO: Add logging
				Debug.WriteLine(e.Message);
			}
			finally
			{
				if (DbConnection.State == System.Data.ConnectionState.Open)
				{
					DbConnection.Close();
				}
			}
			if (result && dbitem is not null)
			{
				entity = new()
				{
					ID = dbitem.ID,
					Name = dbitem.Name,
					Sights = (Sights)dbitem.Sights,
					Barrels = (Barrels)dbitem.Barrels,
					Grips = (Grips)dbitem.Grips,
					Laser = dbitem.Laser
				};
				return result;
			}
			entity = null;
			return false;
		}
	}
}
