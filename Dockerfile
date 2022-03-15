FROM mcr.microsoft.com/dotnet/aspnet:5.0 AS base
WORKDIR /app
EXPOSE 80

FROM mcr.microsoft.com/dotnet/sdk:5.0 AS build
WORKDIR /src
COPY ["Ranbow/Ranbow.csproj", "Ranbow/"]
COPY ["RanbowBack/RanbowBack.csproj", "RanbowBack/"]
COPY ["Ranbowmizer/Ranbowmizer.csproj", "Ranbowmizer/"]
RUN dotnet restore "Ranbow/Ranbow.csproj"
COPY . .
WORKDIR "/src/Ranbow"
RUN dotnet build "Ranbow.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "Ranbow.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "Ranbow.dll"]